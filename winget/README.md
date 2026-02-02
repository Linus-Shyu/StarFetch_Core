# WinGet Manifests

Manifest 使用 1.10.0 规范（winget-pkgs 校验要求），已按目录结构存放：`manifests/l/Linus-Shyu/StarFetch/<version>/`。

## 以后发新版本（不用再在命令行里逐项填）

**方式一：用 komac update（推荐）**

第一个版本在 winget-pkgs 合并后，以后只需：

```bash
komac update Linus-Shyu.StarFetch --version 0.2.4 --urls "https://github.com/Linus-Shyu/StarFetch_Core/releases/download/v0.2.4/starfetch-x86_64-pc-windows-msvc.zip" --submit
```

Komac 会以已有包为基础，只更新版本和 URL/SHA，不用再填一遍 metadata。

**方式二：用仓库里的 Publish to WinGet 工作流**

发新 Release 后，到 Actions → Publish to WinGet → Run workflow，填 release tag（如 `v0.2.4`），由 winget-releaser（Komac）自动提 PR。

**方式三：复制本目录再改**

复制 `0.2.3/` 为 `<新版本>/`，在三个 yaml 里把 `0.2.3`、InstallerUrl、InstallerSha256、ReleaseDate 等改成新版本，再用 `winget validate` 校验后提交到你的 winget-pkgs fork 并开 PR。
