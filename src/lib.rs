//! # Nginx Simple Proxy Config Generator
//!
//! 这是一个基 toml 配置文件生成简单 Nginx 反向代理的生成器。
//!
//! 这个工具仅仅是为了 Nginx 运行在轻量设备上（如路由器等），由于某些原因不能/不便使用图形化配置工具，
//! 使用统一的配置文件生成复数反向代理配置文件的需求而生的。
//!

mod cli;
mod logger;

pub use cli::{Cli, CliArgs};
pub use logger::init_logger;

pub const DEFAULT_LOG_PATH: &'static str = "result.log";
pub const DEFAULT_CONFIG_PATH: &'static str = "config.toml";
pub const DEFAULT_OUTPUT_PATH: &'static str = "output";

pub const DEFAULT_CONFIG_FILE: &'static str = r#"# 示例配置文件

# 主机配置
# 可使用 Nginx 变量，此处的 `$server_addr` 指代 Nginx 本机地址
[server0]
domain = "server0.com"
forward_server = "$server_addr"

# 服务配置
# `https://www.server0.com` --- Nginx --> `https://$server_addr:443`
[[server0.services]]
name = "www"
forward_scheme = "https"
port = 443

# 服务配置
# `http://nav.server0.com` --- Nginx --> `https://$server_addr:80`
[[server0.services]]
name = "nav"
forward_scheme = "http"
port = 80


# 主机配置
[server1]
domain = "server1"
forward_server = "192.168.1.100"

# 服务配置
# `http://h5.server1` --- Nginx --> `https://192.168.1.100:8000`
[[server0.services]]
name = "h5"
forward_scheme = "http"
port = 8000
"#;

pub const NGINX_CONFIG_LOCATION: &'static str = r#"location / {
    add_header       X-Served-By        $host;
    proxy_set_header Host               $host;
    proxy_set_header X-Forwarded-Scheme $scheme;
    proxy_set_header X-Forwarded-Proto  $scheme;
    proxy_set_header X-Forwarded-For    $proxy_add_x_forwarded_for;
    proxy_set_header X-Real-IP          $remote_addr;
    proxy_pass       $forward_scheme://$server:$port$request_uri;
}"#;
