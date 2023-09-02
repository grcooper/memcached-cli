use memcached::Client;

pub mod set;
pub mod get;
pub mod quit;
pub mod noop;

use crate::commands::get::GetBuilder;
use crate::commands::set::SetBuilder;
use crate::commands::quit::Quit;
use crate::commands::noop::NoOp;

pub type Result = std::result::Result<String, CommandError>;

pub struct CommandError {
    message: String
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

pub trait Command {
    fn execute(&mut self, client: &mut Client) -> Result;
}

pub fn generate_command(parts: Vec<&str>) -> Box<dyn Command> {
    match parts[0] {
        "get" => {
            assert!(parts.len() == 2);
            let builder = GetBuilder::new();
            Box::new(builder.key(parts[1].to_string()).build())
        },
        "set" => {
            assert!(parts.len() > 2);
            let mut builder = SetBuilder::new();
            builder = builder.key(parts[1].to_string()).value(parts[2].to_string());
            if parts.len() > 3 {
                builder = builder.expiration(parts[3].parse().unwrap());
            }
            if parts.len() > 4 {
                builder = builder.flags(parts[4].parse().unwrap());
            }
            Box::new(builder.build())
        },
        "quit" => {
            Box::new(Quit {})
        }
        _ => {
            Box::new(NoOp {})
        }
    }
}