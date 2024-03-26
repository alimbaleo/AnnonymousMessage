#![allow(unused)]
use anyhow::Result;

#[tokio::test]
async fn quick_dev() -> Result <()>{
    let hc = httpc_test::new_client("http://localhost:5500");

    hc.expect("reason").do_get("/hello").await?.print().await?;
    Ok(())
}

