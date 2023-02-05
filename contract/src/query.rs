use cosmwasm_std::{ entry_point, Deps, Env, StdResult, Binary };

use crate::{
  msg::{
    QueryMsg,
  }, 
};

//Route the user's query to the appropriate function
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
  
  }
}
