# 发布脚本使用指南

## 脚本说明

### 1. `release.sh` - 智能发布脚本

自动化发布流程，支持版本选择和自动同步。

**功能特性：**
- ✅ 交互式版本选择（patch/minor/major/自定义）
- ✅ 自动更新所有配置文件中的版本号
- ✅ Git 提交和标签创建
- ✅ 自动推送触发 GitHub Actions 构建
- ✅ 安全检查（工作目录清洁性）

### 2. `version.js` - 版本号同步工具

独立的版本号管理工具，用于同步更新所有配置文件。

**支持的文件：**
- `version.json`
- `package.json`
- `Cargo.toml`
- `tauri.conf.json`

## 使用方法

### 完整发布流程

```bash
# 运行发布脚本
./scripts/release.sh
```

脚本会引导您：
1. 显示当前版本
2. 选择版本增量类型：
   - `1` - Patch (修复): 1.0.0 → 1.0.1
   - `2` - Minor (功能): 1.0.0 → 1.1.0
   - `3` - Major (重大): 1.0.0 → 2.0.0
   - `4` - 自定义版本
   - `5` - 取消
3. 确认发布
4. 自动执行发布流程

### 仅更新版本号

```bash
# 更新到指定版本
node scripts/version.js 1.2.3

# 查看当前版本
node scripts/version.js --current
```

## 发布流程详解

### 自动执行的步骤：

1. **环境检查**
   - 验证是否在 git 仓库中
   - 检查工作目录是否干净

2. **版本选择**
   - 显示当前版本
   - 提供版本增量选项
   - 支持自定义版本号

3. **版本号同步**
   - 更新 `version.json`
   - 更新 `package.json`
   - 更新 `Cargo.toml`
   - 更新 `tauri.conf.json`

4. **Git 操作**
   - 提交版本更新
   - 创建版本标签
   - 推送到远程仓库

5. **自动构建**
   - GitHub Actions 自动触发
   - 跨平台编译
   - 发布到 Releases

## 示例

### 发布修复版本
```bash
./scripts/release.sh
# 选择 1 (Patch)
# 1.0.0 → 1.0.1
```

### 发布功能版本
```bash
./scripts/release.sh
# 选择 2 (Minor)
# 1.0.0 → 1.1.0
```

### 发布重大版本
```bash
./scripts/release.sh
# 选择 3 (Major)
# 1.0.0 → 2.0.0
```

### 自定义版本
```bash
./scripts/release.sh
# 选择 4 (自定义)
# 输入: 1.5.0
```

## 注意事项

### 发布前检查
- ✅ 确保所有更改已提交
- ✅ 确保在 main 分支上
- ✅ 确保本地代码与远程同步

### 版本号规范
遵循 [语义化版本](https://semver.org/lang/zh-CN/) 规范：
- **MAJOR**: 不兼容的 API 修改
- **MINOR**: 向下兼容的功能性新增
- **PATCH**: 向下兼容的问题修正

### 回滚操作
如果发布出现问题：
```bash
# 删除本地标签
git tag -d v1.2.3

# 删除远程标签
git push origin :refs/tags/v1.2.3

# 回滚提交（如果需要）
git reset --hard HEAD~1
git push origin main --force
```

## 故障排除

### 常见错误

1. **工作目录不干净**
   ```bash
   git status
   git add .
   git commit -m "fix: commit pending changes"
   ```

2. **权限问题**
   ```bash
   chmod +x scripts/release.sh
   ```

3. **Node.js 版本问题**
   ```bash
   node --version  # 需要 Node.js 14+
   ```

### 手动发布
如果脚本失败，可以手动执行：
```bash
# 更新版本号
node scripts/version.js 1.2.3

# 提交和标签
git add .
git commit -m "chore: bump version to 1.2.3"
git tag -a v1.2.3 -m "Release version 1.2.3"
git push origin main
git push origin v1.2.3
```
