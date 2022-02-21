use crate::MadomeClient;
use log::Level;

#[tokio::test]
async fn create_token() {
    simple_logger::init_with_level(Level::Debug).unwrap();

    let client = MadomeClient::nightly();

    let email = "user@madome.app";

    let _r = client.auth().create_authcode(email).await.unwrap();

    let code = client
        .e2e_channel("http://localhost:32148")
        .authcode(email)
        .await;

    log::info!("code = {}", code.code);

    let _r = client
        .auth()
        .create_token_pair(email, code.code)
        .await
        .unwrap();

    let user = client.user().get_me().await.unwrap();

    assert_eq!(user.email, email);

    // 서버에 e2e 전용 리퀘스트 보내서 인증코드 받아와야됨
    // 차라리 분리된 서버를 만들어서 거기에서 가져오는 게 나을 듯
    //
    // 서버를 열고
    // 요청 보내고 -> broker.e2e.madome.app
    // 응답 받음(비어있음) <- broker.e2e.madome.app
    // 연 서버에 요청 받음 <- broker.e2e.madome.app
    // 잘 받았음^^ -> broker.e2e.madome.app
    //
    // 서버 포트는 e2e channel 요청 보낼때 같이 보내야지
    //
    // madome-runner:<PORT>
    //
    // client.debug().get_authcode("user@madome.app").await.unwrap()
}
