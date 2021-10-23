// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Terminarr {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Radarr(Radarr),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt)]
struct Radarr {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    radarr_command: RadarrCommand,
}

// subsubcommand!
#[derive(StructOpt)]
enum RadarrCommand {
    Status {},
}

fn handle_radarr(radarr: Radarr) {
    match radarr.radarr_command {
        RadarrCommand::Status {} => {
            println!("Radarr status",);
        }
    }
}

fn main() {
    let opts = Terminarr::from_args();
    match opts.cmd {
        Command::Radarr(radarr) => {
            handle_radarr(radarr);
        }
    }
}
