use std::fs;

use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender, Config, config::{Appender, Root}, init_config, Handle};
use server::Node;
use server::file_ops::{FileInfo, NetworkFolder};
mod server;

#[async_std::main]
async fn main() {
    // log();
    // let mut node = Node::init().await;
    // node.run().await;

    let mut n = NetworkFolder::create();
    let f = fs::read("result.json").unwrap();
    let f_info: FileInfo = serde_json::from_slice(&f).unwrap();

    n.reconstruct_file(&f_info);
}


/**
 * Configure logging
 */
fn log() -> Handle {
    let stdout = ConsoleAppender::builder().build();

    let config = Config::builder()
        .appender(Appender::builder().build(
            "stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout")
        .build(LevelFilter::Trace)).unwrap();

    init_config(config).unwrap()
}

