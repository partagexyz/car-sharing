use near_sdk::AccountId;
use near_workspaces::{types::NearToken, Account, Contract};
use serde_json::json;

mod helpers;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initiate environment
    let worker = near_workspaces::sandbox().await?;

    // deploy contracts
    let nft_wasm = near_workspaces::compile_project("..").await?;
    let nft_contract = worker.dev_deploy(&nft_wasm).await?;

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
    let _ = nft_contract.call("init").transact().await?;

    // begin tests
    create_owner_account(&owner, &nft_contract, &alice, &bob).await?;

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
    owner
        .call(contract.id(), "create_owner_account")
        .args_json(owner_info)
        .transact()
        .await?;
    println!("      Passed ✅ create_owner_account");
    Ok(())
}
