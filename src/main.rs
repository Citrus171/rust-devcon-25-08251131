use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Hello Worldと返すハンドラーを定義
    async fn root_handler() -> String {
        "Hello Worl 000  55d".to_string()
    }

    // ルートを定義
    // "/"を踏むと、上で定義したroot_handlerを実行する
    let app = Router::new().route("/", get(root_handler));

    // 指定したポートにサーバを開く
    let listener = tokio::net::TcpListener::bind("0.0.0.0:4000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
