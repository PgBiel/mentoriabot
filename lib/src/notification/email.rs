//! Manages access to the Google Gmail API
use google_gmail1::{api as gmail, hyper, hyper_rustls, Gmail};
use tokio::sync::OnceCell;

use crate::{
    error::{Error, Result},
    model::{Session, Teacher, User},
    util::{self, BRAZIL_TIMEZONE},
};

#[derive(Clone)]
pub struct GmailManager {
    gmail: Gmail<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    user_id: String,
    sender: OnceCell<String>,
}

impl GmailManager {
    /// Connects to the Gmail API with the given authenticator.
    pub(super) async fn connect(
        auth: impl google_apis_common::GetToken + 'static,
        user_id: &str,
    ) -> Result<Self> {
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
            sender: Default::default(),
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
        let from = lettre::message::Mailbox::new(Some("mentoria".into()), sender.parse()?);
        let mut to = lettre::message::Mailboxes::new();

        for recipient in recipients {
            to.push(lettre::message::Mailbox::new(None, recipient.parse()?))
        }
        let to: lettre::message::header::To = to.into();

        let message = lettre::Message::builder()
            .subject(subject)
            .from(from)
            .mailbox(to) // workaround to specify multiple recipients
            .body(content.to_string())?;

        let message = gmail::Message {
            raw: Some(message.formatted()),
            ..Default::default()
        };

        let message_buffer =
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

    /// Send an e-mail to the teacher and to the student notifying that their session
    /// was scheduled - if possible.
    pub async fn send_emails_for_session(
        &self,
        teacher: &Teacher,
        user: &User,
        session: &Session,
    ) -> Result<()> {
        if let Some(teacher_email) = &teacher.email {
            let sender = self.resolve_sender().await?;
            let start_at = session.start_at.with_timezone(&*BRAZIL_TIMEZONE);
            let start_at_dm = util::time::day_month_display(&start_at.date_naive());
            let start_at_hm = util::time::hour_minute_display(start_at.time());

            self.send_to(
                sender,
                [&**teacher_email],
                "Monitoria Marcada",
                &format!(
                    "Sua monitoria com o aluno {} foi agendada para {} às {}!",
                    user.name, start_at_dm, start_at_hm
                ),
            )
            .await
        } else {
            // just ignore it if they can't receive e-mails
            Ok(())
        }
    }

    async fn resolve_sender(&self) -> Result<&String> {
        self.sender
            .get_or_try_init(|| {
                Box::pin(async move {
                    let response = self.gmail.users().get_profile(&self.user_id).doit().await?;
                    response
                        .1
                        .email_address
                        .ok_or_else(|| Error::Other("Could not fetch sender e-mail address."))
                })
            })
            .await
    }
}
