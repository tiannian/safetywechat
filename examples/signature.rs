use warp::{ Filter };
use safetywechat::core::signature::Signature;
use safetywechat::core::message::EncryptedMessage;
use safetywechat::config::WechatBase;
use safetywechat::core::message::Query;

use std::convert::AsRef;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let signature = warp::query::query::<Signature>()
        .map(|sign: Signature| {
            let config = WechatBase {
                app_id: String::from("wx5a3dbaf21ec95f39"),
                secret: String::from("4c3e8320e3ea6b5b3a1d4878b40664d7"),
                token: String::from("abcdefgh"),
                aes_key: String::new()
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
                aes_key: String::from("2Ytb9xgNNs72AvD3W60iY3qFX9w4qgKWurto47l2Kfw"),
            };
            let data = Vec::from(bytes.as_ref());
            let s = String::from_utf8(data).unwrap();
            let message_body: EncryptedMessage = quick_xml::de::from_str(&s).unwrap();
            let message = message_body.decrypt(query, config).unwrap();
            // println!("{:?}", query);
            println!("{:?}", message);
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

