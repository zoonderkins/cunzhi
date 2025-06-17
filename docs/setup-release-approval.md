# 设置发布审批环境

为了让 PR 合并后的自动发布流程能够等待手动批准，需要在 GitHub 仓库中设置一个保护环境。

## 🔧 设置步骤

### 1. 创建 Environment

1. 打开 GitHub 仓库页面
2. 点击 **Settings** 标签
3. 在左侧菜单中点击 **Environments**
4. 点击 **New environment** 按钮
5. 输入环境名称：`release-approval`
6. 点击 **Configure environment**

### 2. 配置保护规则

在环境配置页面：

1. **Required reviewers**
   - 勾选 ✅ **Required reviewers**
   - 在搜索框中输入你的用户名并选择
   - 这样只有你能批准发布

2. **Wait timer** (可选)
   - 如果需要，可以设置等待时间
   - 建议设置为 0 分钟，立即可以审批

3. **Deployment branches** (可选)
   - 可以限制只有特定分支能触发
   - 建议保持默认设置

4. 点击 **Save protection rules**

## 🚀 使用流程

### 自动触发
1. 创建 PR 并添加 `release` 标签
2. 合并 PR 到 main 分支
3. GitHub Actions 自动开始运行

### 手动批准
1. 收到 GitHub 通知邮件
2. 打开 GitHub Actions 页面
3. 找到等待中的工作流程
4. 点击 **Review deployments**
5. 查看版本信息
6. 点击 **Approve and deploy** 确认发布

### 发布完成
1. 版本文件自动更新
2. Git tag 自动创建
3. Release 工作流程自动触发
4. 二进制文件构建和发布
5. Homebrew formula 自动更新

## 📋 版本类型规则

工作流程会根据 PR 标签自动确定版本类型：

- **major** 或 **breaking** 标签 → 主版本号 +1 (如 1.0.0 → 2.0.0)
- **minor** 或 **feature** 标签 → 次版本号 +1 (如 1.0.0 → 1.1.0)  
- **其他情况** → 修订版本号 +1 (如 1.0.0 → 1.0.1)

## ⚠️ 注意事项

1. **Environment 名称必须完全匹配**：`release-approval`
2. **必须添加 release 标签**：PR 必须有 `release` 标签才会触发
3. **权限要求**：确保 GitHub Actions 有 `contents: write` 权限
4. **Personal Access Token**：确保 `PERSONAL_ACCESS_TOKEN` secret 已配置

## 🔍 故障排除

### 工作流程没有触发
- 检查 PR 是否有 `release` 标签
- 检查 PR 是否合并到 `main` 分支

### 无法批准部署
- 检查是否正确设置了 `release-approval` 环境
- 检查是否将自己添加为 required reviewer

### 推送失败
- 检查 `PERSONAL_ACCESS_TOKEN` 是否有效
- 检查 token 是否有 `repo` 权限
