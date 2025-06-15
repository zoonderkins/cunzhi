# 寸止

## 告别AI提前终止烦恼，助力AI更加持久

寸止 是一个基于 MCP (Model Context Protocol) 的智能交互工具，提供弹窗交互和全局记忆管理功能，让AI助手更加持久和智能。

## ✨ 特性

- 🎯 **MCP 标准兼容**: 完全符合 MCP 2024-11-05 协议规范
- 🎨 **现代化UI界面**: 基于 Vue 3 + Naive UI + UnoCSS 的美观交互界面
- 🌙 **主题系统**: 支持明暗主题自动切换，统一的颜色管理系统
- 🔒 **安全的 Markdown 渲染**: 支持代码高亮和安全的内容渲染
- 🖥️ **原生应用体验**: 基于 Tauri 2.0 的跨平台桌面应用
- ⚡ **高效通信**: 使用进程间通信，响应迅速
- 🛠️ **灵活交互**: 支持预定义选项、多选和自由文本输入
- 🧠 **全局记忆管理**: 智能存储和管理开发规范、用户偏好和最佳实践
- 📷 **图片支持**: 支持图片上传、预览和Base64处理
- 🎵 **音效反馈**: 内置音效系统，提供操作反馈
- ⚙️ **窗口管理**: 支持窗口置顶、大小调整、固定模式和自由拉伸模式

## 📸 功能演示

### 智能弹窗界面

![寸止 弹窗演示](./screenshots/demo.png)

_寸止 的现代化弹窗界面，支持 Markdown 渲染、代码高亮、预定义选项和自由文本输入_

## 🚀 快速安装

### 方式一：下载预编译版本（推荐）

