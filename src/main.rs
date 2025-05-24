mod argp;

use argp::argparse::{Argument, ArgumentParser};

use std::env;

pub use crate::argp::argparse;

fn main() {
    let argp = ArgumentParser {
        argsvec: env::args().collect(),
        prefix: '-',
        args: &[
            Argument {
                trigger: "input",
                mtrigger: "i",
                isflag: false,
                isrequired: true
            },
            Argument {
                trigger: "output",
                mtrigger: "o",
                isflag: false,
                isrequired: true
            },
            Argument {
                trigger: "verbose",
                mtrigger: "v",
                isflag: true,
                isrequired: false
            }
        ]
    };

    let fa = argp.compile();
    for arg in fa.args.iter().clone() {
        if arg.arg.isflag { println!("flag: {}", format!("{}, {}", arg.arg.trigger, arg.flagged)); }
        else { println!("val: {}", format!("{}, {}", arg.arg.trigger, arg.val)); }
    }
}
