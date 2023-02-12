#[cfg(test)]
mod tests {
  use crate::execute::{instantiate, execute};
  use crate::msg::{
    InstantiateMsg, ExecuteMsg, ProvisionResponse
  };

  use cosmwasm_std::{
    testing::*, 
    Response, 
    MessageInfo, 
    DepsMut, Binary, from_binary
  };
  use cosmwasm_std::{ Coin, Uint128 };

  fn execute_instantiate(deps: DepsMut, info: MessageInfo) {
    let seed: String = String::from("some random junk");
    let init_msg: InstantiateMsg = InstantiateMsg { seed };

    // we can just call .unwrap() to assert this was a success
    let _res: Response = instantiate(deps, mock_env(), info, init_msg).unwrap();
  }


  #[test]
  fn proper_initialization() {
    //Set up dependencies and single user's wallet
    let mut deps = mock_dependencies();
    let info = mock_info(
      "creator",
      &[Coin {
        denom: "earth".to_string(),
        amount: Uint128::new(1000),
      }],
    );
    //Instantiate the contract
    execute_instantiate(deps.as_mut(), info.clone());
  }
  #[test]
  fn provision() {
    //Set up dependencies and single user's wallet
    let mut deps = mock_dependencies();
    let info = mock_info(
      "creator",
      &[Coin {
        denom: "earth".to_string(),
        amount: Uint128::new(1000),
      }],
    );
    execute_instantiate(deps.as_mut(), info.clone());

    let provision: ExecuteMsg = ExecuteMsg::Provision{  };
    let res: Binary = execute(deps.as_mut(), mock_env(), info.clone(), provision).unwrap();
    let res: ProvisionResponse = from_binary(&res).unwrap();
  }
}