从 [Releases](https://github.com/imhuso/cunzhi/releases) 页面下载对应平台的预编译版本：

- **Linux**: `cunzhi-cli-v*-linux-x86_64.tar.gz`
- **macOS (Intel)**: `cunzhi-cli-v*-macos-x86_64.tar.gz`
- **macOS (Apple Silicon)**: `cunzhi-cli-v*-macos-aarch64.tar.gz`
- **Windows**: `cunzhi-cli-v*-windows-x86_64.zip`

**安装步骤**：
1. 下载对应平台的压缩包并解压
2. 将 `寸止` 和 `等一下` 两个CLI工具放置到任意目录
3. 可选择添加到PATH环境变量，或使用绝对路径调用

**部署示例**：
```bash
# Linux/macOS - 全局安装
tar -xzf cunzhi-cli-v*-linux-x86_64.tar.gz
sudo cp 寸止 等一下 /usr/local/bin/

# Linux/macOS - 用户目录
tar -xzf cunzhi-cli-v*-linux-x86_64.tar.gz
mkdir -p ~/.local/bin
cp 寸止 等一下 ~/.local/bin/
echo 'export PATH="$PATH:~/.local/bin"' >> ~/.bashrc

# Windows - 自定义目录
# 解压 zip 文件到 C:\cunzhi
# 可选：将 C:\cunzhi 添加到系统 PATH
```

### 方式二：使用安装脚本

```bash
# 克隆仓库
git clone https://github.com/imhuso/cunzhi.git
cd cunzhi

# 运行安装脚本
chmod +x install.sh
./install.sh
```

## 📋 部署配置

### MCP 客户端配置

将以下配置添加到您的 MCP 客户端配置文件中：

```json
{
  "mcpServers": {
    "寸止": {
      "command": "寸止"
    }
  }
}
```

**注意**：如果 `寸止` 不在PATH中，请使用绝对路径：
```json
{
  "mcpServers": {
    "寸止": {
      "command": "/path/to/寸止"
    }
  }
}
```

### 工具协作机制

寸止包含两个CLI工具，它们之间有智能的协作机制：

- **寸止**：MCP服务器，处理AI客户端的请求
- **等一下**：弹窗界面，提供用户交互和设置管理

**自动发现逻辑**：
1. 当MCP服务器需要显示弹窗时，会自动查找 `等一下` 工具
2. 查找顺序：同目录 → PATH环境变量 → 错误提示
3. 这意味着您可以将两个工具放在任何位置，只要它们能被找到即可

## 🛠️ MCP 工具

### 1. zhi - 智能交互工具

弹窗交互工具，支持多种输入方式和Markdown渲染。

**主要参数**：
- `message` (必需): 显示给用户的消息内容
- `predefined_options` (可选): 预定义选项列表，支持多选
- `is_markdown` (可选): 是否启用Markdown格式渲染，默认为true

**功能特性**：
- ✅ 预定义选项多选 + 自由文本输入
- ✅ 图片上传、拖拽和Base64处理
- ✅ Markdown渲染和代码高亮
- ✅ 现代化主题系统（明暗主题）
- ✅ 代码复制功能
- ✅ 图片预览和缩放

### 2. jiyi - 全局记忆管理工具

智能记忆管理系统，按项目存储开发规范和偏好设置。

**主要参数**：
- `action` (必需): 操作类型 ("记忆" 或 "回忆")
- `project_path` (必需): 项目路径
- `content` (记忆时必需): 记忆内容
- `category` (可选): 记忆分类 (rule/preference/pattern/context)

**功能特性**：
- ✅ 按项目自动组织记忆
- ✅ 智能分类管理 (规范/偏好/模式/上下文)
- ✅ Git仓库根目录自动识别
- ✅ 压缩格式输出，节省token

## ⚙️ MCP工具管理

寸止支持对MCP工具进行灵活的配置管理，让您可以根据需要启用或禁用特定工具。

### 工具配置特性

- 🎛️ **工具开关**: 支持对可选工具进行启用/禁用控制
- 🔒 **核心保护**: 核心工具（寸止）无法禁用，确保基本功能
- 💾 **配置持久化**: 设置自动保存，重启后保持配置状态
- 🔄 **实时提醒**: 配置变更后提示重连MCP服务以生效
- 📊 **状态统计**: 实时显示已启用工具数量和总体状态

### 工具管理界面

在设置界面的"MCP工具"标签页中，您可以：

1. **查看工具状态**: 每个工具显示名称、描述、启用状态和必需性标识
2. **切换工具状态**: 点击开关即可启用或禁用可选工具
3. **查看工具信息**: 悬停描述可查看完整的工具功能说明
4. **状态反馈**: 界面实时显示工具配置变更和操作结果

### 支持的工具

| 工具 | 名称 | 必需性 | 功能描述 |
|------|------|--------|----------|
| `zhi` | 寸止 | 必需 ⚠️ | 智能代码审查交互工具，支持预定义选项、自由文本输入和图片上传 |
| `ji` | 记忆管理 | 可选 ✅ | 全局记忆管理工具，用于存储和管理重要的开发规范、用户偏好和最佳实践 |

### 配置说明

**必需工具**:
- 寸止（zhi）工具是系统核心功能，始终保持启用状态，无法禁用
- 这确保了MCP客户端始终能够进行基本的交互操作

**可选工具**:
- 记忆管理（ji）工具可以根据需要启用或禁用
- 禁用后，MCP客户端将无法使用该工具的功能
- 配置变更后需要在MCP客户端中重新连接服务以使更改生效

## 📋 使用方法

### 启动MCP服务器
```bash
寸止
```

### 启动弹窗界面
```bash
等一下                          # 启动设置界面
等一下 --mcp-request file       # MCP弹窗模式
```

### 主要功能

**设置管理**：
- 主题切换（明暗主题）
- 窗口管理（置顶、大小调整、固定/自由拉伸模式）
- 音效控制
- 继续回复设置
- MCP工具配置（启用/禁用工具）

**弹窗交互**：
- Markdown内容渲染
- 预定义选项选择
- 自由文本输入
- 图片上传和处理
- 实时预览和反馈

## 📁 项目结构

```
cunzhi/
├── src/
│   ├── rust/
│   │   ├── mcp/                 # MCP服务器实现
│   │   │   ├── tools/           # MCP工具实现
│   │   │   │   ├── memory/      # 记忆管理工具
│   │   │   │   └── interaction/ # 智能交互工具
│   │   │   ├── handlers/        # 请求处理器
│   │   │   └── utils/           # 工具函数
│   │   ├── ui/                  # UI相关功能
│   │   ├── config/              # 配置管理
│   │   ├── bin/mcp_server.rs    # MCP 服务器入口
│   │   ├── main.rs              # 主应用入口
│   │   └── lib.rs               # 库入口
│   ├── frontend/                # Vue 3 前端
│   │   ├── components/          # Vue组件
│   │   ├── composables/         # Vue组合式API
│   │   ├── stores/              # 状态管理
│   │   └── utils/               # 前端工具函数
│   └── assets/                  # 静态资源
│       └── sounds/              # 音频资源
├── .github/workflows/
│   └── build.yml                # 跨平台自动构建
├── scripts/                     # 构建和发布脚本
├── install.sh                   # 安装脚本
├── Cargo.toml                   # Rust 配置
├── package.json                 # Node.js 配置
├── tauri.conf.json              # Tauri 配置
├── uno.config.ts                # UnoCSS 配置
├── vite.config.js               # Vite 配置
└── version.json                 # 版本配置
```

## 🧪 开发

### 环境要求

- **Rust**: 1.70+
- **Node.js**: 18+
- **pnpm**: 推荐包管理器

### 本地开发

```bash
# 克隆项目
git clone https://github.com/imhuso/cunzhi.git
cd cunzhi

# 安装前端依赖
pnpm install

# 开发模式运行
pnpm tauri:dev

# 或者分别运行前后端
pnpm dev          # 前端开发服务器
cargo run         # Tauri 应用
```

### 构建发布版本

```bash
# 构建前端
pnpm build

# 构建 CLI 工具
cargo build --release

# 构建 Tauri 应用包
pnpm tauri:build

# 运行安装脚本（生成CLI工具）
./install.sh
```

### 技术栈

**前端**：
- Vue 3 (Composition API)
- Naive UI (组件库)
- UnoCSS (原子化CSS)
- TypeScript
- Vite (构建工具)

**后端**：
- Rust
- Tauri 2.0 (跨平台框架)
- rmcp (MCP SDK)
- Tokio (异步运行时)

## 🔧 故障排除

### 构建问题

如果遇到构建错误，请检查以下几点：

1. **Linux 系统依赖**：
   ```bash
   # Ubuntu/Debian
   sudo apt-get install -y \
     libwebkit2gtk-4.1-dev \
     libappindicator3-dev \
     librsvg2-dev \
     patchelf \
     pkg-config \
     libglib2.0-dev \
     libgtk-3-dev \
     libasound2-dev

   # CentOS/RHEL
   sudo yum install pkgconfig gtk3-devel alsa-lib-devel
   ```

2. **Windows 构建**：
   - 确保安装了 Visual Studio Build Tools 2022
   - 安装 WebView2 运行时

3. **macOS 构建**：
   - 确保安装了 Xcode Command Line Tools
   ```bash
   xcode-select --install
   ```

### 运行时问题

- **权限问题**：确保 CLI 工具有执行权限 (`chmod +x`)
- **路径问题**：确保两个工具在同一目录或都在 PATH 中
- **依赖问题**：检查系统是否安装了必要的运行时库
- **弹窗不显示**：检查 `等一下` 工具是否能被找到

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License
