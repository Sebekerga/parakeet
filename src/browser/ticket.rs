use rand::{distributions::Alphanumeric, Rng};

const ID_LENGTH: usize = 10;

pub struct RenderTicket {
    id: [u8; ID_LENGTH],
    html: String,
}

impl PartialEq for RenderTicket {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self, other)
    }
}

impl RenderTicket {
    pub fn new(html: &str) -> RenderTicket {
        let id: Vec<u8> = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(ID_LENGTH)
            .collect();
        let id = match id.try_into() {
            Ok(id) => id,
            Err(_) => panic!("Error converting id to array"),
        };
        RenderTicket {
            id,
            html: html.to_string(),
        }
    }

    pub fn get_id(&self) -> String {
        String::from_utf8_lossy(&self.id).to_string()
    }

    pub fn get_html(&self) -> &String {
        &self.html
    }
}
