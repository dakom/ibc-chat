use app_tests::prelude::*;

#[tokio::test]
async fn integration() {
    let app = TestApp::new();

    let mut client = app.clone().into_contract_client(); 

    let resp = client.exec_send_message("hello world").await;

    // this should fail as we have not connected an ibc channel
    assert!(resp.is_err());

    // let's try to connect a channel
    // TODO ;)
}