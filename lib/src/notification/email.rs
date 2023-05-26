//! Manages access to the Google Gmail API

use google_gmail1::{api as gmail, hyper, hyper_rustls, oauth2, Gmail};

use crate::{
    error::{Error, Result},
    util,
};

#[derive(Clone)]
pub struct GmailManager {
    gmail: Gmail<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    user_id: String,
}

impl GmailManager {
    /// Connects to the Gmail API with the given secret.
    pub(super) async fn connect(secret: oauth2::ApplicationSecret, user_id: &str) -> Result<Self> {
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .build()
        .await?;

        let gmail = Gmail::new(
            hyper::Client::builder().build(
                hyper_rustls::HttpsConnectorBuilder::new()
                    .with_native_roots()
                    .https_or_http()
                    .enable_http1()
                    .enable_http2()
                    .build(),
            ),
            auth,
        );

        Ok(Self {
            gmail,
            user_id: user_id.to_string(),
        })
    }

    /// Send an e-mail with the given info.
    async fn send_to(
        &self,
        sender: &str,
        recipients: impl IntoIterator<Item = &str>,
        subject: &str,
        content: &str,
    ) -> Result<()> {
        let headers = recipients
            .into_iter()
            .map(|recipient| gmail::MessagePartHeader {
                name: Some("To".to_string()),
                value: Some(recipient.to_string()),
            })
            .chain(
                vec![
                    gmail::MessagePartHeader {
                        name: Some("Subject".to_string()),
                        value: Some(subject.to_string()),
                    },
                    gmail::MessagePartHeader {
                        name: Some("From".to_string()),
                        value: Some(sender.to_string()),
                    },
                ]
                .into_iter(),
            )
            .collect();

        let message = gmail::Message {
            payload: Some(gmail::MessagePart {
                body: Some(gmail::MessagePartBody {
                    data: Some(util::bases::base64_encode_bytes(content.as_bytes()).into_bytes()),
                    ..Default::default()
                }),
                headers: Some(headers),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut message_buffer =
            tempfile::tempfile().map_err(|_| Error::Other("failed to create tempfile"))?;
        const EMAIL_MIMETYPE: &str = "message/rfc822";

        self.gmail
            .users()
            .messages_send(message, &self.user_id)
            .upload(
                message_buffer,
                EMAIL_MIMETYPE
                    .parse()
                    .map_err(|_| Error::Other("failed to parse email mimetype"))?,
            )
            .await?;

        Ok(())
    }
}
