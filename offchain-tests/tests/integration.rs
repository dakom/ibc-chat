use app_tests::prelude::*;

#[tokio::test]
async fn integration() {
    let app = TestApp::new();

    let mut client = app.clone().into_contract_client(); 


    let resp = client.exec_send_message("hello world").await;

    // TODO - we want to get IBC working so that this assertion can pass
    // this should fail as we have not connected an ibc channel
    assert!(resp.is_err());

    // TODO (after above TODO)- run same exact test as onchain-tests/src/runner.rs
}