#[tokio::test]
async fn test_contract_deployment() -> Result<(), Box<dyn std::error::Error>> {
    let sandbox = near_workspaces::sandbox().await?;
    let contract_wasm = near_workspaces::compile_project("./").await?;

    let user_account = sandbox.dev_create_account().await?;

    let deploy_contract_outcome = user_account.deploy(&contract_wasm).await?;
    assert!(deploy_contract_outcome.is_success());

    Ok(())
}