use webdriverbidi::remote::browsing_context::GetTreeParameters;
use webdriverbidi::session::WebDriverBiDiSession;

// --------------------------------------------------

use crate::errors::BrowserError;

// --------------------------------------------------

// Alias Capabilities and CapabilityRequest from webdriverbidi for easy import
pub type Capabilities = webdriverbidi::webdriver::capabilities::Capabilities;
pub type CapabilityRequest = webdriverbidi::webdriver::capabilities::CapabilityRequest;

// --------------------------------------------------

pub struct Browser {
    pub webdriverbidi_session: WebDriverBiDiSession,
    pub browsing_context: Option<String>,
}

// --------------------------------------------------

// WebDriverBiDi session management
impl Browser {
    pub fn new(capabilities: Capabilities, host: &str, port: u16) -> Self {
        Self {
            webdriverbidi_session: WebDriverBiDiSession::new(host.to_string(), port, capabilities),
            browsing_context: None,
        }
    }

    pub async fn open(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.webdriverbidi_session.start().await?;
        let get_tree_params = GetTreeParameters::new(None, None);
        let get_tree_rslt = self
            .webdriverbidi_session
            .browsing_context_get_tree(get_tree_params)
            .await?;
        self.browsing_context = Some(get_tree_rslt.contexts[0].context.clone());
        Ok(())
    }

    pub async fn close(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.webdriverbidi_session.close().await?;
        Ok(())
    }
}

// --------------------------------------------------

// Navigation
impl Browser {
    pub async fn goto(&mut self, url: &str) -> Result<(), BrowserError> {
        crate::navigation::goto(
            &mut self.webdriverbidi_session,
            self.browsing_context
                .as_ref()
                .ok_or_else(|| {
                    BrowserError::NavigationError("No browsing context available".to_owned())
                })?
                .to_string(),
            url,
        )
        .await?;
        Ok(())
    }
}
