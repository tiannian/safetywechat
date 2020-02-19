use warp::Filter;
use safetywechat::core::signature::Signature;
use safetywechat::config::OfficialAccount;
use safetywechat::config::WechatBase;

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("wechat")
        .and(warp::query::query::<Signature>())
        .map(|sign: Signature| {
            println!("{:?}", sign);
            let base = WechatBase {
                app_id: String::from("wx5a3dbaf21ec95f39"),
                secret: String::from("4c3e8320e3ea6b5b3a1d4878b40664d7"),
            };
            let config = OfficialAccount {
                base,
                token: String::from("abcdefgh"),
                aes_key: String::new()
            };
            let result = sign.validate(config);
            println!("{:?}", result);
            match result {
                Some(nonce) => nonce,
                None => String::from(""),
            }
        });

    warp::serve(hello)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

