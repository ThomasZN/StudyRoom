use std::time::Duration;
use tokio::{runtime::Runtime, time::sleep};

async fn count_and_run<F>(count: u32, f: F)
where
    F: Fn() -> &'static str,
{
    for i in 1..=count {
        sleep(Duration::from_secs(1)).await;
        println!("[{}/{}] {}", i, count, f());
    }
}

async fn learn_song() -> &'static str {
    count_and_run(5, || "learn song").await;
    "Bury the Light"
}

async fn sing_song(song_name: &'static str) {
    count_and_run(3, || song_name).await;
}

async fn dance() {
    count_and_run(10, || "dance").await;
}

async fn learn_and_sing() {
    // 要唱歌必须得先学会歌曲.
    // 我们这里使用 `.await` 而不是 `block_on` 来
    // 防止线程阻塞, 这样也可以同时跳舞.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();

    // `join!` 类似 `.await`，但是可以同时等待多个 `future` 执行完成.
    // 如果我们 `learn_and_sing` 这个 `future` 被阻塞, 那么 `dance`
    // 这个 `future` 将接管当前的线程. 如果 `dance` 被阻塞, 那么 `learn_and_sing`
    // 就可以重新开始. 如果这个两个 `future` 都被阻塞, 那么 `async_main`
    // 也将被阻塞并让位给执行程序.
    tokio::join!(f1, f2);
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(async_main());
}
