use ommatidia::{Detector, Engine, RemoteConfig};

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let docker: Engine = RemoteConfig::default()
        .try_into()
        .expect("Unable to access Docker");

    let detector = Detector::spawn(docker, "pure:0.1")
        .await
        .expect("Unable to spawn container");

    detector.stop().await;
}
