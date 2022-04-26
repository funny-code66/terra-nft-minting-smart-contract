use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Free Mint limit exceeded")]
    FreeLimitExceeded {},

    #[error("Not a minter")]
    NotMinter {},

    #[error("Not a signer")]
    NotSigner {},

    #[error("Not all signed")]
    NotAllSigned {},

    #[error("Cannot get extension")]
    CannotGetExtension {},

    #[error("Cannot execute your message, make sure if func exists")]
    CannotExecuteMsg {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },
}
