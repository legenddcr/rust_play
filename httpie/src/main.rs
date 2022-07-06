use std::str::FromStr;
use anyhow::{anyhow, Result};
use clap::{AppSettings, Parser};
use reqwest::Url;

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Test <test@test.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmds: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    Get {
        #[clap(parse(try_from_str = parse_url))]
        url: String,
    },
    Post {
        #[clap(parse(try_from_str = parse_url))]
        url: String,
        #[clap(parse(try_from_str = parse_kv_pair))]
        body: Vec<String>,
    },
}

fn parse_url(s: &str) -> Result<String> {
    let _url: Url = s.parse()?;

    Ok(s.into())
}

#[derive(Debug)]
struct KvPair {
    k: String,
    v: String,
}

impl FromStr for KvPair {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split("=");
        let err = || anyhow!(format!("Failed to parse {}", s));
        Ok(Self {
            k: (split.next().ok_or_else(err)?).to_string(),
            v: (split.next().ok_or_else(err)?).to_string(),
        })
    }
}

fn parse_kv_pair(s: &str) -> Result<String> {
    Ok(s.parse()?)
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
