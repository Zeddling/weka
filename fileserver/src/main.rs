use log::LevelFilter;
use log4rs::{append::console::ConsoleAppender, Config, config::{Appender, Root}, init_config, Handle};
use server::Node;

mod server;

#[async_std::main]
async fn main() {
    log();
    let mut node = Node::init().await;
    node.run().await;
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

