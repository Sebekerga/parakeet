use crate::{
    browser::{
        containers::browser::{BrowserConfigurationBuilder, BrowserContainer},
        result::{self, Result},
        ticket::RenderTicket,
    },
    page_props::PageProperties,
};

/// Main interface for rendering HTML to PDF
pub struct RenderingEngine {
    browser_container: BrowserContainer,
}

impl RenderingEngine {
    /// Create a new RenderingEngine
    /// # Arguments
    /// * `tabs` - The number of tabs to create in the browser
    /// # Returns
    /// * The new RenderingEngine
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

    /// Render the given HTML to PDF
    /// # Arguments
    /// * `html` - The HTML to render
    /// * `page_properties` - The properties of the resulting PDF page
    /// # Returns
    /// * The rendered PDF blob
    pub async fn render_html(
        &self,
        html: &str,
        page_properties: PageProperties,
    ) -> Result<Vec<u8>> {
        let mut tab = self.browser_container.lock_tab().await;

        let ticket = RenderTicket::new(html, page_properties);
        tab.render_html(&ticket).await
    }
}
