//! Manages access to the Google Calendar API

use google_calendar3::{
    api::{Event, EventDateTime},
    hyper, hyper_rustls, CalendarHub,
};

use crate::{error::Result, model::Session};

/// Manages Google Calendar operations.
#[derive(Clone)]
pub struct CalendarManager {
    hub: CalendarHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
    calendar_id: String,
}

impl CalendarManager {
    /// Connects to the Google Calendar API, creating a new CalendarManager instance.
    pub(super) async fn connect(
        auth: impl google_apis_common::GetToken + 'static,
        calendar_id: &str,
    ) -> Result<Self> {
        let hub = CalendarHub::new(
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
            hub,
            calendar_id: calendar_id.into(),
        })
    }

    /// Creates a Google Calendar event, given a Session object.
    async fn create_event_for_session(&self, session: &Session) -> Result<Event> {
        let event = Event {
            // FIXME: use translations
            description: "Mentoria".to_string().into(),
            start: Some(EventDateTime {
                date_time: Some(session.start_at),
                ..Default::default()
            }),
            end: Some(EventDateTime {
                date_time: Some(session.end_at),
                ..Default::default()
            }),
            ..Default::default()
        };

        self.hub
            .events()
            .insert(event, &self.calendar_id)
            .doit()
            .await
            .map(|(_, event)| event)
            .map_err(From::from)
    }
}