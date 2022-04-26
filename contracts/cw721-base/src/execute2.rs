use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::constants::*;
use crate::error::ContractError;
use crate::msg::*;
use crate::state::*;
use crate::traits::*;

const BASE_URI: &str = "ipfs://QmRiLKmhizpnwqpHGeiJnL4G6fsPAxdEdCiDkuJpt7xHPH/";

impl<'a> Cw721ExtendedContract<'a> {
    pub fn execute(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::FreeMint(msg) => self.execute_free_mint(deps, env, info, msg),
            ExecuteMsg::Withdraw {} => self.execute_withdraw(deps, env, info),
            ExecuteMsg::SetArtReveal { art_reveal } => {
                self.execute_set_art_reveal(deps, env, info, art_reveal)
            }
            ExecuteMsg::Sign {} => self.execute_sign(deps, env, info),
            ExecuteMsg::AddWhitelist { member } => {
                self.execute_add_whitelist(deps, env, info, member)
            }
            ExecuteMsg::RemoveWhitelist { member } => {
                self.execute_remove_whitelist(deps, env, info, member)
            }
            ExecuteMsg::AddExtension(msg) => {
                self.execute_add_extension(deps, env, info, msg.token_id, msg.extension)
            }
            _ => Cw721ExtendedContract::default()._execute(deps, env, info, msg),
        }
    }
}

impl<'a> Cw721ExtendedExecute<Extension> for Cw721ExtendedContract<'a> {
    fn execute_withdraw(
        &self,
        deps: DepsMut,
        env: Env,
        _info: MessageInfo,
    ) -> Result<Response, ContractError> {
        let team_signed = self
            .cw3_signature
            .may_load(deps.storage, &Addr::unchecked(ADDR_TEAM))?
            .unwrap_or(false);
        let pro_signed = self
            .cw3_signature
            .may_load(deps.storage, &Addr::unchecked(ADDR_PRO))?
            .unwrap_or(false);
        let treas_signed = self
            .cw3_signature
            .may_load(deps.storage, &Addr::unchecked(ADDR_TREAS))?
            .unwrap_or(false);

        if !team_signed || !pro_signed || !treas_signed {
            return Err(ContractError::NotAllSigned {});
        }

        let current_uluna_amount = deps
            .querier
            .query_balance(env.contract.address.to_string(), "uluna")?
            .amount;

        let team_portion = current_uluna_amount * Uint128::from(30u128) / Uint128::from(100u128);
        let pro_portion = current_uluna_amount * Uint128::from(14u128) / Uint128::from(100u128);
        let treas_portion = current_uluna_amount * Uint128::from(56u128) / Uint128::from(100u128);

        self.cw3_signature
            .save(deps.storage, &Addr::unchecked(ADDR_TEAM), &(false))?;
        self.cw3_signature
            .save(deps.storage, &Addr::unchecked(ADDR_PRO), &(false))?;
        self.cw3_signature
            .save(deps.storage, &Addr::unchecked(ADDR_TREAS), &(false))?;

        let mut messages: Vec<CosmosMsg> = vec![];
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_TEAM.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: team_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_PRO.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: pro_portion,
            }],
        }));
        messages.push(CosmosMsg::Bank(BankMsg::Send {
            to_address: ADDR_TREAS.to_string(),
            amount: vec![Coin {
                denom: String::from("uluna"),
                amount: treas_portion,
            }],
        }));
        Ok(Response::new().add_messages(messages))
    }

    fn execute_set_art_reveal(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        art_reveal: bool,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.is_on_reveal.save(deps.storage, &art_reveal)?;

        Ok(Response::new()
            .add_attribute("action", "set_art_reveal")
            .add_attribute("base_uri", &art_reveal.to_string()))
    }

    fn execute_free_mint(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: FreeMintMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let freemint_count: u64 = self
            .freemint_count
            .may_load(deps.storage)?
            .unwrap_or_default();

        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }

        if freemint_count >= 1 {
            return Err(ContractError::FreeLimitExceeded {});
        };

        let token_id: &str = &(freemint_count + 3001).to_string()[..];
        // create the token

        let extension_response: GetExtensionResponse<Extension> = deps.querier.query_wasm_smart(
            env.contract.address.clone(),
            &QueryMsg::GetExtension {
                token_id: String::from(token_id),
            },
        )?;
        // create the token
        let token = TokenInfo {
            owner: deps.api.addr_validate(&msg.owner)?,
            approvals: vec![],
            token_uri: Some(format!("{}{}.json", BASE_URI, token_id)),
            extension: extension_response.extension,
        };

        self.tokens
            .update(deps.storage, token_id, |old| match old {
                Some(pre_token) => match pre_token.owner == "not_yet_set" {
                    false => Err(ContractError::Claimed {}),
                    true => Ok(token),
                },
                None => Err(ContractError::CannotGetExtension {}),
            })?;

        let old_balance = self.wallet_balance.may_load(deps.storage, &info.sender)?;
        let new_balance = match old_balance {
            None => 1,
            Some(val) => val + 1,
        };
        self.wallet_balance
            .save(deps.storage, &info.sender, &new_balance)?;

        self.freemint_count
            .save(deps.storage, &(freemint_count + 1))?;

        Ok(Response::new()
            .add_attribute("action", "free_mint")
            .add_attribute("minter", info.sender)
            .add_attribute("owner", msg.owner)
            .add_attribute("token_id", token_id))
    }

    fn execute_sign(
        &self,
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
    ) -> Result<Response, ContractError> {
        if info.sender == ADDR_TEAM || info.sender == ADDR_PRO || info.sender == ADDR_TREAS {
            self.cw3_signature
                .save(deps.storage, &info.sender, &(true))?;
            return Ok(Response::new()
                .add_attribute("action", "sign_for_withdraw")
                .add_attribute("signer", info.sender)
                .add_attribute("time", &env.block.time.seconds().to_string()));
        } else {
            return Err(ContractError::NotSigner {});
        }
    }

    fn execute_add_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.whitelist
            .save(deps.storage, &Addr::unchecked(member.clone()), &(true))?;
        Ok(Response::new()
            .add_attribute("action", "add_to_whitelist")
            .add_attribute("member", &member))
    }

    fn execute_remove_whitelist(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        member: String,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        self.whitelist
            .save(deps.storage, &Addr::unchecked(member.clone()), &(false))?;
        Ok(Response::new()
            .add_attribute("action", "remove_from_whitelist")
            .add_attribute("member", &member))
    }

    fn execute_add_extension(
        &self,
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        token_id: String,
        ext: Extension,
    ) -> Result<Response, ContractError> {
        if info.sender != self.minter.load(deps.storage)? {
            return Err(ContractError::NotMinter {});
        }
        let token = TokenInfo {
            owner: Addr::unchecked("not_yet_set"),
            approvals: vec![],
            token_uri: Some(String::from("not_yet_set")),
            extension: ext.clone(),
        };

        self.tokens
            .update(deps.storage, &&token_id[..], |old| match old {
                Some(pre_token) => match pre_token.owner == "not_yet_set" {
                    false => Err(ContractError::Claimed {}),
                    true => Ok(token),
                },
                None => Ok(token),
            })?;
        Ok(Response::new()
            .add_attribute("action", &format!("add extension for TOKEN #{}", token_id))
            .add_attribute("extension.image", &ext.unwrap().image.unwrap()))
    }
}
