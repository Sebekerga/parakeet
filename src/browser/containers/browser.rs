use derive_builder::Builder;
use headless_chrome::Browser;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard};
use tokio::time;

use super::tab::TabContainer;
use crate::browser::result::{self, Result};

/// Configuration for the browser
#[derive(Builder)]
pub struct BrowserConfiguration {
    pub headless: bool,
    pub tabs_count: usize,
}

/// A container for a browser, holding browser and tab containers
pub struct BrowserContainer {
    config: BrowserConfiguration,

    browser: Browser,
    tabs: Vec<Arc<Mutex<TabContainer>>>,
}

const WAIT_INTERVAL: u64 = 25;

impl BrowserContainer {
    /// Create a new BrowserContainer
    /// # Arguments
    /// * `config` - Configuration for the browser
    /// # Returns
    /// * The new BrowserContainer
    pub fn new(config: BrowserConfiguration) -> Result<BrowserContainer> {
        let browser = launch_browser(&config)?;
        let mut new_container = BrowserContainer {
            browser,
            config,
            tabs: vec![],
        };
        new_container.init_tabs()?;

        log::info!("Browser container initialized");
        Ok(new_container)
    }

    /// Restarts the browser, held by the container
    pub fn restart_browser(&mut self) {
        self.browser = match launch_browser(&self.config) {
            Ok(browser) => browser,
            Err(err) => panic!("error creating browser: {}", err),
        };
    }

    /// Locks a tab for use
    pub async fn lock_tab(&self) -> MutexGuard<'_, TabContainer> {
        loop {
            for tab in self.tabs.iter() {
                let Ok(locked_tab) = tab.try_lock() else {
                    continue;
                };
                return locked_tab;
            }

            let mut interval = time::interval(Duration::from_millis(WAIT_INTERVAL));
            interval.tick().await;
        }
    }

    /// Initializes the tabs in the browser, creating the correct number of tabs and tab containers
    /// # Errors
    /// * If there is an unexpected error with the browser
    pub fn init_tabs(&mut self) -> Result<()> {
        const STAGE: &str = "init_tabs";

        // opening tabs
        let tabs_count = || {
            let tabs_binding = self.browser.get_tabs().clone();
            let tabs = tabs_binding
                .lock()
                .map_err(|err| result::error!(STAGE, "{err:?}"))?;

            let tab_count = tabs.len();
            Ok(tab_count)
        };

        log::debug!("opening tabs");
        while tabs_count()? < self.config.tabs_count {
            self.browser
                .new_tab()
                .map_err(|err| result::error!(STAGE, "error when creating tabs: {err:?}"))?;
        }

        // creating tab containers
        log::debug!("initializing tab containers");
        let tabs_binding = self.browser.get_tabs().clone();
        let tabs = tabs_binding
            .lock()
            .map_err(|err| result::error!(STAGE, "error when creating tab containers: {err:?}"))?;
        for tab in tabs.iter() {
            self.tabs
                .push(Arc::new(Mutex::new(TabContainer::new(tab.clone()))));
        }

        Ok(())
    }
}

/// Launches a browser with the given configuration
/// # Arguments
/// * `config` - Configuration for the browser
/// # Returns
/// * The new browser
fn launch_browser(config: &BrowserConfiguration) -> Result<Browser> {
    const STAGE: &str = "launch_browser";
    let launch_options = headless_chrome::LaunchOptions::default_builder()
        .headless(config.headless)
        .idle_browser_timeout(std::time::Duration::from_secs(3600))
        .port(Some(39625))
        .build()
        .map_err(|err| result::error!(STAGE, "error creating launch options: {err}"))?;

    let browser = Browser::new(launch_options)
        .map_err(|err| result::error!(STAGE, "error creating browser: {err}"))?;

    Ok(browser)
}
