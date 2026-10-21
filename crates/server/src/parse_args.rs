use clap::Parser;
use lemmy_server::CmdArgs as LemmyCmdArgs;

#[derive(Parser, Debug)]
pub struct PTTCmdArgs {
    #[command(flatten)]
    pub lemmy_args: LemmyCmdArgs,
}

pub fn parse_args() -> PTTCmdArgs {
    let mut raw_args: Vec<String> = std::env::args().collect();

    // migration subcommand.
    if raw_args.iter().any(|s| s == "migration") {
        return PTTCmdArgs::parse_from(raw_args);
    }

    raw_args.push("--disable-http-server".to_string());
    let args = PTTCmdArgs::parse_from(raw_args);

    println!("parse_args: args: {args:#?}");

    args
}
