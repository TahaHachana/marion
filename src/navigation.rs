use webdriverbidi::remote::browsing_context::{NavigateParameters, ReadinessState};
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::errors::BrowserError;

pub async fn goto(
    session: &mut WebDriverBiDiSession,
    browsing_context: String,
    url: &str,
) -> Result<(), BrowserError> {
    let navigate_params = NavigateParameters::new(
        browsing_context.clone(),
        url.into(),
        Some(ReadinessState::Complete),
    );
    session
        .browsing_context_navigate(navigate_params)
        .await
        .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
    Ok(())
}

// pub struct Navigator<'a> {
//     browser: &'a mut Browser,
// }

// impl<'a> Navigator<'a> {
//     pub fn new(browser: &'a mut Browser) -> Self {
//         Navigator { browser }
//     }

//     pub async fn goto(&mut self, url: &str) -> Result<(), BrowserError> {
//         // Navigate to the specified URL
//         if let Some(ref browsing_context) = self.browser.browsing_context {
//             let navigate_params = NavigateParameters::new(
//                 browsing_context.clone(),
//                 url.to_string(),
//                 Some(ReadinessState::Complete),
//             );
//             self.browser.webdriverbidi_session
//                 .browsing_context_navigate(navigate_params)
//                 .await
//                 .map_err(|e| BrowserError::NavigationError(e.to_string()))?;
//             Ok(())
//         } else {
//             Err(BrowserError::NavigationError("No browsing context available".into()))
//         }
//     }

//     pub fn back(&self) {
//         // Navigate back
//     }

//     pub fn forward(&self) {
//         // Navigate forward
//     }

//     pub fn refresh(&self) {
//         // Refresh the page
//     }
// }
