//! Manages access to the Google Calendar API

use google_calendar3::{
    api::{
        ConferenceData, ConferenceSolutionKey, CreateConferenceRequest, Event, EventAttendee,
        EventDateTime,
    },
    hyper, hyper_rustls, CalendarHub,
};

use crate::{
    error::Result,
    model::{Session, Teacher},
};

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
    pub async fn create_event_for_session(
        &self,
        teacher: &Teacher,
        session: &Session,
    ) -> Result<Event> {
        let event = Event {
            // FIXME: use translations
            summary: "Mentoria".to_string().into(),
            description: "Mentoria".to_string().into(),
            start: Some(EventDateTime {
                date_time: Some(session.start_at),
                ..Default::default()
            }),
            end: Some(EventDateTime {
                date_time: Some(session.end_at),
                ..Default::default()
            }),
            // create Google Meet conference
            conference_data: Some(ConferenceData {
                create_request: Some(CreateConferenceRequest {
                    conference_solution_key: Some(ConferenceSolutionKey {
                        type_: Some("hangoutsMeet".to_string()),
                    }),
                    // a random and unique request ID is necessary for some reason
                    request_id: Some(crate::util::time::brazil_now().timestamp_millis().to_string()),
                    status: None,
                }),
                ..Default::default()
            }),
            attendees: Some(vec![EventAttendee {
                email: Some(teacher.email.clone()),
                response_status: Some("needsAction".to_string()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        self.hub
            .events()
            .insert(event, &self.calendar_id)
            .conference_data_version(1)  // enables creating conferences
            .doit()
            .await
            .map(|(_, event)| event)
            .map_err(From::from)
    }
}
