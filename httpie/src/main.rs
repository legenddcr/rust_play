use clap::{AppSettings, Parser};

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
        url: String,
    },
    Post {
        url: String,
        body: Vec<String>,
    },
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
