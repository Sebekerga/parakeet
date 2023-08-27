use headless_chrome::{types::PrintToPdfOptions, Tab};
use std::{env, sync::Arc, thread, time::Duration};
use tokio::time;

use super::{
    ticket::RenderTicket,
    utils::{
        result::{self, Error, Result},
        t_log,
    },
};

const WAIT_INTERVAL: u64 = 25;

pub struct TabContainer {
    tab: Arc<Tab>,
}

impl TabContainer {
    pub fn new(tab: Arc<Tab>) -> TabContainer {
        TabContainer { tab }
    }

    pub async fn render_html(&mut self, ticket: &RenderTicket) -> Result<Vec<u8>> {
        const STAGE: &str = "render_html";

        let t_id = ticket.get_id();
        let html = ticket.get_html().clone();
        let tab = self.tab.clone();
        t_log::debug!(t_id, "navigating to page");

        let handle = thread::spawn(move || {
            t_log::debug!(t_id, "saving page to temp");
            let dir = env::temp_dir();
            let path = dir.join(&format!("page_{t_id}.html"));
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
            tab.navigate_to(&page_url.to_string()).map_err(|e| {
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
                    paper_width: Some(8.3),
                    paper_height: Some(11.7),
                    margin_top: Some(0.0),
                    margin_bottom: Some(0.0),
                    margin_left: Some(0.0),
                    margin_right: Some(0.0),
                    page_ranges: Some(format!("1")),
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

        let resulting_pdf = handle.join().map_err(|e| {
            t_log::error!(t_id, "error joining thread: {e:?}");
            Error {
                stage: "render_html",
                message: format!("error joining thread: {e:?}"),
            }
        })?;

        resulting_pdf
    }
}
