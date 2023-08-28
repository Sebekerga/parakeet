use headless_chrome::{types::PrintToPdfOptions, Tab};
use std::{env, sync::Arc, thread, time::Duration};
use tokio::time;

use crate::browser::{
    result::{self, Result},
    t_log,
    ticket::RenderTicket,
};

const WAIT_INTERVAL: u64 = 25;

/// A container for a tab, used to render HTML to PDF
pub struct TabContainer {
    tab: Arc<Tab>,
}

impl TabContainer {
    /// Create a new TabContainer
    /// # Arguments
    /// * `tab` - Tab to use
    pub fn new(tab: Arc<Tab>) -> TabContainer {
        TabContainer { tab }
    }

    /// Render the given HTML to PDF
    /// # Arguments
    /// * `ticket` - Ticket containing the HTML to render
    /// # Returns
    /// * Rendered PDF blob
    /// # Errors
    /// * If there is an unexpected error with the browser
    /// * If there is an error saving or retrieving the page to temp
    pub async fn render_html(&mut self, ticket: &RenderTicket) -> Result<Vec<u8>> {
        const STAGE: &str = "render_html";

        t_log::info!(ticket.get_id(), "rendering HTML");

        let t_id = ticket.get_id();
        let html = ticket.get_html().clone();
        let tab = self.tab.clone();
        let page_properties = ticket.get_page_properties().clone();
        t_log::debug!(t_id, "navigating to page");

        let handle = thread::spawn(move || {
            t_log::debug!(t_id, "saving page to temp");
            let dir = env::temp_dir();
            let path = dir.join(format!("page_{t_id}.html"));
            std::fs::write(&path, html).map_err(|e| {
                let err_msg = format!("error saving page to temp: {e:?}");
                t_log::error!(t_id, "{err_msg}");
                result::error!(STAGE, "{err_msg}")
            })?;
            t_log::debug!(t_id, "page saved to {path:?}");

            let page_url = url::Url::from_file_path(&path).map_err(|e| {
                let err_msg = format!("error creating url from path: {e:?}");
                t_log::error!(t_id, "{err_msg}");
                result::error!(STAGE, "{err_msg}")
            })?;

            t_log::debug!(t_id, "navigating to page");
            tab.navigate_to(page_url.as_ref()).map_err(|e| {
                let err_msg = format!("error navigating to page: {e:?}");
                t_log::error!(t_id, "{err_msg}");
                result::error!(STAGE, "{err_msg}")
            })?;
            if let Err(e) = tab.wait_until_navigated() {
                let err_msg = format!("error waiting until navigated: {e:?}");
                t_log::warng!(t_id, "{err_msg}")
            };

            t_log::debug!(t_id, "printing to PDF");
            let pdf_data = tab
                .print_to_pdf(Some(PrintToPdfOptions {
                    landscape: Some(false),
                    display_header_footer: Some(false),
                    print_background: Some(true),
                    scale: Some(1.0),
                    paper_width: Some(page_properties.paper_width),
                    paper_height: Some(page_properties.paper_height),
                    margin_top: Some(page_properties.margin_top),
                    margin_bottom: Some(page_properties.margin_bottom),
                    margin_left: Some(page_properties.margin_left),
                    margin_right: Some(page_properties.margin_right),
                    ..Default::default()
                }))
                .map_err(|e| {
                    let err_msg = format!("error printing to PDF: {e:?}");
                    t_log::error!(t_id, "{err_msg}");
                    result::error!(STAGE, "{err_msg}")
                })?;

            t_log::debug!(t_id, "PDF printing ok");
            Ok(pdf_data)
        });

        let mut interval = time::interval(Duration::from_millis(WAIT_INTERVAL));
        while !handle.is_finished() {
            interval.tick().await;
        }

        let t_id = ticket.get_id();

        handle.join().map_err(|e| {
            let err_msg = format!("error joining thread: {e:?}");
            t_log::error!(t_id, "{err_msg}");
            result::error!(STAGE, "{err_msg}")
        })?
    }
}
