# totp-cli

<div align="center">

[![GitHub Release](https://img.shields.io/github/v/release/agb1964/totp-cli?color=blue&logo=github)](https://github.com/agb1964/totp-cli/releases/latest)
[![GitHub Downloads](https://img.shields.io/github/downloads/agb1964/totp-cli/total?color=blue&logo=github)](https://github.com/agb1964/totp-cli/releases)
[![CI](https://github.com/agb1964/totp-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/agb1964/totp-cli/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org)
[![RFC 6238](https://img.shields.io/badge/RFC-6238%20Compliant-success.svg)](https://datatracker.ietf.org/doc/html/rfc6238)

**A fast and lightweight command-line tool for TOTP (Time-based One-Time Password) generation and verification for 2FA/MFA with terminal QR code display.**

<br />

[**English**](README.md) • [Русский](README.ru.md) • [简体中文](README.zh.md)

<br />

[Features](#-features) •
[Installation](#-installation) •
[Usage](#-usage) •
[Exit Codes](#-exit-codes) •
[Compatibility](#-compatibility) •
[Project Status](#-project-status) •
[Development](#-development) •
[License](#-license)

</div>

---

> [!NOTE]
>
> **📌 Project Status**
>
> This project was created to solve a specific practical task for the author, and its current feature set completely fulfills that purpose. Therefore, active future development, maintenance, and feature expansion by the author are not planned.
>
> The project is fully functional, tested, and ready for use. If you need additional features, customization, or extensions, **forks are warmly welcomed**! You are completely free to fork this repository and adapt it under the terms of the MIT License.

---

## ✨ Features

- 🔑 **Base32 Secret Generation**: Generates cryptographically secure 160-bit (20-byte) secrets in standard Base32 encoding.
- 📱 **Terminal QR Code Display**: Instantly renders compact Unicode QR codes directly in your console for quick mobile scanning.
- 🔗 **Standard otpauth URI**: Full support for `otpauth://totp/{Issuer}:{Account}?secret=...&issuer={Issuer}` format.
- ⏱️ **Strict RFC 6238 Compliance**:
  - Algorithm: **HMAC-SHA1**
  - Code length: **6 digits**
  - Time step: **30 seconds**
  - Skew / Drift tolerance: **±1 time step** (±30s to compensate for clock drift).
- 🤖 **Script & CI/CD Friendly**: Predictable, standard process exit codes for shell automation.
- ⚡ **Zero Overhead**: Clean, high-performance Rust binary with no bloat.

---

## 📦 Installation

### Option 1: Download Pre-built Binary (Recommended)

Pre-compiled standalone binaries for Linux, macOS, and Windows are available on the [**Releases**](https://github.com/agb1964/totp-cli/releases) page:

| Platform | Architecture | Archive |
| --- | --- | --- |
| 🐧 **Linux** | x86_64 | `totp-cli-linux-x86_64.tar.gz` |
| 🍏 **macOS** | Apple Silicon (arm64) | `totp-cli-macos-arm64.tar.gz` |
| 🪟 **Windows** | x86_64 | `totp-cli-windows-x86_64.zip` |

#### Quick Install via Terminal (Linux / macOS)

```bash
# 1. Download the archive for your system (e.g. Linux x86_64):
curl -LO https://github.com/agb1964/totp-cli/releases/latest/download/totp-cli-linux-x86_64.tar.gz

# For macOS Apple Silicon (M1/M2/M3/M4):
# curl -LO https://github.com/agb1964/totp-cli/releases/latest/download/totp-cli-macos-arm64.tar.gz

# 2. Extract the archive
tar -xzf totp-cli-*.tar.gz

# 3. Move binary into your PATH
sudo mv totp-cli /usr/local/bin/
chmod +x /usr/local/bin/totp-cli
```

---

### Option 2: Install via Cargo

```bash
cargo install --path .
```

After installation, `totp-cli` will be accessible globally from your terminal.

---

### Option 3: Build from Source

```bash
git clone https://github.com/agb1964/totp-cli.git
cd totp-cli
cargo build --release
```

The compiled binary will be located at `./target/release/totp-cli`.

---

## 🚀 Usage

### Help

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

### 1. Generate a new secret and QR code

The `generate` command creates a new Base32 secret, forms the authenticator URI, and renders a QR code in your terminal:

```bash
totp-cli generate --issuer "GitHub" --account "user@example.com"
```

**Options:**

- `--issuer` (optional, default: `totp-cli`): Service or company name.
- `--account` (optional, default: `user`): User account identifier or email.

**Example Output:**

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

Scan the QR code with your mobile authenticator app.

---

### 2. Verify a one-time code

The `verify` command validates a 6-digit TOTP token against a Base32 secret, accounting for time drift:

```bash
totp-cli verify --secret "64R5AC5CH464NCKPVT724WRY5BYIXZJW" --code "123456"
```

**Parameters:**

- `--secret`: Secret key in Base32 format.
- `--code`: 6-digit code from the authenticator app.

**Success output:**

```text
Успешно (код валиден)
```

*(exit code: `0`)*

**Failure output:**

```text
Ошибка (код неверен или просрочен)
```

*(exit code: `1`)*

---

## 🚦 Exit Codes

Standard exit codes for straightforward shell scripting and automation:

| Exit Code | Meaning |
| :---: | :--- |
| `0` | **Success**: TOTP code is valid and active (or secret generated successfully). |
| `1` | **Failure / Denied**: Code is incorrect, expired, or invalid. |
| `2` | **Input / Execution Error**: Invalid Base32 secret, bad code format (not 6 digits), or system clock failure. |

### Bash script example

```bash
#!/usr/bin/env bash

SECRET="64R5AC5CH464NCKPVT724WRY5BYIXZJW"
read -r -p "Enter 6-digit 2FA code: " CODE

totp-cli verify --secret "$SECRET" --code "$CODE"
STATUS=$?

if [ $STATUS -eq 0 ]; then
    echo "✅ Access granted!"
elif [ $STATUS -eq 1 ]; then
    echo "❌ Invalid or expired code."
else
    echo "⚠️ Parameter error."
fi
```

---

## 📱 Compatibility

Fully compatible with all standard 2FA/MFA applications and hardware tokens:

- 🔹 **Google Authenticator** (iOS / Android)
- 🔹 **Apple Passwords / iCloud Keychain** (iOS / macOS)
- 🔹 **1Password**
- 🔹 **Bitwarden / Vaultwarden**
- 🔹 **Aegis Authenticator** (Android)
- 🔹 **2FAS Authenticator**
- 🔹 **YubiKey Authenticator**
- 🔹 **Microsoft Authenticator**

---

## 🛠️ Development

You can use the provided [Makefile](Makefile) for development workflows:

```bash
make setup      # Install required tools (clippy, rustfmt, cargo-machete, cargo-features)
make test       # Run unit tests
make lint       # Run clippy and rustfmt check
make check      # Run all checks (test, lint, machete, features)
make build      # Build optimized release binary
make tag        # Create Git tag from Cargo.toml version
make release    # Run checks, create tag, and push to GitHub
```

Or standard Cargo commands:

```bash
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

---

## 📂 Project Structure

```text
totp-cli/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml               # CI testing matrix
│   │   └── release.yml          # Multi-platform release builds
│   ├── ISSUE_TEMPLATE/          # Issue templates
│   ├── dependabot.yml           # Automated dependency updates
│   └── pull_request_template.md # PR template
├── scripts/
│   └── release.sh               # Release automation script
├── src/
│   ├── config.rs                # CLI arguments (clap) and constants
│   ├── main.rs                  # Entry point
│   └── totp.rs                  # TOTP logic, QR rendering, unit tests
├── Cargo.toml                   # Manifest and package metadata
├── CHANGELOG.md                 # Changelog
├── CONTRIBUTING.md              # Contributing guidelines
├── LICENSE                      # MIT License
├── Makefile                     # Build, test, and release tasks
├── README.md                    # Documentation (English)
├── README.ru.md                 # Documentation (Русский)
└── README.zh.md                 # Documentation (简体中文)
```

---

## 📄 License

This project is licensed under the [MIT License](LICENSE).
