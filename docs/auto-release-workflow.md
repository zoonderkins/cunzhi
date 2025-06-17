# PR 合并触发发布 + 手动版本选择

这个工作流程让你在 PR 合并后能够手动选择版本号，然后自动完成发布。

## 🚀 工作流程概述

1. **PR 合并触发** → 计算建议版本号，显示发布信息
2. **手动选择版本** → 在 GitHub Actions 中选择版本类型
3. **自动发布** → 更新版本文件，创建 tag，触发构建

## 📋 无需额外设置

新的工作流程不需要创建 GitHub Environment，直接可以使用！

## 🔄 使用流程

### 第一步：创建和合并 PR

1. 创建 PR 并添加 `release` 标签
2. 合并 PR 到 main 分支
3. GitHub Actions 自动运行 `prepare-release` 阶段

### 第二步：查看发布信息

1. 打开 GitHub Actions 页面
2. 找到刚运行的 "Auto Release (PR)" 工作流程
3. 查看 Summary 中的发布信息：
   - 当前版本
   - 建议版本（基于 PR 标签自动计算）
   - PR 标题

### 第三步：手动选择版本并发布

1. 在 Actions 页面点击 "Auto Release (PR)" 工作流程
2. 点击 **Run workflow** 按钮
3. 选择版本类型：
   - **patch**: 修复 bug (如 1.0.0 → 1.0.1)
   - **minor**: 新功能 (如 1.0.0 → 1.1.0)
   - **major**: 破坏性更改 (如 1.0.0 → 2.0.0)
   - **custom**: 自定义版本号
4. 如果选择 custom，输入具体版本号（如 1.2.3）
5. 点击 **Run workflow** 完成发布

### 第四步：自动完成发布

1. 版本文件自动更新（package.json, Cargo.toml 等）
2. Git tag 自动创建
3. Release 工作流程自动触发
4. 二进制文件构建和发布
5. Homebrew formula 自动更新

## 📊 版本类型规则

### PR 标签自动判断（第一阶段建议）

- **major** 或 **breaking** 标签 → 主版本号 +1
- **minor** 或 **feature** 标签 → 次版本号 +1  
- **其他情况** → 修订版本号 +1

### 手动选择（第二阶段确认）

你可以覆盖自动建议，选择任何版本类型或自定义版本号。

## ⚠️ 注意事项

1. **必须添加 release 标签**：PR 必须有 `release` 标签才会触发
2. **权限要求**：确保 GitHub Actions 有 `contents: write` 权限
3. **Personal Access Token**：确保 `PERSONAL_ACCESS_TOKEN` secret 已配置
4. **两阶段流程**：第一阶段只是准备，第二阶段才真正发布

## 🔍 故障排除

### 第一阶段没有触发
- 检查 PR 是否有 `release` 标签
- 检查 PR 是否合并到 `main` 分支

### 找不到 Run workflow 按钮
- 确保在 "Auto Release (PR)" 工作流程页面
- 确保有 Actions 权限

### 推送失败
- 检查 `PERSONAL_ACCESS_TOKEN` 是否有效
- 检查 token 是否有 `repo` 权限

## 🎯 优势

- ✅ **灵活性**：可以覆盖自动建议的版本号
- ✅ **安全性**：手动确认，避免错误发布
- ✅ **简单性**：无需复杂的环境设置
- ✅ **可视性**：清晰的发布信息展示
- ✅ **自动化**：选择版本后全自动完成
