#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;

pub mod contract;
mod error;
pub mod integration_tests;
pub mod msg;
pub mod state;
pub use crate::error::ContractError;
pub use cw721_base::Extension;
use sg721::InstantiateMsg;
use sg_std::StargazeMsgWrapper;

pub type Cw721Base<'a> = cw721_base::Cw721Contract<'a, Extension, StargazeMsgWrapper>;

pub mod entry {
    use super::*;
    use crate::{
        contract::{
            _instantiate, approve, approve_all, burn, mint, query_collection_info, ready, revoke,
            revoke_all, send_nft, transfer_nft,
        },
        msg::QueryMsg,
    };
    use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, StdResult};
    use sg721::ExecuteMsg;
    use sg_std::Response;

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Base::default();
        _instantiate(tract, deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension>,
    ) -> Result<Response, ContractError> {
        let tract = Cw721Base::default();
        match msg {
            ExecuteMsg::_Ready {} => ready(tract, deps, env, info),
            ExecuteMsg::TransferNft {
                recipient,
                token_id,
            } => transfer_nft(tract, deps, env, info, recipient, token_id),
            ExecuteMsg::SendNft {
                contract,
                token_id,
                msg,
            } => send_nft(tract, deps, env, info, contract, token_id, msg),
            ExecuteMsg::Approve {
                spender,
                token_id,
                expires,
            } => approve(tract, deps, env, info, spender, token_id, expires),
            ExecuteMsg::Revoke { spender, token_id } => {
                revoke(tract, deps, env, info, spender, token_id)
            }
            ExecuteMsg::ApproveAll { operator, expires } => {
                approve_all(tract, deps, env, info, operator, expires)
            }
            ExecuteMsg::RevokeAll { operator } => revoke_all(tract, deps, env, info, operator),
            ExecuteMsg::Burn { token_id } => burn(tract, deps, env, info, token_id),
            ExecuteMsg::Mint(msg) => mint(tract, deps, env, info, msg),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::CollectionInfo {} => to_binary(&query_collection_info(deps)?),
            _ => Cw721Base::default().query(deps, env, msg.into()),
        }
    }
}
