use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, MutexGuard};

use derive_builder::Builder;
use headless_chrome::Browser;
use tokio::time;

use super::result::{self, Error, Result};
use super::tab::TabContainer;

#[derive(Builder)]
pub struct BrowserConfiguration {
    pub headless: bool,
    pub tabs_count: usize,
}

pub struct BrowserContainer {
    config: BrowserConfiguration,

    browser: Browser,
    tabs: Vec<Arc<Mutex<TabContainer>>>,
}

const WAIT_INTERVAL: u64 = 25;

impl BrowserContainer {
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

    pub fn restart_browser(&mut self) {
        self.browser = match launch_browser(&self.config) {
            Ok(browser) => browser,
            Err(err) => panic!("Error creating browser: {}", err),
        };
    }

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

fn launch_browser(config: &BrowserConfiguration) -> Result<Browser> {
    let launch_options = headless_chrome::LaunchOptions::default_builder()
        .headless(config.headless)
        .idle_browser_timeout(std::time::Duration::from_secs(3600))
        .port(Some(39625))
        .build()
        .map_err(|err| Error {
            stage: "launch_browser",
            message: format!("Error creating launch options: {}", err),
        })?;

    let browser = Browser::new(launch_options).map_err(|err| Error {
        stage: "launch_browser",
        message: format!("Error creating browser: {}", err),
    })?;

    Ok(browser)
}
