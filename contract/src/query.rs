use std::convert::TryInto;

use cosmwasm_std::{ entry_point, Deps, StdResult, StdError };
use secret_toolkit_crypto::secp256k1::{PrivateKey, Signature};

use crate::{
  msg::{
    ValidateResponse,
  }, state::privkey_read, 
};

//Route the user's query to the appropriate function
#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
  match msg {
    QueryMsg::ValidateJWT { jwt } => to_binary(&)
  }
}

//Validate the user's JWT
fn validate_jwt(deps: Deps, jwt: String) -> StdResult<ValidateResponse> {
  let jwt_sections: Vec<&str> = jwt.split(".").collect::<Vec<&str>>();
  let payload: &str = jwt_sections[1];
  let signature: [u8; 64] = match jwt_sections[2].as_bytes().try_into() {
    Ok(signature) => signature,
    Err(_) => { return Err(StdError::GenericErr{ msg: String::from("Invalid signature format") }) }
  };
  let signature = Signature::parse(&signature)?;
  

  let private_key: PrivateKey = PrivateKey::parse(&privkey_read(deps.storage).load()?)?;
  private_key.pubkey().verify(jwt.as_bytes(), signature, api)
}
