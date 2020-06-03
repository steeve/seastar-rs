extern crate seastar;
use std::time::Duration;

fn main() {
    seastar::app_template::run(|| {
        seastar::asynq::spawn(async {
            println!("1!");
            seastar::asynq::sleep(Duration::from_secs(1)).await;
            println!("2!");
            seastar::asynq::sleep(Duration::from_secs(1)).await;
            println!("3!");
        });
        seastar::asynq::spawn(async {
            let size = seastar::asynq::file_size("/Users/steeve/go/src/github.com/steeve/seastar-rs/rs.h".to_string()).await;
            println!("file size is {}", size);
        });
    });
}
