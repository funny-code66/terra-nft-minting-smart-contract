mod constants;
mod contract_tests;
mod error;
mod execute;
mod execute2;
pub mod msg;
mod query;
mod query2;
pub mod state;
mod traits;

pub use crate::constants::*;
pub use crate::error::ContractError;
pub use crate::msg::*;
pub use crate::state::*;

// This is a simple type to let us handle empty extensions

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;

    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // // This makes a conscious choice on the various generics used by the contract
    // #[entry_point]
    // pub fn migrate(
    //     _deps: DepsMut,
    //     _env: Env,
    //     _info: MessageInfo,
    //     _msg: MigrateMsg,
    // ) -> StdResult<Response> {
    //     Ok(Response::default())
    // }

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let tract = Cw721ExtendedContract::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let tract = Cw721ExtendedContract::default();
        tract.execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        let tract = Cw721ExtendedContract::default();
        tract.query(deps, env, msg)
    }
}
