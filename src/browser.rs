use webdriverbidi::remote::browsing_context::GetTreeParameters;
use webdriverbidi::session::WebDriverBiDiSession;
use webdriverbidi::Capabilities;

pub struct Browser {
    webdriverbidi_session: WebDriverBiDiSession,
    browsing_context: Option<String>,
}

impl Browser {
    pub fn new(capabilities: Capabilities, host: &str, port: u16) -> Self {
        Self {
            webdriverbidi_session: WebDriverBiDiSession::new(host.to_string(), port, capabilities),
            browsing_context: None,
        }
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let _ = self.webdriverbidi_session.start().await?;
        let get_tree_params = GetTreeParameters::new(None, None);
        let get_tree_rslt = self
            .webdriverbidi_session
            .browsing_context_get_tree(get_tree_params)
            .await?;
        self.browsing_context = Some(get_tree_rslt.contexts[0].context.clone());
        Ok(())
    }

    pub async fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.webdriverbidi_session.close().await?;
        Ok(())
    }
}
