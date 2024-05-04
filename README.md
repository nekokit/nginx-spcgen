# Nginx Simple Proxy Config Generator

这是一个基 toml 配置文件生成简单 Nginx 反向代理的生成器。

这个工具仅仅是为了 Nginx 运行在轻量设备上（如路由器等），由于某些原因不能/不便使用图形化配置工具，
使用统一的配置文件生成复数反向代理配置文件的需求而生的。

##  说明

这是一个简单的 Nginx 反向代理配置文件生成器，通过一个统一的配置文件生成模块化的 Nginx Server 配置文件。
配置文件应被 include 进 Nginx 的 `http` 模块中。

由于是自用简单小工具，并不会支持过多的自定义配置，目前支持的功能如下：

- 生成示例 toml 配置
- 测试 toml 配置
- 根具 toml 配置生成 Nginx 配置文件

Nginx 监听本地 80 端口接受 http 请求，收到特定域名请求后转发到指定地址和端口。

## 示例配置文件

```toml
# 示例配置文件

# 主机配置
# 可使用 Nginx 变量，此处的 `$server_addr` 指代 Nginx 本机地址
[server0]
domain = "server0.com"
forward_server = "$server_addr"

# 服务配置
# `http://www.server0.com` --- Nginx --> `https://$server_addr:443`
[[server0.services]]
name = "www"
forward_scheme = "https"
port = 443

# 服务配置
# `http://nav.server0.com` --- Nginx --> `http://$server_addr:80`
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
```

## 使用

下载 Release 中的对应版本，通过 shell 调用。

命令形式: `nginx_spcgen_cli [选项] <命令>`

选项:
- -f, --file <配置文件路径>
  - 指定文件路径，默认为工作路径下的 `config.toml`，会覆盖原有文件。
- -h, --help
  - 显示帮助（使用 `-h` 查看概括）
- -V, --version
  - 显示版本信息

命令:
- example - 生成示例配置
- test - 测试配置
- generate [生成选项] - 生成 Nginx 配置
- help - 显示帮助

生成选项:
- -o, --output <输出文件夹>
  - 指定文件路径，默认为工作路径下的 `output`，会先清空文件夹。
- -h, --help
  - 显示生成帮助（使用 `-h` 查看概括）

例：

```shell
nginx_spcgen_cli generate
```

```shell
nginx_spcgen_cli -f config.toml generate -o output
```

生成的 Nginx 配置默认在工作路径的 `output` 文件夹下，
将该文件夹所有内容拷贝到 `/etc/nginx/conf.d` 文件夹（该文件夹一般默认被 include 进 Nginx 的 http 模块）,
或复制到其他文件夹并在 Nginx 默认配置中的 http 模块中增加：

```nginx
include <文件夹路径>/*.conf
```
