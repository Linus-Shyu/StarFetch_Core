# Release Workflow 配置说明

推送 `v*` tag 后，自动完成：**打包全平台/全架构 → 发布到 GitHub Release → 更新 APT / Homebrew / Winget**。

## 流程概述

1. **Build**：只跑 **3 个 job**（不再用 24 个 matrix job），避免账号并发限制导致“只跑 2 个”：
   - **build-linux**（ubuntu）：本机 x86_64 + cargo deb，其余用 cross 串行打（Linux/BSD/Windows-GNU 共十几项），产出 `.deb`、`.tar.gz`、`.zip`。
   - **build-macos**（macos）：x86_64-apple-darwin、aarch64-apple-darwin。
   - **build-windows**（windows）：x86_64/aarch64/i686-pc-windows-msvc。
2. **GitHub Release**：合并 3 个 artifact 后，将所有产物上传到当前 tag 的 Release 页面。
3. **APT**：将 `.deb` 推送到 Cloudsmith（需 `CLOUDSMITH_API_KEY`）。
4. **Homebrew**：更新 `Linus-Shyu/homebrew-tap`（需 `HOMEBREW_TAP_TOKEN`）。
5. **Winget**：推送到你的 `winget-pkgs` fork（需 `WINGET_PKGS_TOKEN`），再向官方提 PR。

## 功能特性

✅ **全平台构建** (共 24 个平台)

**Linux (GNU libc)** - 8 个平台
- x86_64-unknown-linux-gnu - 生成 `.deb` + `.tar.gz`
- aarch64-unknown-linux-gnu - 生成 `.deb` + `.tar.gz`
- armv7-unknown-linux-gnueabihf - 生成 `.tar.gz`
- i686-unknown-linux-gnu - 生成 `.tar.gz`
- riscv64gc-unknown-linux-gnu - 生成 `.tar.gz`
- powerpc64le-unknown-linux-gnu - 生成 `.tar.gz`
- s390x-unknown-linux-gnu - 生成 `.tar.gz`
- mips64el-unknown-linux-gnuabi64 - 生成 `.tar.gz`

**Linux (musl - Alpine)** - 3 个平台
- x86_64-unknown-linux-musl - 生成 `.tar.gz`
- aarch64-unknown-linux-musl - 生成 `.tar.gz`
- armv7-unknown-linux-musleabihf - 生成 `.tar.gz`

**macOS** - 2 个平台
- x86_64-apple-darwin - 生成 `.tar.gz`
- aarch64-apple-darwin (Apple Silicon) - 生成 `.tar.gz`

**Windows (MSVC)** - 3 个平台
- x86_64-pc-windows-msvc - 生成 `.zip`
- aarch64-pc-windows-msvc - 生成 `.zip`
- i686-pc-windows-msvc - 生成 `.zip`

**Windows (GNU)** - 2 个平台
- x86_64-pc-windows-gnu - 生成 `.zip`
- i686-pc-windows-gnu - 生成 `.zip`

**FreeBSD** - 2 个平台
- x86_64-unknown-freebsd - 生成 `.tar.gz`
- aarch64-unknown-freebsd - 生成 `.tar.gz`

**OpenBSD** - 2 个平台
- x86_64-unknown-openbsd - 生成 `.tar.gz`
- aarch64-unknown-openbsd - 生成 `.tar.gz`

**NetBSD** - 2 个平台
- x86_64-unknown-netbsd - 生成 `.tar.gz`
- aarch64-unknown-netbsd - 生成 `.tar.gz`

✅ **通用安装脚本**

项目根目录提供跨平台安装脚本：
- `install.sh`（Linux/macOS/BSD）
- `install.ps1`（Windows）

脚本会根据 `uname`/系统架构自动选择对应的 release 资产，并在找不到预编译包时回退到源码编译（需要 Rust + Git）。

✅ **自动发布**
- GitHub Release
- Cloudsmith (APT 仓库)
- Homebrew Formula
- Winget Manifest

## 必需的 Secrets 配置

在 GitHub 仓库的 Settings > Secrets and variables > Actions 中添加以下 secrets：

### 1. CLOUDSMITH_API_KEY (已有)
用于推送到 Cloudsmith APT 仓库。

### 2. HOMEBREW_TAP_TOKEN (新增)
用于更新 Homebrew Formula。

**创建步骤：**
1. 前往 https://github.com/settings/tokens
2. 创建 Personal Access Token (Classic)
3. 权限：`repo` (完整仓库访问权限)
4. 将 token 添加到仓库 secrets 中，命名为 `HOMEBREW_TAP_TOKEN`

**注意：** 确保你的 homebrew tap 仓库 (`Linus-Shyu/homebrew-tap`) 存在，并且该 token 有权限推送。

### 3. WINGET_PKGS_TOKEN
用于向你的 **winget-pkgs fork** 推送 manifest。

**步骤：**
1. Fork [microsoft/winget-pkgs](https://github.com/microsoft/winget-pkgs) 到你的账户（如 `Linus-Shyu/winget-pkgs`）。
2. 创建有 `repo` 权限的 Personal Access Token，写入仓库 secret `WINGET_PKGS_TOKEN`。
3. 每次发版后 workflow 会推送到你的 fork；你再从 fork 向 `microsoft/winget-pkgs` 提 PR，合并后用户即可 `winget install Linus-Shyu.StarFetch`。

## Homebrew Formula 仓库设置

1. 创建或确认 `Linus-Shyu/homebrew-tap` 仓库存在
2. 如果不存在 Formula 文件，workflow 会自动创建
3. 如果已存在，workflow 会自动更新版本和 SHA256

## Winget Manifest 设置

1. Fork `microsoft/winget-pkgs` 仓库
2. 或者创建自己的 winget 仓库
3. 如果使用 microsoft/winget-pkgs，workflow 会推送到你的 fork，然后你需要创建 PR

## 使用方法

1. 确保所有 secrets 已配置
2. 创建并推送 tag：
   ```bash
   git tag v0.2.3
   git push origin v0.2.3
   ```
3. Workflow 会自动：
   - 构建所有平台的二进制文件
   - 发布到 GitHub Release
   - 推送到 Cloudsmith
   - 更新 Homebrew Formula
   - 更新 Winget Manifest

## 故障排除

### Homebrew 更新失败
- 检查 `HOMEBREW_TAP_TOKEN` 是否正确配置
- 确认 homebrew tap 仓库存在且有写入权限
- 检查 Formula 文件格式是否正确

### Winget 更新失败
- 检查 `WINGET_PKGS_TOKEN` 是否正确配置
- 确认已 fork `microsoft/winget-pkgs` 仓库
- 检查 manifest 文件格式是否符合 winget 规范

### Cloudsmith 更新失败
- 检查 `CLOUDSMITH_API_KEY` 是否正确配置
- 确认 Cloudsmith 仓库路径正确

## 注意事项

- 所有更新步骤都有 `|| true` 或 `|| exit 0`，确保单个步骤失败不会影响整体流程
- 如果某个包管理器更新失败，其他步骤仍会继续执行
- 建议在首次使用前手动测试各个步骤
