use cosmwasm_std::{
  DepsMut, 
  Env, 
  MessageInfo, 
  StdResult, 
  Response, 
  entry_point, Binary, to_binary, testing::mock_dependencies, 
};
use secret_toolkit_crypto::{sha_256, Prng, secp256k1::{PrivateKey}};
use base64::{self, Engine, engine::general_purpose};

use crate::{
  state::{
    Header, 
    Payload, 
    privkey, privkey_read, 
  }, 
  msg::{
    ExecuteMsg, 
    InstantiateMsg, ProvisionResponse
  }
};

//Instantiate the smart contract
#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
  let prng_seed: Vec<u8> = sha_256(general_purpose::STANDARD.encode(&msg.seed.clone()).as_bytes()).to_vec();
  let mut rng = Prng::new(&prng_seed, msg.seed.clone().as_bytes());

  let private_key: PrivateKey = PrivateKey::parse(&rng.rand_bytes())?;
  // let public_key: PublicKey = private_key.pubkey();

  privkey(deps.storage).save(&private_key.serialize())?;

  Ok(Response::default())
}

//Route execute messages to their appropriate destination
#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: ExecuteMsg) -> StdResult<Binary> {
  match msg {
    ExecuteMsg::Provision { } => to_binary(&try_provision(deps, env, info)?),
  }
}

//Provision a JWT to the user which expires in 1 day
fn try_provision(deps: DepsMut, env: Env, info: MessageInfo) -> StdResult<ProvisionResponse> {
  let exp = env.block.time.plus_seconds(86400);
  let header = Header { typ: String::from("JWT"), alg: String::from("HS256") }.clone().to_string();
  let payload = Payload { address: info.sender.clone().into_string(), exp }.clone().to_string();
  let encoded_header: String = general_purpose::URL_SAFE_NO_PAD.encode(header.clone());
  let encoded_payload: String = general_purpose::URL_SAFE_NO_PAD.encode(payload.clone());

  let mut msg_to_sign: Vec<u8> = vec![];
  msg_to_sign.append(&mut header.clone().as_bytes().to_vec());
  msg_to_sign.append(&mut payload.clone().as_bytes().to_vec());
  let private_key = PrivateKey::parse(&privkey_read(deps.storage).load()?)?;
  let signature = private_key.sign(&msg_to_sign, mock_dependencies().api).serialize();
  let encoded_signature: String = general_purpose::URL_SAFE_NO_PAD.encode(signature);

  let jwt = encoded_header + "." + &encoded_payload + "." + &encoded_signature;
  Ok(ProvisionResponse{ jwt })
}

