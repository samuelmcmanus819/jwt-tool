# Secret Messenger Contract
The secret messenger contract is the backend for the Secret Messenger DApp

## Future Work
- Implement permissioned viewing (see SNIP-20 permissioned viewing in SCRT docs)
- Implement connection-orientation
- Implement groups

## CLI Usage
### Deployment on Local Secret
1. Change to this directory.
2. Build the contract by running `make build`
3. Start your local secret node and upload this directory. 
```
docker run -it --rm \
 -p 9091:9091 -p 26657:26657 -p 1317:1317 -p 5000:5000 \
 -v $(pwd):/root/code \
 --name localsecret ghcr.io/scrtlabs/localsecret
```
4. Pop a shell on your secret node `docker exec -it localsecret /bin/bash`
5. Change directories to the code directory `cd code`
6. Upload the contract to the local network  `secretd tx compute store contract.wasm.gz --from a --gas 1000000 -y --keyring-backend test`
7. Get the code ID of the contract `secretd query compute list-code`
8. Instantiate the contract `secretd tx compute instantiate $CODE_ID '{ "name": "secret messenger beta" }' --from a --label "my secret messenger" -y --keyring-backend test`

Note: If this doesn't work, then visit the [Secret Network](https://github.com/scrtlabs/SecretNetwork) Github and verify that you're running the latest CLI version. 

9. Get the address of the contract `secretd query compute list-contract-by-code $CODE_ID`

#### Add Users
1. Execute an add user command `secretd tx compute execute $CONTRACT_ADDRESS '{ "register": { "username": <your-username> } }' --from <wallet-name>`

Note: You'll need to execute the above command from two different wallets to send messages back and forth.

2. Verify that your user exists `secretd query compute query $CONTRACT_ADDRESS '{ "get_all_users": {  } }'`

#### Send a Message
Note: To perform this action you'll need to have two registered users.
1. Get a list of all chattable users (all users except for your current user) `secretd query compute query $CONTRACT_ADDRESS '{ "get_chattable_users": { "self_address": "$USER1_WALLET_ADDRESS" } }'`
2. Send a message to another user `secretd tx compute execute $CONTRACT_ADDRESS '{ "send_message": { "recipient": "$RECIPIENT_ADDRESS", "message": "Hi" } }' --from a --keyring-backend test`
3. Get a list of all messages between you and the other user `secretd query compute query $CONTRACT_ADDRESS '{ "get_messages": { "self_address": "$USER1_WALLET_ADDRESS", "user2": "$USER2_WALLET_ADDRESS" } }'`

### Deployment on Testnet
1. Set up your development environment to use the SCRT testnet.
```
secretcli config node https://rpc.pulsar.scrttestnet.com
secretcli config output json
secretcli config chain-id pulsar-2
secretcli keys add <a-name> --recover
```
2. Change directories to this directory. 
3. Build the code by running `make build`
4. Upload the contract to the local network  `secretcli tx compute store contract.wasm.gz --from <wallet-name> --gas 2000000 -y`
5. Get the code ID of the contract `secretcli query compute list-code`. Typically your code ID will be the latest one, but you can verify by ensuring that the `creator` is your wallet ID
6. Instantiate the contract `secretcli tx compute instantiate $CODE_ID '{ "name": "secret messenger beta" }' --from testnet --label "my secret messenger rrandomstring" -y`
7. Find the address of the code `secretcli query compute list-contract-by-code $CODE_ID`

#### Add Users
1. Execute an add user command `secretcli tx compute execute $CONTRACT_ADDRESS '{ "register": { "username": <your-username> } }' --from <wallet-name>`

Note: You'll need to execute the above command from two different wallets to send messages back and forth.

2. Verify that your user exists `secretcli query compute query $CONTRACT_ADDRESS '{ "get_all_users": {  } }'`

#### Send a Message
Note: To perform this action you'll need to have two registered users.
1. Get a list of all chattable users (all users except for your current user) `secretcli query compute query $CONTRACT_ADDRESS '{ "get_chattable_users": { "self_address": "$USER1_WALLET_ADDRESS" } }'`
2. Send a message to another user `secretcli tx compute execute $CONTRACT_ADDRESS '{ "send_message": { "recipient": "$RECIPIENT_ADDRESS", "message": "Hi" } }' --from <wallet-name>`
3. Get a list of all messages between you and the other user `secretcli query compute query $CONTRACT_ADDRESS '{ "get_messages": { "self_address": "$USER1_WALLET_ADDRESS", "user2": "$USER2_WALLET_ADDRESS" } }'`