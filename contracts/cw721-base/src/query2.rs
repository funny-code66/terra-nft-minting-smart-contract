use crate::constants::*;
use crate::msg::*;
use crate::state::*;
use crate::traits::*;
use cosmwasm_std::{to_binary, Addr, Binary, Decimal, Deps, Env, StdResult, Uint128};

impl<'a> Cw721ExtendedContract<'a> {
    pub fn query(&self, deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::RoyaltyInfo {
                token_id,
                sale_price,
            } => to_binary(&self.query_royalties_info(deps, token_id, sale_price)?),
            QueryMsg::CheckRoyalties {} => to_binary(&self.check_royalties(deps)?),
            QueryMsg::IsOnReveal {} => to_binary(&self.query_is_on_reveal(deps)?),
            QueryMsg::GetTokenUri { token_id } => {
                to_binary(&self.query_get_token_uri(deps, token_id)?)
            }
            QueryMsg::GetBalance { owner } => to_binary(&self.query_get_balance(deps, owner)?),
            QueryMsg::IsOnWhitelist { member } => {
                to_binary(&self.check_is_on_whitelist(deps, member)?)
            }
            QueryMsg::GetExtension { token_id } => {
                to_binary(&self.query_get_extension(deps, token_id)?)
            }
            _ => Cw721ExtendedContract::default()._query(deps, env, msg),
        }
    }
}

impl<'a> Cw721ExtendedQuery<Extension> for Cw721ExtendedContract<'a> {
    fn query_royalties_info(
        &self,
        deps: Deps,
        token_id: String,
        sale_price: Uint128,
    ) -> StdResult<RoyaltiesInfoResponse> {
        let percentage = Decimal::percent(ROYALTY_PERCENTAGE);
        let royalty_from_sale_price = sale_price * percentage;

        Ok(RoyaltiesInfoResponse {
            address: ROYALTY_ADDRESS.to_string(),
            royalty_amount: royalty_from_sale_price,
        })
    }

    fn check_royalties(&self, _deps: Deps) -> StdResult<CheckRoyaltiesResponse> {
        Ok(CheckRoyaltiesResponse {
            royalty_payments: true,
        })
    }

    fn query_is_on_reveal(&self, deps: Deps) -> StdResult<IsOnRevealResponse> {
        let res: bool = self
            .is_on_reveal
            .may_load(deps.storage)?
            .unwrap_or_default();

        Ok(IsOnRevealResponse { is_on_reveal: res })
    }

    fn query_get_token_uri(&self, deps: Deps, token_id: String) -> StdResult<GetTokenUriResponse> {
        let is_on_reveal = self
            .is_on_reveal
            .may_load(deps.storage)?
            .unwrap_or_default();

        let res = match is_on_reveal {
            true => format!("{}{}.json", BASE_URI, token_id),
            false => String::from("NOT_YET_REVEALED"),
        };

        Ok(GetTokenUriResponse { token_uri: res })
    }

    fn query_get_balance(&self, deps: Deps, owner: String) -> StdResult<GetBalanceResponse> {
        let res = self
            .wallet_balance
            .may_load(deps.storage, &Addr::unchecked(owner))?
            .unwrap_or(0);
        Ok(GetBalanceResponse { balance: res })
    }

    fn check_is_on_whitelist(
        &self,
        deps: Deps,
        member: String,
    ) -> StdResult<IsOnWhitelistResponse> {
        let res = self
            .whitelist
            .may_load(deps.storage, &Addr::unchecked(member))?
            .unwrap_or(false);
        Ok(IsOnWhitelistResponse {
            is_on_whitelist: res,
        })
    }

    fn query_get_extension(
        &self,
        deps: Deps,
        token_id: String,
    ) -> StdResult<GetExtensionResponse<Extension>> {
        let info = self.tokens.load(deps.storage, &token_id)?;
        Ok(GetExtensionResponse {
            extension: info.extension,
        })
    }
}
