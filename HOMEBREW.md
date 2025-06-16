# Homebrew 发布指南

## 🍺 安装方法

```bash
# 添加 tap
brew tap imhuso/cunzhi

# 安装工具集（包含寸止和等一下两个CLI）
brew install cunzhi
```

安装完成后，您将获得两个CLI工具：
- **寸止** - MCP服务器
- **等一下** - 弹窗界面

## 🔧 MCP客户端配置

在您的MCP客户端中添加以下配置：

```json
{
  "mcpServers": {
    "寸止": {
      "command": "寸止"
    }
  }
}
```

## 📋 使用方法

```bash
# 启动MCP服务器
寸止

# 启动设置界面
等一下
```

## 🔄 更新和卸载

```bash
# 更新
brew update
brew upgrade cunzhi

# 卸载
brew uninstall cunzhi
brew untap imhuso/cunzhi
```

---

## 🚀 开发者发布流程

### 自动发布（推荐）

1. 创建并推送tag：
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```

2. GitHub Actions会自动：
   - 构建CLI工具
   - 创建GitHub Release
   - 更新Homebrew Formula

### 手动发布

如果需要手动更新Formula：

1. 计算新版本的SHA256：
   ```bash
   VERSION="v1.0.0"
   wget "https://github.com/imhuso/cunzhi/archive/refs/tags/${VERSION}.tar.gz"
   sha256sum "${VERSION}.tar.gz"
   ```

2. 更新 `Formula/cunzhi.rb`：
   - 修改 `url` 为新版本的下载链接
   - 修改 `sha256` 为计算出的哈希值
   - 修改 `version` 为新版本号

3. 提交更改：
   ```bash
   git add Formula/cunzhi.rb
   git commit -m "Update Homebrew formula to ${VERSION}"
   git push
   ```

## 🧪 本地测试

在发布前，可以本地测试Formula：

```bash
# 安装本地版本
brew install --build-from-source ./Formula/cunzhi.rb

# 测试安装的工具
寸止 --version
等一下 --version

# 卸载测试版本
brew uninstall cunzhi
```

## 📝 注意事项

- 确保每次发布前都测试了CLI工具的构建
- 版本号应该遵循语义化版本规范
- Formula文件中的依赖项要保持最新
- 考虑添加更多平台支持（Linux等）
