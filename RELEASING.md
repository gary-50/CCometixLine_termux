# 发布流程（GitHub Actions + npm）

1. **准备版本号**
   - 在 `Cargo.toml` 和 `npm` 目录下的所有 `package.json` 中写入要发布的版本（本仓库已同步为 `1.0.8`）。
   - 确认 `README`、`README.zh.md` 以及 `npm/main/README.md` 中提到的安装命令无误。

2. **确认凭证**
   - GitHub 仓库的 `Settings → Secrets → Actions` 中配置 `NPM_TOKEN`（已经完成则跳过）。
   - 本地终端执行过一次 `npm login`，确保可以发布到自己的 `@gary-50` scope。

3. **提交并推送**
   ```bash
   git add .
   git commit -m "chore: prepare release 1.0.8"
   git push origin master
   ```

4. **创建发布标签**
   ```bash
   git tag v1.0.8
   git push origin v1.0.8
   ```
   推送标签会触发 `.github/workflows/release.yml`，由CI自动：
   - 构建各平台二进制
   - 运行 `npm/scripts/prepare-packages.js`
   - 依次发布 `@gary-50/ccline-88cc-<platform>` 与主包 `@gary-50/ccline-88cc`

5. **验证**
   - 关注 GitHub Actions “Release” workflow 运行结果，确认所有 `npm publish` 步骤成功。
   - 在自己机器上执行 `npm install -g @gary-50/ccline-88cc`，验证安装无误。
   - 如需在 README 或 Release Notes 中记录变更，更新后重新推送（无须重新打标签，除非需要重新发布）。

> 如果需要发布新版本，重复上述流程并将版本号替换为新的 semver（例如 `1.0.9`）。
