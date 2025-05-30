extern crate ffmpeg_next as ffmpeg;

mod argp;

use argp::argparse::{Argument, ArgumentParser};

use log::info;
use log::LevelFilter;
use std::env;

use log4rs::append::console::ConsoleAppender;
use log4rs::config::Config;
use log4rs::config::Appender;
use log4rs::config::runtime::Root;

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
    let argp = ArgumentParser {
        argsvec: env::args().collect(),
        prefix: '-',
        name: "tea",
        desc: "tea goes from one pot to another, maybe something happened",
        args: &[
            Argument {
                trigger: "input",
                mtrigger: "i",
                isflag: false,
                isrequired: true,
                help: "path to input file"
            },
            Argument {
                trigger: "output",
                mtrigger: "o",
                isflag: false,
                isrequired: true,
                help: "path to output file"
            },
            Argument {
                trigger: "verbose",
                mtrigger: "v",
                isflag: true,
                isrequired: false,
                help: "will print more debug stuff"
            },
            Argument {
                trigger: "nopluh",
                mtrigger: "np",
                isflag: true,
                isrequired: false,
                help: "don't print the first line :("
            }
        ]
    };

    let fa = argp.compile();

    match fa.get("nopluh") {
        Some(arg) => {
            if !arg.flagged { info!("pluh @umatterbro dc/ig/gh https://github.com/cutsweettea/tfc"); }
        }
        None => {}
    }

    let inf;
    let outf;
    match fa.get("input") {
        Some(arg) => { inf = arg.val.clone(); },
        None => { panic!("no input file specified"); }
    }

    match fa.get("output") {
        Some(arg) => { outf =  arg.val.clone(); },
        None => { panic!("no output file specified"); }
    }

    info!("{}", format!("converting input '{}' and writing output to '{}'", inf, outf));
}