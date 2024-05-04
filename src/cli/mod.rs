//! # Cli 模块

use std::{
    collections::HashMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};

use crate::{DEFAULT_CONFIG_FILE, DEFAULT_CONFIG_PATH, DEFAULT_OUTPUT_PATH, NGINX_CONFIG_LOCATION};

/// # Cli 程序对象
///
/// 存储相关信息。
pub struct Cli {
    /// 命令行参数
    cli_args: CliArgs,

    /// 反向代理配置信息
    server_config: HashMap<String, HostInfo>,
}

impl Cli {
    /// 通过命令行参数创建 Cli 对象
    ///
    /// # Arguments
    ///
    /// - `cli_args` - 命令行参数
    ///
    /// # Returns
    ///
    /// Cli 对象
    pub fn create(cli_args: CliArgs) -> Self {
        Self {
            cli_args,
            server_config: HashMap::new(),
        }
    }

    /// 获取配置文件路径
    fn get_config_path(&self) -> PathBuf {
        self.cli_args
            .file
            .clone()
            .unwrap_or(PathBuf::from(DEFAULT_CONFIG_PATH))
    }

    /// 工具运行入口
    pub fn startup(&mut self) -> Result<()> {
        match &self.cli_args.main_command {
            MainCommand::Example => self.example(),
            MainCommand::Test => {
                self.load_config()?;
                self.display_config();
                Ok(())
            }
            MainCommand::Generate { output } => {
                let output_path = match output {
                    Some(v) => v.to_path_buf(),
                    None => PathBuf::from(DEFAULT_OUTPUT_PATH),
                };
                self.load_config()?;
                self.display_config();
                self.generate_nginx_config(&output_path)
            }
        }
    }

    /// 写入示例配置文件
    pub fn example(&self) -> Result<()> {
        let path = self.get_config_path();
        fs::File::create(path)?.write_all(DEFAULT_CONFIG_FILE.as_bytes())?;
        Ok(())
    }

    /// 从文件加载配置文件
    pub fn load_config(&mut self) -> Result<()> {
        let path = self.get_config_path();
        self.server_config = toml::from_str(&fs::read_to_string(path)?)?;
        Ok(())
    }

    /// 列出识别的配置
    pub fn display_config(&self) {
        for (host_name, host) in self.server_config.iter() {
            let mut services_list = Vec::new();
            services_list.push(format!("[{}]", host_name.red()));
            for service in host.services.iter() {
                services_list.push(format!(
                    "http://{}.{} -> {}://{}:{}",
                    service.name.bright_green(),
                    host.domain.green(),
                    service.forward_scheme.cyan(),
                    host.forward_server.blue(),
                    service.port.to_string().bright_blue()
                ));
            }
            println!("{}\n", services_list.join("\n"));
        }
    }

    /// 生成配置
    ///
    /// # Arguments
    ///
    /// - `path` - 输出文件夹路径
    pub fn generate_nginx_config(&self, path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir(path)?;
        } else if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            bail!("output path is not a dir: {}", path.display())
        };
        for (host_name, host) in &self.server_config {
            let config_path = path.join(format!("{}.conf", host_name));
            let mut content = Vec::new();
            for service in &host.services {
                let service_content = format!(
                    r#"# {}
server {{
  server_name         {}.{};
  set $forward_scheme {};
  set $server         {};
  set $port           {};

  listen 80;
  listen [::]:80;
  include conf.d/proxy_include/location.conf;
}}
"#,
                    service.name,
                    service.name,
                    host.domain,
                    service.forward_scheme,
                    host.forward_server,
                    service.port
                );
                content.push(service_content);
            }
            fs::File::create(config_path)?.write_all(content.join("\n").as_bytes())?;
        }
        Self::write_location(&path.join("proxy_include"))?;
        println!("已完成！输出文件夹：{}", path.display());
        Ok(())
    }

    /// 写入 location 配置
    fn write_location(path: &Path) -> Result<()> {
        if !path.exists() {
            fs::create_dir(path)?;
        } else if path.is_dir() {
            fs::remove_dir_all(path)?;
        } else {
            bail!("output path is not a dir: {}", path.display())
        };
        fs::File::create(path.join("location.conf"))?
            .write_all(NGINX_CONFIG_LOCATION.as_bytes())?;
        Ok(())
    }
}

/// # 主机信息
///
/// 一个主机负责一个域名，多个服务，每个服务使用子域名。
#[derive(Deserialize, Serialize)]
pub struct HostInfo {
    /// 域名
    pub domain: String,
    /// 代理服务器
    pub forward_server: String,
    /// 服务列表
    pub services: Vec<ServiceInfo>,
}

/// # 服务信息
#[derive(Deserialize, Serialize)]
pub struct ServiceInfo {
    /// 名称兼子域名
    pub name: String,
    /// 代理方式
    pub forward_scheme: String,
    /// 端口号
    pub port: u16,
}

/// # 命令行参数
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    /// 选项：指定文件路径
    #[arg(
        short,
        long,
        help = "指定文件路径",
        long_help = "指定文件路径，默认为工作路径下的 `config.toml`，会覆盖原有文件。"
    )]
    file: Option<PathBuf>,

    /// 主要命令
    #[command(subcommand)]
    pub main_command: MainCommand,
}

/// # 主要命令模块
#[derive(Debug, Subcommand)]
pub enum MainCommand {
    /// 生成示例配置
    #[command(about = "生成示例配置")]
    Example,

    /// 测试配置
    #[command(about = "测试配置")]
    Test,

    /// 生成 Nginx 配置
    #[command(about = "生成 Nginx 配置")]
    Generate {
        /// 选项：Nginx 配置输出文件夹
        #[arg(
            short,
            long,
            help = "Nginx 配置输出文件夹",
            long_help = "指定文件路径，默认为工作路径下的 `output`，会先清空文件夹。"
        )]
        output: Option<PathBuf>,
    },
}
