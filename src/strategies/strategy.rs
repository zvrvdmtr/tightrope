use crate::domain::server::Server;

pub trait Strategy {
    fn get_next_server(&self) -> Option<&Server>;
}
