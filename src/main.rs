extern crate ffmpeg_next as ffmpeg;

use log::info;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::Config;
use log4rs::config::Appender;
use log4rs::config::runtime::Root;
use log::LevelFilter;

fn setup_log4rs() {
    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Info))
        .unwrap();
    log4rs::init_config(config).unwrap();
}

fn main() {
    setup_log4rs();
    info!("testing");
}