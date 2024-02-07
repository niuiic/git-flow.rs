use super::*;

#[tokio::test]
async fn echo_t() {
    Echo::warning("hello");
    Echo::info("hello");
    Echo::success("hello");
    Echo::error("hello");
    let finish = Echo::progress("hello");
    tokio::spawn(async {
        sleep(Duration::from_millis(1000)).await;
    })
    .await
    .unwrap();
    finish(true, "hello");
}
