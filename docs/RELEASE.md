# 发版指南

寸止项目支持多种发版方式，您可以根据需要选择最适合的方式。

## 🚀 发版方式

### 方式一：本地脚本发版（传统方式）

**适用场景：**
- 快速发版
- 完全控制发版流程
- 本地环境可用

**使用方法：**
```bash
./scripts/release.sh
# 选择 "1) 本地发版 (当前方式)"
```

**流程：**
1. 选择版本类型（patch/minor/major/custom）
2. 脚本自动更新版本号文件
3. 创建 commit 和 tag
4. 推送到 GitHub
5. 触发 GitHub Actions 构建

### 方式二：GitHub Actions 手动发版（推荐）

**适用场景：**
- 更安全的发版环境
- 无需本地 Git 权限
- 标准化的发版流程

**使用方法：**
```bash
./scripts/release.sh
# 选择 "2) GitHub Actions 发版 (推荐)"
```

**流程：**
1. 脚本引导您打开 GitHub Actions 页面
2. 在网页上填写发版参数：
   - **版本类型**：选择 patch/minor/major/custom
   - **自定义版本号**：仅当选择 custom 时填写（格式：x.y.z）
3. GitHub Actions 自动执行发版
4. 自动构建和发布

**版本号设置示例：**
- 选择 `patch`：1.0.0 → 1.0.1（自动计算）
- 选择 `minor`：1.0.0 → 1.1.0（自动计算）
- 选择 `major`：1.0.0 → 2.0.0（自动计算）
- 选择 `custom` + 输入 `1.5.3`：→ 1.5.3（手动指定）

**GitHub Actions 界面操作：**
```
GitHub Actions 页面
├── [Run workflow ▼] 按钮
└── 弹出表单：
    ├── 版本类型: [patch ▼]  ← 选择版本类型
    │   ├── patch (修复)
    │   ├── minor (功能)
    │   ├── major (重大更改)
    │   └── custom (自定义)
    ├── 自定义版本号: [     ]  ← 仅 custom 时填写
    └── [Run workflow] 按钮  ← 点击开始发版
```

### 方式三：PR 自动发版

**适用场景：**
- 团队协作
- 代码审查后发版
- 自动化程度最高

**使用方法：**
1. 创建 PR 到 main 分支
2. 添加 `release` 标签
3. 可选：添加版本类型标签（`major`、`minor`、`patch`）
4. 合并 PR 后自动发版

## 📋 版本类型说明

| 类型 | 说明 | 示例 |
|------|------|------|
| **patch** | 修复 bug，向后兼容 | 1.0.0 → 1.0.1 |
| **minor** | 新功能，向后兼容 | 1.0.0 → 1.1.0 |
| **major** | 重大更改，可能不兼容 | 1.0.0 → 2.0.0 |
| **custom** | 自定义版本号 | 1.0.0 → 1.5.3 |

## 🔧 配置说明

### GitHub Actions 权限

确保仓库设置中启用了以下权限：
- `contents: write` - 创建 tag 和 release
- `pull-requests: write` - PR 自动发版

### 标签说明

PR 自动发版支持的标签：
- `release` - 必需，标记为发版 PR
- `major` 或 `breaking` - 主版本号升级
- `minor` 或 `feature` - 次版本号升级
- 无特殊标签 - 默认为 patch 版本

## 🎯 最佳实践建议

### 小团队/个人项目
- 使用 **GitHub Actions 手动发版**
- 简单、安全、无需本地环境

### 团队协作项目
- 使用 **PR 自动发版**
- 结合代码审查流程
- 自动化程度最高

### 紧急修复
- 使用 **本地脚本发版**
- 最快的发版方式
- 适合紧急情况

## 🔍 发版后检查

发版完成后，请检查：

1. **GitHub Release** - 确认 release 已创建
2. **构建状态** - 检查 GitHub Actions 是否成功
3. **Homebrew** - 验证 Formula 是否已更新
4. **版本文件** - 确认所有版本号已同步

## 🐛 常见问题

**Q: GitHub Actions 发版失败怎么办？**
A: 检查权限设置，确保 `GITHUB_TOKEN` 有足够权限

**Q: 本地发版时提示权限不足？**
A: 确保有仓库的推送权限，或使用 GitHub Actions 发版

**Q: 版本号没有同步到所有文件？**
A: 检查脚本是否正确更新了 `version.json`、`Cargo.toml`、`package.json` 等文件

**Q: 如何回滚版本？**
A: 删除错误的 tag，重新发版正确的版本号
