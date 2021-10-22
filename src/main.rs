// #[macro_use]
// extern crate structopt;

// use std::path::PathBuf;
use structopt::StructOpt;

// #[derive(StructOpt, Debug)]
// #[structopt(name = "git", about = "the stupid content tracker")]
// enum Git {
//     #[structopt(name = "add")]
//     Add {
//         #[structopt(short = "i")]
//         interactive: bool,
//         #[structopt(short = "p")]
//         patch: bool,
//         #[structopt(parse(from_os_str))]
//         files: Vec<PathBuf>,
//     },
//     #[structopt(name = "fetch")]
//     Fetch {
//         #[structopt(long = "dry-run")]
//         dry_run: bool,
//         #[structopt(long = "all")]
//         all: bool,
//         repository: Option<String>,
//     },
//     #[structopt(name = "commit")]
//     Commit {
//         #[structopt(short = "m")]
//         message: Option<String>,
//         #[structopt(short = "a")]
//         all: bool,
//     },
// }

#[derive(StructOpt, Debug)]
#[structopt(name = "terminarr", about = "Manage *arrs")]
enum Terminarr {
    #[structopt(name = "status")]
    Status {},
}

fn main() {
    match Terminarr::from_args() {
        Terminarr::Status {} => {
            println!("Get status");
        } // _ => (),
    }
}
