use browser::{
    containers::browser::{BrowserConfigurationBuilder, BrowserContainer},
    result::Result,
};

use crate::browser::ticket::RenderTicket;

pub mod browser;

pub struct RenderingEngine {
    browser_container: BrowserContainer,
}

impl RenderingEngine {
    pub fn new(tabs: usize) -> Self {
        let config = BrowserConfigurationBuilder::default()
            .headless(false)
            .tabs_count(tabs)
            .build()
            .unwrap();
        let container = BrowserContainer::new(config);
        RenderingEngine {
            browser_container: container.unwrap(),
        }
    }

    pub async fn render_html(&self, html: &str) -> Result<Vec<u8>> {
        let mut tab = self.browser_container.lock_tab().await;

        let ticket = RenderTicket::new(html);
        tab.render_html(&ticket).await
    }
}
