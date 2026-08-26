# totp-cli

<div align="center">

[![GitHub Release](https://img.shields.io/github/v/release/agb1964/totp-cli?color=blue&logo=github)](https://github.com/agb1964/totp-cli/releases/latest)
[![GitHub Downloads (latest release)](https://img.shields.io/github/downloads/agb1964/totp-cli/latest/total?color=blue&logo=github)](https://github.com/agb1964/totp-cli/releases/latest)
[![CI](https://github.com/agb1964/totp-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/agb1964/totp-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![RFC 6238](https://img.shields.io/badge/RFC-6238%20Compliant-success.svg)](https://datatracker.ietf.org/doc/html/rfc6238)

**用于双因素身份验证 (2FA/MFA) 的快速、轻量级 TOTP 命令行工具，支持在终端中直接渲染二维码。**

<br />

[English](README.md) • [Русский](README.ru.md) • [**简体中文**](README.zh.md)

<br />

[功能特性](#-功能特性) •
[安装指南](#-安装指南) •
[使用说明](#-使用说明) •
[退出状态码](#-退出状态码) •
[应用兼容性](#-应用兼容性) •
[项目状态](#-项目状态) •
[开发与测试](#-开发与测试) •
[开源协议](#-开源协议)

</div>

---

> [!NOTE]
>
> **📌 项目状态与维护说明**
>
> 本项目最初是为了满足作者特定的实际应用需求而创建的，目前的功能已完全达成既定目标。因此，作者后续暂无计划对本项目进行持续的功能扩展或主动维护。
>
> 本项目功能完整、测试充分且运行稳定。如果您需要更多功能、扩展或定制化修改，**非常欢迎随时 Fork 本项目**！您可以在 MIT 开源协议允许的范围内自由修改、复用和衍生开发。

---

## ✨ 功能特性

- 🔑 **Base32 密钥生成**：生成符合密码学安全标准的 160 位（20 字节）Base32 编码密钥。
- 📱 **终端二维码渲染**：直接在控制台中绘制紧凑的 Unicode 二维码，方便手机身份验证器即时扫描。
- 🔗 **标准 otpauth URI 规范**：支持标准的 `otpauth://totp/{Issuer}:{Account}?secret=...&issuer={Issuer}` 格式。
- ⏱️ **严格遵循 RFC 6238 规范**：
  - 算法：**HMAC-SHA1**
  - 验证码长度：**6 位数字**
  - 时间步长 (Time Step)：**30 秒**
  - 容错时间漂移 (Skew)：**±1 个周期**（±30 秒，有效解决设备间时钟不同步问题）。
- 🤖 **便于脚本自动化与 CI/CD**：提供确定且规范的进程退出状态码（Exit Codes），方便编写自动化脚本。
- ⚡ **极佳性能与零冗余**：使用 Rust 编写，编译为单一二进制可执行文件，无多余外部依赖。

---

## 📦 安装指南

### 方式 1：直接下载预编译二进制文件（推荐）

您可以直接前往 [**Releases**](https://github.com/agb1964/totp-cli/releases) 页面下载适用于您操作系统的独立可执行文件压缩包：

| 操作系统 | 架构 | 压缩包文件 |
| --- | --- | --- |
| 🐧 **Linux** | x86_64 | `totp-cli-linux-x86_64.tar.gz` |
| 🍏 **macOS** | Apple Silicon (arm64) | `totp-cli-macos-arm64.tar.gz` |
| 🪟 **Windows** | x86_64 | `totp-cli-windows-x86_64.zip` |

#### 终端快速下载与安装（Linux / macOS）

```bash
# 1. 下载适合您系统的压缩包（以 Linux x86_64 为例）：
curl -LO https://github.com/agb1964/totp-cli/releases/latest/download/totp-cli-linux-x86_64.tar.gz

# macOS Apple Silicon (M1/M2/M3/M4) 请使用：
# curl -LO https://github.com/agb1964/totp-cli/releases/latest/download/totp-cli-macos-arm64.tar.gz

# 2. 解压压缩包
tar -xzf totp-cli-*.tar.gz

# 3. 将二进制文件移动至系统的 PATH 路径中
sudo mv totp-cli /usr/local/bin/
chmod +x /usr/local/bin/totp-cli
```

---

### 方式 2：通过 Cargo 安装

如果您已安装 Rust 工具链：

```bash
cargo install --path .
```

安装完成后，即可在系统终端任意位置运行 `totp-cli` 命令。

---

### 方式 3：从源码编译

```bash
git clone https://github.com/agb1964/totp-cli.git
cd totp-cli
cargo build --release
```

编译生成的可执行文件位于 `./target/release/totp-cli`。

---

## 🚀 使用说明

### 查看帮助信息

```bash
totp-cli --help
```

```text
Генерация и проверка TOTP-кодов

Usage: totp-cli <COMMAND>

Commands:
  generate  Сгенерировать новый секрет и QR-код
  verify    Проверить 6-значный код
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
```

---

### 1. 生成新密钥与终端二维码

使用 `generate` 命令生成新的 Base32 密钥、标准 URI 以及终端二维码：

```bash
totp-cli generate --issuer "GitHub" --account "user@example.com"
```

**参数说明：**

- `--issuer`（可选，默认值：`totp-cli`）：服务或发行机构名称。
- `--account`（可选，默认值：`user`）：用户账户名或电子邮箱。

**输出示例：**

```text
Секрет (Base32): 64R5AC5CH464NCKPVT724WRY5BYIXZJW
URI: otpauth://totp/GitHub:user%40example.com?secret=64R5AC5CH464NCKPVT724WRY5BYIXZJW&issuer=GitHub

    █▀▀▀▀▀█ █▀█ ▀██▄▄▄▀█▄▀▀ ▄ ▀█▀▀▄██ █▀▀▀▀▀█    
    █ ███ █ ▀ ▀█ ▄▄  ▀▀█▀▀▄ █  ▄ █▀▀█ █ ███ █    
    █ ▀▀▀ █ ▀ █ ██▄▄ ▀█   █▄▄ ▀   ▀▀  █ ▀▀▀ █    
    ▀▀▀▀▀▀▀ ▀▄█ █▄█▄▀ ▀▄▀ ▀▄█▄█ █ ▀▄▀ ▀▀▀▀▀▀▀    
    ▀ ▄█▀▀▀▀█▀▀▀  █ ▀▀▄█▀   ▀▄▄▄ ▀█▄▄█  █▄▀▀▀    
    █ ▄ ▄█▀ ▄ ▄▄▀▀  █▄█ ▄▀▀█▀▀▄▀ █ ▄▀▀▀▀██  ▀    
     ▄  ▄▄▀▀▄██ ▄▀  ▀▄▀█▄ ▄▀▀  ▀  █▄▄▀ █▄ ▄▄█    
      ▄▀█▀▀█▀ ▀█▄▄██ ▄▄▀██ █▀▀▀▀▄▀  ▄▄█ ▀▀▄█▀    
    ▄█▄█▄▀▀██▄ ▀▄▄█▀ █   ▄▀ ▄  ▀█  █▀▄  ▄█ █     
    █▄█▄▀▄▀▄█ ▀▀▀ █ █▀▀██▄ █▄█▀ ▀▄█▄██▀ █ ██▀    
    ▀ ▀▄  ▀▀▄▄██▀  ▄  ▀▄█▀▄█ █ █▀▄▄█  ▀█▄▀  ▀    
    ▄▀█▄▀▀▀██ ▀█▀█ █ █ ▄█▀▀▀▄▀▄█▀▄  ▀  ▀█▄▄ ▄    
    ▀▄ ▀█▄▀ ▄▀▀█▀▄█▀▀  ▀   █ █▄▄▄▄ ▀▄██ ▄▀█ ▄    
     █▀▀▄█▀█▀▀  ▄▄ █▄  █▄▀▄ ▄  █ ▀▄▀▄   ▀█▄      
    █ ▀▄▀ ▀▀▀ ▄▄ █  ▀█▄▀█  ▄▄█   ▄▄██▄█ ▄▀  ▄    
    █████ ▀▄ ▄██ ██▄▄▄█ ▀▀▀▀▄▀▀▄█▄▀ █ ▀▄█▀█▄▀    
    ▀▀ ▀ ▀▀ █▀ ▄█▀▄▀▄█▄▄  ▄▄▄██ ▀▀▀▀█▀▀▀█▀█▀▀    
    █▀▀▀▀▀█ █▀▀▀█▄▄▀  ▄██▀ ▀██ █ █▄▀█ ▀ ██ ▀     
    █ ███ █ █ █▄ ████▀▄ ▄▀██▀█▄██▀████▀▀▀█▀██    
    █ ▀▀▀ █  ▀▄▄▄▄▄██ ▀▀▀ █   ▀█▄ ▄▄ █ ▀██▀▀█    
    ▀▀▀▀▀▀▀ ▀  ▀▀     ▀▀   ▀▀▀ ▀   ▀▀  ▀▀        
```

使用任意手机双因素身份验证器扫描终端中显示的二维码即可完成绑定。

---

### 2. 验证动态一次性验证码

使用 `verify` 命令验证用户输入的 6 位动态验证码（系统已自动计算时间容错区间）：

```bash
totp-cli verify --secret "64R5AC5CH464NCKPVT724WRY5BYIXZJW" --code "123456"
```

**参数说明：**

- `--secret`：Base32 格式的密钥。
- `--code`：手机验证器上显示的 6 位数字验证码。

**验证成功输出：**

```text
Успешно (код валиден)
```

*(退出码：`0`)*

**验证失败输出：**

```text
Ошибка (код неверен или просрочен)
```

*(退出码：`1`)*

---

## 🚦 退出状态码 (Exit Codes)

程序遵循标准的进程退出代码设计，非常适合嵌入 Bash/Zsh 脚本与自动化流水线：

| 退出码 | 含义说明 |
| :---: | :--- |
| `0` | **成功 (Success)**：TOTP 验证码正确有效，或密钥生成成功。 |
| `1` | **拒绝 (Rejected)**：验证码错误、已过期或未通过校验。 |
| `2` | **执行错误 (Execution Error)**：无效的 Base32 密钥、验证码格式错误（非 6 位数字）或系统时间获取失败。 |

### Bash 脚本集成示例

```bash
#!/usr/bin/env bash

SECRET="64R5AC5CH464NCKPVT724WRY5BYIXZJW"
read -r -p "请输入 6 位 2FA 验证码: " CODE

totp-cli verify --secret "$SECRET" --code "$CODE"
STATUS=$?

if [ $STATUS -eq 0 ]; then
    echo "✅ 身份验证成功，允许访问！"
elif [ $STATUS -eq 1 ]; then
    echo "❌ 验证码错误或已过期。"
else
    echo "⚠️ 输入参数格式有误。"
fi
```

---

## 📱 应用兼容性

生成的密钥与二维码完全兼容主流 2FA 应用程序与硬件密钥：

- 🔹 **Google Authenticator** (iOS / Android)
- 🔹 **Apple Passwords / iCloud 钥匙串** (iOS / macOS)
- 🔹 **1Password**
- 🔹 **Bitwarden / Vaultwarden**
- 🔹 **Aegis Authenticator** (Android)
- 🔹 **2FAS Authenticator**
- 🔹 **YubiKey Authenticator**
- 🔹 **Microsoft Authenticator**

---

## 🛠️ 开发与测试

您可以直接使用 [Makefile](Makefile) 快速执行日常开发任务：

```bash
make setup      # 安装所有必需工具 (clippy, rustfmt, cargo-machete, cargo-features)
make test       # 运行单元测试
make lint       # 运行 clippy 与格式检查
make check      # 运行全套检查 (test, lint, machete, features)
make build      # 编译优化版 release 二进制文件
make tag        # 根据 Cargo.toml 自动创建 Git 标签
make release    # 检查代码、创建标签并推送至 GitHub 触发发布
```

或者使用 Cargo 原生命令：

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

---

## 📂 项目文件结构

```text
totp-cli/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml               # CI 持续集成流水线
│   │   └── release.yml          # 多平台二进制打包发布流水线
│   ├── ISSUE_TEMPLATE/          # Issue 问题与功能建议模板
│   ├── dependabot.yml           # 依赖项自动更新配置
│   └── pull_request_template.md # Pull Request 模板
├── scripts/
│   └── release.sh               # 自动化发布与打标签脚本
├── src/
│   ├── config.rs                # 命令行参数解析 (clap) 与配置常量
│   ├── main.rs                  # 程序主入口
│   └── totp.rs                  # TOTP 核心逻辑、二维码生成与单元测试
├── Cargo.toml                   # 依赖配置与包元数据
├── CHANGELOG.md                 # 版本变更记录
├── CONTRIBUTING.md              # 贡献指南
├── LICENSE                      # MIT 开源许可证
├── Makefile                     # 自动化构建与发布脚本
├── README.md                    # 项目文档 (English)
├── README.ru.md                 # 项目文档 (Русский)
└── README.zh.md                 # 项目文档 (简体中文)
```

---

## 📄 开源协议

本项目基于 [MIT](LICENSE) 开源协议分发与使用。
