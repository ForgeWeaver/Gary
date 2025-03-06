pub mod cli;
pub mod position_manager;
pub mod utils;
pub mod wallet;

pub type BoxedStdError = Box<dyn std::error::Error>;
pub type StdResult = Result<(), BoxedStdError>;
