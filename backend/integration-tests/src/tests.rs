// use near_sdk::AccountId;
use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;

// use car_sharing::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initiate environment
    let worker = near_workspaces::sandbox().await?;

    // deploy contracts
    let car_sharing_wasm = near_workspaces::compile_project("..").await?;
    let car_sharing_contract = worker.dev_deploy(&car_sharing_wasm).await?;

    // create accounts
    let owner = worker.root_account().unwrap();
    let alice = owner
        .create_subaccount("alice")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;
    let bob = owner
        .create_subaccount("bob")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;
    let charlie = owner
        .create_subaccount("charlie")
        .initial_balance(NearToken::from_near(30))
        .transact()
        .await?
        .into_result()?;

    // Initialize contracts
    let _ = car_sharing_contract.call("init").transact().await?;

    // begin tests
    // create_owner_account(&owner, &car_sharing_contract, &alice, &bob).await?;

    Ok(())
}

async fn create_owner_account(
    owner: &Account,
    contract: &Contract,
    alice: &Account,
    bob: &Account,
) -> Result<(), Box<dyn std::error::Error>> {
    let owner_info = json!({
        "owner_id": "1",
        "name": "name",
    });
    let result = owner
        .call(contract.id(), "create_owner_account")
        .args_json(&owner_info)
        .transact()
        .await?.into_result()?;
    assert_eq!(result, Ok("test"));
    // Should return an error
    owner
        .call(contract.id(), "create_owner_account")
        .args_json(&owner_info)
        .transact()
        .await?;
    println!("      Passed ✅ create_owner_account");
    Ok(())
}
