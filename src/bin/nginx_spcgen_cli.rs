use std::{path::Path, process};

use clap::Parser;
use log::error;
use nginx_spcgen::{init_logger, Cli, CliArgs, DEFAULT_LOG_PATH};

fn main() {
    // 初始化日志
    let _ = match init_logger(Path::new(DEFAULT_LOG_PATH)) {
        Ok(handle) => handle,
        Err(_) => {
            eprintln!("日志初始化失败");
            process::exit(-1)
        }
    };

    let mut cli = Cli::create(CliArgs::parse());

    // 执行
    if let Err(e) = cli.startup() {
        error!("{:?}", e);
        process::exit(-1);
    }
}
