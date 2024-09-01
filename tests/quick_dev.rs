#![allow(unused)]

use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result<()> {

    // testing hello world routes
    let hc = httpc_test::new_client("http://localhost:8080")?;

    hc.do_get("/hello-world?name=Mrk").await?.print().await?;

    Ok(())
}