use google_apis_common::oauth2::{
    self as yup_oauth2,
    authenticator_delegate::{DefaultInstalledFlowDelegate, InstalledFlowDelegate},
};

use crate::lib::error::{Error, Result};

const AUTH_VAR: &str = "MRB_AUTH";
const AUTH_CLEAN_VAR: &str = "MRB_AUTH_CLEAN";
const APPLICATION_SECRET_PATH: &str = "secrets/client-secret.json";
const OAUTH_TOKEN_CACHE_PATH: &str = "secrets/oauth-token-cache.json";

/// Authenticates the user by providing an authentication link (if MRB_AUTH=1 is enabled).
/// The user must confirm the authentication on the browser, and
/// paste the token, which is then cached to the disk.
#[derive(Debug, Clone)]
pub struct Authenticator {
    token: String,
}

impl Authenticator {
    pub async fn authenticate() -> Result<Self> {
        if std::env::var(AUTH_CLEAN_VAR).map_or(false, |s| s == "1") {
            let Ok(_) = std::fs::remove_file(OAUTH_TOKEN_CACHE_PATH)
                else {
                    panic!("Could not remove the cache file at {OAUTH_TOKEN_CACHE_PATH}.");
                };
        }
        let secret = yup_oauth2::read_application_secret(APPLICATION_SECRET_PATH)
            .await
            .map_err(|e| Error::String(format!("Failed to get the client secret; ensure there is a {APPLICATION_SECRET_PATH} file: {e}")))?;

        let authenticator = yup_oauth2::InstalledFlowAuthenticator::builder(
            secret,
            yup_oauth2::InstalledFlowReturnMethod::Interactive,
        )
        .persist_tokens_to_disk(OAUTH_TOKEN_CACHE_PATH)
        .flow_delegate(Box::new(AuthDelegate))
        .build()
        .await?;

        // Google API scopes necessary
        let scopes = &[
            "https://www.googleapis.com/auth/calendar.app.created",
            "https://www.googleapis.com/auth/calendar.calendarlist.readonly",
            "https://www.googleapis.com/auth/calendar.calendars.readonly",
            "https://www.googleapis.com/auth/calendar.events",
            "https://www.googleapis.com/auth/gmail.send",
            "https://www.googleapis.com/auth/gmail.metadata",
        ];

        let token = authenticator.token(scopes).await?;
        Ok(Self {
            token: token
                .token()
                .ok_or_else(|| Error::Other("Failed to get the auth token."))?
                .to_string(),
        })
    }
}

type GetTokenOutput<'a> = std::pin::Pin<
    Box<
        dyn std::future::Future<
                Output = std::result::Result<
                    Option<String>,
                    Box<dyn std::error::Error + Send + Sync>,
                >,
            > + Send
            + 'a,
    >,
>;

impl google_apis_common::GetToken for Authenticator {
    fn get_token<'a>(&'a self, _scopes: &'a [&str]) -> GetTokenOutput<'a> {
        Box::pin(async move { Ok(Some(self.token.clone())) })
    }
}

/// Overrides the default auth interaction flow by only prompting
/// interactively for a token if MRB_AUTH=1 is enabled.
struct AuthDelegate;

impl InstalledFlowDelegate for AuthDelegate {
    fn redirect_uri(&self) -> Option<&str> {
        DefaultInstalledFlowDelegate.redirect_uri()
    }

    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        need_code: bool,
    ) -> std::pin::Pin<
        Box<dyn std::future::Future<Output = std::result::Result<String, String>> + Send + 'a>,
    > {
        // only proceed interactively if MRB_AUTH=1
        if std::env::var(AUTH_VAR).map_or(false, |s| s == "1") {
            DefaultInstalledFlowDelegate.present_user_url(url, need_code)
        } else {
            panic!(
                "No OAuth2 token cache found.
                    Please specify the {AUTH_VAR}=1 environment variable to specify it interactively,
                    or add it to the file {OAUTH_TOKEN_CACHE_PATH}."
            );
        }
    }
}
