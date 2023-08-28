use rand::{distributions::Alphanumeric, Rng};

use crate::page_props::PageProperties;

const ID_LENGTH: usize = 10;

/// A ticket that is used to pass HTML around the library
pub struct RenderTicket {
    id: [u8; ID_LENGTH],
    html: String,
    page_properties: PageProperties,
}

impl PartialEq for RenderTicket {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl RenderTicket {
    /// Create a new RenderTicket, generating unique ID
    /// # Arguments
    /// * `html` - The HTML to render
    /// * `page_properties` - The properties of the resulting PDF page
    /// # Returns
    /// * The new RenderTicket
    pub fn new(html: &str, page_properties: PageProperties) -> RenderTicket {
        let id: Vec<u8> = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(ID_LENGTH)
            .collect();
        let id = match id.try_into() {
            Ok(id) => id,
            Err(_) => panic!("error converting id to array"),
        };
        RenderTicket {
            id,
            html: html.to_string(),
            page_properties,
        }
    }

    /// Get the ID of the ticket
    /// # Returns
    /// * The ID of the ticket
    pub fn get_id(&self) -> String {
        String::from_utf8_lossy(&self.id).to_string()
    }

    /// Get the HTML of the ticket
    /// # Returns
    /// * Requested HTML
    pub fn get_html(&self) -> &String {
        &self.html
    }

    /// Get the page properties of the ticket
    /// # Returns
    /// * Requested page properties
    pub fn get_page_properties(&self) -> &PageProperties {
        &self.page_properties
    }
}
