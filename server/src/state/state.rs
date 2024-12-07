use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum State {
    Handshake,
    Status,
    Login,
    Configuration,
    Transfer,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::Handshake => f.write_str("Handshake"),
            State::Status => f.write_str("Status"),
            State::Login => f.write_str("Login"),
            State::Configuration => f.write_str("Configuration"),
            State::Transfer => f.write_str("Transfer"),
        }
    }
}
