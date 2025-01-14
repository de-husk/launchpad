use cosmwasm_std::StdError;
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract has no funds")]
    NoFunds {},

    #[error("Group contract invalid address `{addr}`")]
    InvalidGroup { addr: String },

    #[error("Group contract invalid total weight `{weight}`")]
    InvalidWeight { weight: u64 },

    #[error("Invalid executor `{addr}`")]
    InvalidExecutor { addr: String },

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Reply error")]
    ReplyOnSuccess {},
}
