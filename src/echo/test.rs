use super::*;

#[tokio::test]
async fn echo_t() {
    Echo::warn("hello");
    Echo::info("hello");
    Echo::success("hello");
    Echo::error("hello");
    let stop = Echo::progress("hello");
    tokio::spawn(async {
        sleep(Duration::from_millis(1000)).await;
    })
    .await
    .unwrap();
    stop();
}
