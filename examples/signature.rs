use warp::{ Filter };
use safetywechat::core::signature::Signature;
use safetywechat::config::WechatBase;
use safetywechat::core::message::Query;
use safetywechat::config::{ MessageFormat, EncryptMode };
use safetywechat::core::server::Server;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let signature = warp::query::query::<Signature>()
        .map(|sign: Signature| {
            let config = WechatBase {
                app_id: String::from("wx5a3dbaf21ec95f39"),
                secret: String::from("4c3e8320e3ea6b5b3a1d4878b40664d7"),
                token: String::from("abcdefgh"),
                aes_key: Some(String::new()),
                msg_type: MessageFormat::XML,
                encrypt_mode: EncryptMode::Plaintext,
            };
            println!("{:?}", sign);
            let result = sign.validate(config.clone());
            println!("{:?}", result);
            match result {
                Some(nonce) => nonce,
                None => String::from(""),
            }
        });

    let message = warp::body::content_length_limit(1024 * 32)
        .and(warp::query::query())
        .and(warp::body::bytes())
        .map(|query: Query, bytes: bytes::Bytes| {
            let config = WechatBase {
                app_id: String::from("wx5a3dbaf21ec95f39"),
                secret: String::from("4c3e8320e3ea6b5b3a1d4878b40664d7"),
                token: String::from("abcdefgh"),
                aes_key: Some(String::from("2Ytb9xgNNs72AvD3W60iY3qFX9w4qgKWurto47l2Kfw")),
                msg_type: MessageFormat::Json,
                encrypt_mode: EncryptMode::Encrypted,
            };
            let server = Server::new(config);
            let r = server.parse_input(query, bytes);
            println!("{:?}", r);
            String::new()
        });

    let wechat = warp::path!("wechat").and(
        warp::get().and(signature)
        .or(warp::post().and(message))
    );


    warp::serve(wechat)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

