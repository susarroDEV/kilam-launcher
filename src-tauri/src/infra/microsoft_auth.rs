use crate::business::auth::{AuthProvider, UserProfile};
use crate::error::Result;
use async_trait::async_trait;
use reqwest::Client;
use crate::business::auth::AuthType;
use tauri_plugin_store::StoreExt;

const CLIENT_ID: &str = "a3b355d3-569d-4db8-9323-86fdb75e8e41";
const KEYRING_SERVICE: &str = "kilam-launcher";
const KEYRING_USER: &str = "microsoft-token";

pub struct MicrosoftAuthProvider {
  app_handle: tauri::AppHandle,
  client: Client,
}

impl MicrosoftAuthProvider {
  pub fn new(app_handle: tauri::AppHandle, client: Client) -> Self {
    Self { app_handle, client }
  }

  async fn start_oauth(&self) -> Result<(String, String)> { 
    let (tx, rx) = tokio::sync::oneshot::channel::<String>();
    let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

    let port = tauri_plugin_oauth::start(move |url| {
      let url = tauri::Url::parse(&url).ok();
      let code = url
        .as_ref()
        .and_then(|u| u.query_pairs().find(|(k, _)| k == "code"))
        .map(|(_, v)| v.to_string());

      if let Some(code) = code {
        if let Some(tx) = tx.lock().unwrap().take() {
          let _ = tx.send(code);
        }
      }
    })?;

    let auth_url = format!(
      "https://login.microsoftonline.com/consumers/oauth2/v2.0/authorize\
        ?client_id={CLIENT_ID}\
        &response_type=code\
        &redirect_uri=http://localhost:{port}\
        &scope=XboxLive.signin%20offline_access\
        &prompt=select_account"
    );

    let redirect_uri = format!("http://localhost:{}", port);

    tauri_plugin_opener::open_url(auth_url, None::<&str>)?;

    let code = rx.await.map_err(|_| {
      crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession)
    })?;

    Ok((code, redirect_uri))
  }

  async fn exchange_code(&self, code: &str, redirect_uri: &str) -> Result<String> {
    let params = [
      ("client_id", CLIENT_ID),
      ("code", code),
      ("grant_type", "authorization_code"),
      ("redirect_uri", redirect_uri),
      ("scope", "XboxLive.signin offline_access"),
    ];

    let res: serde_json::Value = self.client
      .post("https://login.microsoftonline.com/consumers/oauth2/v2.0/token")
      .form(&params)
      .send()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .json()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    res["access_token"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::business::auth::AuthError::MissingSession.into())
  }

  async fn authenticate_xbox(&self, ms_token: &str) -> Result<(String, String)> {
    let body = serde_json::json!({
      "Properties": {
        "AuthMethod": "RPS",
        "SiteName": "user.auth.xboxlive.com",
        "RpsTicket": format!("d={}", ms_token)
      },
      "RelyingParty": "http://auth.xboxlive.com",
      "TokenType": "JWT"
    });

    let res: serde_json::Value = self.client
      .post("https://user.auth.xboxlive.com/user/authenticate")
      .json(&body)
      .send()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .json()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    let token = res["Token"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))?;

    let userhash = res["DisplayClaims"]["xui"][0]["uhs"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))?;

    Ok((token, userhash))
  }

  async fn authenticate_xsts(&self, xbl_token: &str) -> Result<String> {
    let body = serde_json::json!({
      "Properties": {
        "SandboxId": "RETAIL",
        "UserTokens": [xbl_token]
      },
      "RelyingParty": "rp://api.minecraftservices.com/",
      "TokenType": "JWT"
    });

    let res: serde_json::Value = self.client
      .post("https://xsts.auth.xboxlive.com/xsts/authorize")
      .json(&body)
      .send()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .json()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    res["Token"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))
  }

  async fn authenticate_minecraft(&self, xsts_token: &str, userhash: &str) -> Result<String> {
    let body = serde_json::json!({
      "identityToken": format!("XBL3.0 x={};{}", userhash, xsts_token)
    });

    let res: serde_json::Value = self.client
      .post("https://api.minecraftservices.com/authentication/login_with_xbox")
      .json(&body)
      .send()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .json()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    tracing::error!("MC auth response: {}", res);

    res["access_token"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))
  }

  async fn fetch_profile(&self, mc_token: &str) -> Result<(String, String)> {
    let res: serde_json::Value = self.client
      .get("https://api.minecraftservices.com/minecraft/profile")
      .bearer_auth(mc_token)
      .send()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .json()
      .await
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    let username = res["name"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))?;

    let uuid = res["id"]
      .as_str()
      .map(|s| s.to_string())
      .ok_or_else(|| crate::error::LauncherError::Auth(crate::business::auth::AuthError::MissingSession))?;

    Ok((username, uuid))
  }
}

#[async_trait]
impl AuthProvider for MicrosoftAuthProvider {
  async fn login(&self, _username: &str) -> Result<UserProfile> {

    let (code, redirect_uri) = self.start_oauth().await?;
    tracing::error!("OAuth code OK");
    
    let ms_token = self.exchange_code(&code, &redirect_uri).await?;
    tracing::error!("MS token OK");
    
    let (xbl_token, userhash) = self.authenticate_xbox(&ms_token).await?;
    tracing::error!("XBL OK: userhash={}", userhash);
    
    let xsts_token = self.authenticate_xsts(&xbl_token).await?;
    tracing::error!("XSTS OK");
    
    let mc_token = self.authenticate_minecraft(&xsts_token, &userhash).await?;
    tracing::error!("MC token OK");
    
    let (username, uuid) = self.fetch_profile(&mc_token).await?;
    tracing::error!("Profile OK: {}", username);

    keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .set_password(&mc_token)
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?;

    let profile = UserProfile {
      username,
      uuid,
      auth_type: AuthType::Microsoft,
      token: Some(mc_token),
    };

    let store = self.app_handle.store_builder("session.json").build()?;
    store.set("profile", serde_json::to_value(&profile)?);

    Ok(profile)
  }

  async fn logout(&self) -> Result<()> {
    let store = self.app_handle.store_builder("session.json").build()?;
    if store.get("profile").is_some() {
      store.delete("profile");
    }

    let _ = keyring::Entry::new(KEYRING_SERVICE, KEYRING_USER)
      .map_err(|e| crate::business::auth::AuthError::InvalidUsername(e.to_string()))?
      .delete_credential();

    Ok(())
  }

  async fn current_session(&self) -> Result<Option<UserProfile>> {
    let store = self.app_handle.store_builder("session.json").build()?;
    match store.get("profile") {
      Some(p) => Ok(Some(serde_json::from_value(p)?)),
      None => Ok(None),
    }
  }
}
