# CCometixLine-88CC

[English](README.md) | [中文](README.zh.md)

A Claude Code status line companion written in Rust, optimized for Termux and Android users who rely on 88Code. This fork is maintained by [gary-50](https://github.com/gary-50) and ships a set of pre-built npm packages so the status line can be installed on **any** platform with a single command.

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
[![CI](https://github.com/gary-50/CCometixLine_termux/actions/workflows/ci.yml/badge.svg)](https://github.com/gary-50/CCometixLine_termux/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@gary-50/ccline-88cc.svg?color=cb0000&label=npm)](https://www.npmjs.com/package/@gary-50/ccline-88cc)

> ### Project origins
>
> - The original CCometixLine was created by [Haleclipse](https://github.com/Haleclipse/CCometixLine) under the MIT license.
> - [HoBeedzc](https://github.com/HoBeedzc) built the first 88Code adaptation.
> - This repository continues the 88Code/Ter mux friendly work and adds npm distribution, Termux fixes, and continuous release automation. Credits for upstream code and ideas remain intact in `LICENSE`.

---

## Highlights

- **Real-time Claude Code status line** displaying current model, workspace path, git status, context tokens and remaining 88Code quota.
- **Cross-platform binaries** for Linux (glibc + musl), macOS (Intel + Apple Silicon), Windows, and Linux ARM64/Termux—all published on npm as optional deps.
- **Interactive TUI** configuration (`ccline-88cc -c`) with live preview, theme presets, and per-segment toggles.
- **Claude Code enhancer** that patches local Claude installs to remove “Context low” warnings, enable verbose output, and back up settings.
- **Stable networking** layer with retry, caching, and offline tolerances tailored to 88Code proxy behaviour.

## Table of contents

1. [Installation](#installation)
2. [Configure Claude Code](#configure-claude-code)
3. [Manual & offline installs](#manual--offline-installs)
4. [Usage tips](#usage-tips)
5. [Platform packages on npm](#platform-packages-on-npm)
6. [Development](#development)
7. [Release workflow](#release-workflow)
8. [FAQ](#faq)

## Installation

### One command via npm (recommended)

```bash
# Global install
npm install -g @gary-50/ccline-88cc

# Or yarn
yarn global add @gary-50/ccline-88cc

# Or pnpm
pnpm add -g @gary-50/ccline-88cc
```

For users in China:

```bash
npm install -g @gary-50/ccline-88cc --registry https://registry.npmmirror.com
```

After installation:

- Run `ccline-88cc` to print the status line immediately.
- Run `ccline-88cc -c` for the interactive theme/configurator.
- Termux users receive the static ARM64 build automatically (postinstall detects Termux and guides you through copying the binary to `~/.claude/ccline`).

> **Need 88Code credentials?** Configure them once inside the TUI or edit `~/.claude/ccline/config.toml`.

### Keeping up to date

```bash
npm update -g @gary-50/ccline-88cc
```

## Configure Claude Code

Add the status line command into `settings.json`. The safest method is to point Claude to the binary placed in `~/.claude/ccline`:

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/ccline/ccline-88cc",
    "padding": 0
  }
}
```

If the npm global install directory is already on `PATH`, this shortcut works:

```json
{
  "statusLine": {
    "type": "command",
    "command": "ccline-88cc",
    "padding": 0
  }
}
```

Windows users can point to `%USERPROFILE%\.claude\ccline\ccline-88cc.exe`.

## Manual & offline installs

Download binaries from the [Releases](https://github.com/gary-50/CCometixLine_termux/releases) page:

| Platform | Archive | Notes |
|----------|---------|-------|
| Linux x64 (glibc) | `ccline-88cc-linux-x64.tar.gz` | Ubuntu 22.04+, Debian 11+, RHEL 9+ |
| Linux x64 (musl)  | `ccline-88cc-linux-x64-static.tar.gz` | Works on Alpine / any distro |
| Linux ARM64 | `ccline-88cc-linux-arm64.tar.gz` | Termux-friendly static build |
| macOS Intel | `ccline-88cc-macos-x64.tar.gz` | Requires macOS 12+ |
| macOS Apple Silicon | `ccline-88cc-macos-arm64.tar.gz` | Native arm64 build |
| Windows x64 | `ccline-88cc-windows-x64.zip` | Extract `ccline-88cc.exe` |

Each archive contains a single binary. Copy it to `~/.claude/ccline/` (or `%USERPROFILE%\.claude\ccline\` on Windows) and ensure it is executable.

## Usage tips

- `ccline-88cc` with no arguments prints the status line once; run it continuously to keep the panel current.
- `ccline-88cc -c` opens the TUI to tweak themes, toggle segments, and update API tokens.
- The Quota monitor retries failed requests up to three times. When offline, the last cached quota is shown.
- Logs and config live under `~/.claude/ccline/`.

## Platform packages on npm

The main npm package (`@gary-50/ccline-88cc`) depends on optional binaries:

| npm package | Platform |
|-------------|----------|
| `@gary-50/ccline-88cc-darwin-x64` | macOS Intel |
| `@gary-50/ccline-88cc-darwin-arm64` | macOS Apple Silicon |
| `@gary-50/ccline-88cc-linux-x64` | Linux glibc |
| `@gary-50/ccline-88cc-linux-x64-musl` | Linux musl/static |
| `@gary-50/ccline-88cc-linux-arm64` | Linux ARM64 & Termux |
| `@gary-50/ccline-88cc-win32-x64` | Windows x64 |

Installing the main package automatically downloads the correct binary for your platform.

## Development

```bash
git clone https://github.com/gary-50/CCometixLine_termux.git
cd CCometixLine_termux
cargo build
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

CI builds the following matrices:

- Unit tests + lint on Ubuntu.
- Nightly binaries for Linux (glibc/musl/arm64), Windows, and both macOS architectures.

## Release workflow

1. Update versions in `Cargo.toml` and every npm `package.json`.
2. Follow `RELEASING.md` or run `npm/scripts/prepare-packages.js <version>`.
3. Tag `vX.Y.Z` and push. GitHub Actions builds artifacts.
4. Publish each npm binary package first, then the main npm package (automated in CI once `NODE_AUTH_TOKEN` is configured, or run locally as we did for v1.0.8).

## FAQ

**Is this affiliated with Anthropic or 88Code?**  
No. This is a community-maintained adaptation and ships under the same MIT license.

**Can I use it without npm?**  
Yes—download the binary from Releases or build from source with Cargo.

**How do I prevent GitHub Actions from running on documentation-only commits?**  
Add `[skip ci]` to the commit message when pushing README edits.

## License & acknowledgements

- Licensed under [MIT](LICENSE).
- Credits to [Haleclipse](https://github.com/Haleclipse/CCometixLine) and [HoBeedzc](https://github.com/HoBeedzc) for the original codebase.
- Extra thanks to the 88Code community for testing the Termux builds.

## Star history

[![Star History Chart](https://api.star-history.com/svg?repos=gary-50/CCometixLine_termux&type=Date)](https://star-history.com/#gary-50/CCometixLine_termux&Date)
