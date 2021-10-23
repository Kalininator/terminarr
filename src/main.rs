// use std::path::PathBuf;
use serde::Deserialize;
use serde::Serialize;
use serde_json::Result;
use std::collections::HashMap;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Terminarr {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Sonarr(Sonarr),
}

// Subcommand can also be externalized by using a 1-uple enum variant
#[derive(StructOpt)]
struct Sonarr {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    sonarr_command: SonarrCommand,
}

// subsubcommand!
#[derive(StructOpt)]
enum SonarrCommand {
    Status {},
    Series {},
    Login { url: String, api_key: String },
}

fn handle_sonarr(sonarr: Sonarr, config: &MyConfig) {
    match sonarr.sonarr_command {
        SonarrCommand::Status {} => {
            sonarr_status(config);
        }
        SonarrCommand::Series {} => {
            sonarr_series(config);
        }
        SonarrCommand::Login { url, api_key } => {
            let my_conf = MyConfig { url, api_key };
            confy::store("terminarr", my_conf);
            println!("Saved config",);
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SystemStatus {
    version: String,
    // buildTime: String,
    // isDebug: bool,
    // isProduction: bool,
    // isAdmin: bool,
    // isUserInteractive: bool,
    // startupPath: String,
    // appData: String,
    // osVersion: String,
    // isMono: bool,
    // isLinux: bool,
    // isWindows: bool,
    branch: String,
    authentication: String,
    // startOfWeek: Option<u8>,
    // urlBase: String,
}

fn sonarr_status(cfg: &MyConfig) {
    let url = format!("{}/api/system/status?apiKey={}", cfg.url, cfg.api_key);
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let status: SystemStatus = serde_json::from_str(&resp).unwrap();
    println!("{:?}", status);
}

#[derive(Serialize, Deserialize, Debug)]
struct Series {
    title: String,
    status: String,
    network: Option<String>,
}

fn sonarr_series(cfg: &MyConfig) {
    let url = format!("{}/api/series?apiKey={}", cfg.url, cfg.api_key);
    let resp = reqwest::blocking::get(url).unwrap().text().unwrap();
    let series: Vec<Series> = serde_json::from_str(&resp).unwrap();
    for s in series {
        println!("Series {} is {}", s.title, s.status);
    }
}

#[derive(Serialize, Deserialize)]
struct MyConfig {
    url: String,
    api_key: String,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            url: String::from("http://localhost:7878"),
            api_key: "foo".into(),
        }
    }
}

fn main() {
    let cfg: MyConfig = confy::load("terminarr").unwrap();
    let opts = Terminarr::from_args();
    match opts.cmd {
        Command::Sonarr(sonarr) => {
            handle_sonarr(sonarr, &cfg);
        }
    }
}
