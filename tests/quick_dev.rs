use anyhow::Result;

#[tokio::test]
async fn quick_dev_root() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000/")?;
    hc.do_get("/").await?.print().await?;

    Ok(())
}

#[tokio::test]
async fn quick_dev_assets() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000/")?;
    hc.do_get("/assets").await?.print().await?;

    Ok(())
}

