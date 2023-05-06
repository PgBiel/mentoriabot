//! Manages access to the Google Calendar API

use google_calendar3::{api::Calendar, hyper, hyper_rustls, oauth2, CalendarHub};

/// Manages Google Calendar operations.
pub struct CalendarManager {
    hub: CalendarHub<hyper_rustls::HttpsConnector<hyper::client::HttpConnector>>,
}

impl CalendarManager {
    /// Connects to the Google Calendar API, creating a new CalendarManager instance.
    async fn connect(secret: oauth2::ApplicationSecret) -> Self {
        let auth = oauth2::InstalledFlowAuthenticator::builder(
            secret,
            oauth2::InstalledFlowReturnMethod::HTTPRedirect,
        )
        .build()
        .await
        .unwrap();
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

        Self { hub }
    }
}
