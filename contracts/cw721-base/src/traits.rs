use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::error::*;
use crate::msg::*;
use cosmwasm_std::{Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, Uint128};

// TODO: move this somewhere else... ideally cosmwasm-std
pub trait CustomMsg: Clone + std::fmt::Debug + PartialEq + JsonSchema {}

impl CustomMsg for Empty {}

pub trait Cw721Extended<T, C>: Cw721ExtendedExecute<T> + Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
    C: CustomMsg,
{
}

pub trait Cw721ExtendedExecute<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    fn execute_withdraw(
        &self,
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError>;

    fn execute_set_art_reveal(
        &self,
        _deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        art_reveal: bool,
    ) -> Result<Response, ContractError>;

    fn execute_free_mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: FreeMintMsg<T>,
    ) -> Result<Response, ContractError>;

    fn execute_sign(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError>;

    fn execute_add_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError>;

    fn execute_remove_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError>;

    fn execute_add_extension(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        ext: T,
    ) -> Result<Response, ContractError>;
}

pub trait Cw721ExtendedQuery<T>
where
    T: Serialize + DeserializeOwned + Clone + Default,
{
    fn query_royalties_info(
        &self,
        deps: Deps,
        token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse>;

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse>;

    fn query_is_on_reveal(&self, _deps: Deps) -> StdResult<IsOnRevealResponse>;

    fn query_get_token_uri(&self, _deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse>;

    fn query_get_extension(
        &self,
        _deps: Deps,
        token_id: String,
    ) -> StdResult<GetExtensionResponse<T>>;

    fn query_get_balance(&self, deps: Deps, owner: String) -> StdResult<GetBalanceResponse>;

    fn check_is_on_whitelist(&self, deps: Deps, member: String)
        -> StdResult<IsOnWhitelistResponse>;
}
