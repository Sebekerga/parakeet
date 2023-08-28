use browser::{
    containers::browser::{BrowserConfigurationBuilder, BrowserContainer},
    result::Result,
};

use crate::browser::{result, ticket::RenderTicket};

pub mod browser;

pub struct RenderingEngine {
    browser_container: BrowserContainer,
}

impl RenderingEngine {
    pub fn new(tabs: usize) -> Result<Self> {
        const STAGE: &str = "new_render_engine";
        let config = BrowserConfigurationBuilder::default()
            .headless(true)
            .tabs_count(tabs)
            .build()
            .map_err(|err| {
                let err_msg = format!("error when building config: {}", err);
                log::error!("{}: {}", STAGE, err_msg);
                result::error!(STAGE, "{err_msg}")
            })?;

        let container = BrowserContainer::new(config)?;
        Ok(RenderingEngine {
            browser_container: container,
        })
    }

    pub async fn render_html(&self, html: &str) -> Result<Vec<u8>> {
        let mut tab = self.browser_container.lock_tab().await;

        let ticket = RenderTicket::new(html);
        tab.render_html(&ticket).await
    }
}
