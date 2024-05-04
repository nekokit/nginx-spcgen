use std::process;

use clap::Parser;
use nginx_spcgen::{Cli, CliArgs};

fn main() {
    // 创建
    let mut cli = Cli::create(CliArgs::parse());

    // 执行
    if let Err(e) = cli.startup() {
        eprintln!("{:?}", e);
        process::exit(-1);
    }
}
