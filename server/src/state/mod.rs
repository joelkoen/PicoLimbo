pub mod handle_configuration_state;
pub mod handle_handshake_state;
pub mod handle_login_state;
pub mod handle_play_state;
pub mod handle_status_state;

use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub enum State {
    Handshake,
    Status,
    Login,
    Configuration,
    Play,
    Transfer,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            State::Handshake => f.write_str("Handshake"),
            State::Status => f.write_str("Status"),
            State::Login => f.write_str("Login"),
            State::Configuration => f.write_str("Configuration"),
            State::Play => f.write_str("Play"),
            State::Transfer => f.write_str("Transfer"),
        }
    }
}
