# CCometixLine-88CC

[English](README.md) | [中文](README.zh.md)

一个基于 Rust 的 Claude Code 状态栏工具，专为 88Code 与 Termux/Android 用户优化。当前版本由 [gary-50](https://github.com/gary-50) 维护，提供 npm 分发和多平台静态二进制，方便在任何环境一键安装。

![Language:Rust](https://img.shields.io/static/v1?label=Language&message=Rust&color=orange&style=flat-square)
![License:MIT](https://img.shields.io/static/v1?label=License&message=MIT&color=blue&style=flat-square)
[![CI](https://github.com/gary-50/CCometixLine_termux/actions/workflows/ci.yml/badge.svg)](https://github.com/gary-50/CCometixLine_termux/actions/workflows/ci.yml)
[![npm](https://img.shields.io/npm/v/@gary-50/ccline-88cc.svg?color=cb0000&label=npm)](https://www.npmjs.com/package/@gary-50/ccline-88cc)

> ### 项目来源
>
> - 原始 CCometixLine 由 [Haleclipse](https://github.com/Haleclipse/CCometixLine) 在 MIT 许可下发布。
> - [HoBeedzc](https://github.com/HoBeedzc) 首次为 88Code 适配。
> - 本仓库在其基础上继续开发，增加 npm 发布、Termux 特性的修复与自动化脚本，完整保留原作者授权信息。

---

## 核心亮点

- **实时 Claude Code 状态栏**：模型、仓库目录、Git 状态、上下文 token 与 88Code 余额一目了然。
- **跨平台二进制**：Linux (glibc/musl)、macOS (Intel/Apple Silicon)、Windows、Termux ARM64，一次构建全部发布。
- **交互式 TUI**：`ccline-88cc -c` 实时预览主题、分段开关、API Key 设置。
- **Claude 增强脚本**：自动备份、移除“Context low”警告、开启详细模式。
- **稳健网络层**：为 88Code 代理场景设计的缓存 + 重试，离线也能显示最近余额。

## 目录

1. [安装方式](#安装方式)
2. [接入 Claude Code](#接入-claude-code)
3. [离线/手动安装](#离线手动安装)
4. [使用提示](#使用提示)
5. [npm 平台包](#npm-平台包)
6. [开发与贡献](#开发与贡献)
7. [发布流程](#发布流程)
8. [常见问题](#常见问题)

## 安装方式

### npm 一键安装（推荐）

```bash
npm install -g @gary-50/ccline-88cc
# 或 yarn / pnpm
yarn global add @gary-50/ccline-88cc
pnpm add -g @gary-50/ccline-88cc
```

中国大陆可使用镜像：

```bash
npm install -g @gary-50/ccline-88cc --registry https://registry.npmmirror.com
```

安装完成后：

- 直接运行 `ccline-88cc` 查看状态栏。
- 执行 `ccline-88cc -c` 进入配置面板设置主题、API Key。
- Termux 自动识别后会提示将二进制复制到 `~/.claude/ccline`。

升级命令：

```bash
npm update -g @gary-50/ccline-88cc
```

## 接入 Claude Code

在 `settings.json` 中添加：

```json
{
  "statusLine": {
    "type": "command",
    "command": "~/.claude/ccline/ccline-88cc",
    "padding": 0
  }
}
```

若 npm 全局目录已加入 `PATH`，也可直接：

```json
{
  "statusLine": {
    "type": "command",
    "command": "ccline-88cc",
    "padding": 0
  }
}
```

Windows 用户指向 `%USERPROFILE%\.claude\ccline\ccline-88cc.exe` 即可。

## 离线/手动安装

从 [Releases](https://github.com/gary-50/CCometixLine_termux/releases) 下载对应档案，解压后将可执行文件放入 `~/.claude/ccline`（Windows 放到 `%USERPROFILE%\.claude\ccline`）。

| 平台 | 文件名 | 说明 |
|------|--------|------|
| Linux x64 (glibc) | `ccline-88cc-linux-x64.tar.gz` | Ubuntu 22.04+ 等 |
| Linux x64 (musl) | `ccline-88cc-linux-x64-static.tar.gz` | 任意发行版 |
| Linux ARM64 / Termux | `ccline-88cc-linux-arm64.tar.gz` | 静态 musl |
| macOS Intel | `ccline-88cc-macos-x64.tar.gz` | macOS 12+ |
| macOS Apple Silicon | `ccline-88cc-macos-arm64.tar.gz` | 原生 arm64 |
| Windows x64 | `ccline-88cc-windows-x64.zip` | 解压后获取 `ccline-88cc.exe` |

## 使用提示

- 无参数运行输出一次状态栏；搭配 Claude Code 会定期刷新。
- `-c` 打开配置 TUI，可切换主题、调整段落、设置 88Code Token。
- 网络失败会自动重试三次，离线时显示上一次缓存的余额。
- 所有配置存放在 `~/.claude/ccline/`。

## npm 平台包

主包 `@gary-50/ccline-88cc` 会根据平台自动安装下列可选依赖：

| npm 包名 | 平台 |
|----------|------|
| `@gary-50/ccline-88cc-darwin-x64` | macOS Intel |
| `@gary-50/ccline-88cc-darwin-arm64` | macOS Apple Silicon |
| `@gary-50/ccline-88cc-linux-x64` | Linux glibc |
| `@gary-50/ccline-88cc-linux-x64-musl` | Linux musl/static |
| `@gary-50/ccline-88cc-linux-arm64` | Linux ARM64/Termux |
| `@gary-50/ccline-88cc-win32-x64` | Windows x64 |

## 开发与贡献

```bash
git clone https://github.com/gary-50/CCometixLine_termux.git
cd CCometixLine_termux
cargo build
cargo test
cargo fmt -- --check
cargo clippy -- -D warnings
```

CI 会运行测试、lint，并构建所有平台的 nightly 版本。欢迎提交 Issue / PR（提交说明中可加入 `[skip ci]` 避免纯文档变更触发 CI）。

## 发布流程

1. 在 `Cargo.toml` 与各 `npm/package.json` 中更新版本号。
2. 运行 `node npm/scripts/prepare-packages.js <版本>`。
3. `git tag vX.Y.Z && git push origin vX.Y.Z` 触发 Release workflow。
4. 配置好 `NPM_TOKEN` 后，CI 会自动发布所有平台 npm 包；或参考 `RELEASING.md` 手动发布。

## 常见问题

- **是否与 Anthropic/88Code 官方有关？** 否，本项目为社区自发适配。
- **可以不用 npm 吗？** 可以，通过 Release 下载或 `cargo build --release`。
- **Termux 需要 root 吗？** 不需要，所有文件存放在用户目录。

## 许可与致谢

- 项目遵循 [MIT 许可证](LICENSE)。
- 感谢 Haleclipse 与 HoBeedzc 的原始工作，以及 88Code 社区的反馈。

## Star 统计

[![Star History Chart](https://api.star-history.com/svg?repos=gary-50/CCometixLine_termux&type=Date)](https://star-history.com/#gary-50/CCometixLine_termux&Date)
