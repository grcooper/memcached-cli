use crate::commands::{Command, Result};
use memcached::Client;

pub struct Quit {}
pub const QUIT: &str = "QUIT";

impl Command for Quit {
    fn execute(&mut self, _client: &mut Client) -> Result {
        Ok(String::from(QUIT))
    }
}