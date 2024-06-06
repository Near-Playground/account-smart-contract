use near_sdk::NearToken;
use near_workspaces::types::{KeyType, SecretKey};
use serde_json::json;

#[tokio::test]
async fn create_will() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let owner = sandbox.dev_deploy(&contract_wasm).await?;
    let other_user = sandbox.dev_create_account().await?;

    let new_secret_key = SecretKey::from_random(KeyType::ED25519);
    let new_public_key = new_secret_key.public_key();

    let current_timestamp = sandbox.view_block().await?.timestamp();
    let one_week_later = current_timestamp + 604800;

    let other_user_create_will_outcome = other_user
        .call(owner.id(), "create_will")
        .args_json(json!({
            "beneficiary_public_key": new_public_key.to_string(),
            "beneficiary_effective_date": one_week_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact().await?;
    assert!(other_user_create_will_outcome.is_failure());

    let owner_create_will_without_deposit_outcome = owner
        .call("create_will")
        .args_json(json!({
            "beneficiary_public_key": new_public_key.to_string(),
            "beneficiary_effective_date": one_week_later
        }))
        .transact()
        .await?;
    assert!(owner_create_will_without_deposit_outcome.is_failure());

    let owner_create_will_outcome = owner
        .call("create_will")
        .args_json(json!({
            "beneficiary_public_key": new_public_key.to_string(),
            "beneficiary_effective_date": one_week_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(owner_create_will_outcome.is_success());

    Ok(())
}

#[tokio::test]
async fn delete_will() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let owner = sandbox.dev_deploy(&contract_wasm).await?;
    let other_user = sandbox.dev_create_account().await?;

    let other_user_delete_will_outcome = other_user
        .call(owner.id(), "delete_will")
        .args_json(json!({}))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(other_user_delete_will_outcome.is_failure());

    let owner_delete_will_without_deposit_outcome = owner
        .call("delete_will")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(owner_delete_will_without_deposit_outcome.is_failure());

    let owner_delete_will_outcome = owner
        .call("delete_will")
        .args_json(json!({}))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(owner_delete_will_outcome.is_success());

    Ok(())
}

#[tokio::test]
async fn execute_will() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let owner = sandbox.dev_deploy(&contract_wasm).await?;

    let other_user = sandbox.dev_create_account().await?;

    let execute_empty_will_outcome = other_user
        .call(owner.id(), "execute_will")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(execute_empty_will_outcome.is_failure());

    let new_secret_key = SecretKey::from_random(KeyType::ED25519);
    let new_public_key = new_secret_key.public_key();

    let current_timestamp = sandbox.view_block().await?.timestamp();
    let one_week_before = current_timestamp - 604800;

    let owner_create_will_outcome = owner
        .call("create_will")
        .args_json(json!({
            "beneficiary_public_key": new_public_key.to_string(),
            "beneficiary_effective_date": one_week_before
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(owner_create_will_outcome.is_success());

    let execute_will_outcome = other_user
        .call(owner.id(), "execute_will")
        .args_json(json!({}))
        .transact()
        .await?;
    assert!(execute_will_outcome.is_success());

    Ok(())
}

#[tokio::test]
async fn extend_will() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let owner = sandbox.dev_deploy(&contract_wasm).await?;
    let other_user = sandbox.dev_create_account().await?;

    let current_timestamp = sandbox.view_block().await?.timestamp();
    let one_week_later = current_timestamp + 604800;
    let two_weeks_later = current_timestamp + 1209600;

    let extend_will_without_will_outcome = owner
        .call("extend_will")
        .args_json(json!({
            "beneficiary_effective_date": two_weeks_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(extend_will_without_will_outcome.is_failure());

    let new_secret_key = SecretKey::from_random(KeyType::ED25519);
    let new_public_key = new_secret_key.public_key();
    
    let owner_create_will_outcome = owner
        .call("create_will")
        .args_json(json!({
            "beneficiary_public_key": new_public_key.to_string(),
            "beneficiary_effective_date": one_week_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(owner_create_will_outcome.is_success());

    let other_user_extend_will_outcome = other_user
        .call(owner.id(), "extend_will")
        .args_json(json!({
            "beneficiary_effective_date": two_weeks_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(other_user_extend_will_outcome.is_failure());

    let owner_extend_will_without_deposit_outcome = owner
        .call("extend_will")
        .args_json(json!({
            "beneficiary_effective_date": two_weeks_later
        }))
        .transact()
        .await?;
    assert!(owner_extend_will_without_deposit_outcome.is_failure());

    let owner_extend_will_outcome = owner
        .call("extend_will")
        .args_json(json!({
            "beneficiary_effective_date": two_weeks_later
        }))
        .deposit(NearToken::from_yoctonear(1))
        .transact()
        .await?;
    assert!(owner_extend_will_outcome.is_success());

    Ok(())
}

#[tokio::test]
async fn get_will() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let owner = sandbox.dev_deploy(&contract_wasm).await?;

    let get_will_outcome = owner.view("get_will").args_json(json!({})).await;
    assert!(get_will_outcome.is_ok());

    Ok(())
}