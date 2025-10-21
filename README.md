# 寸止 🛑

> **個人客製化版本 - Fork from [imhuso/cunzhi](https://github.com/imhuso/cunzhi)**

這是我針對個人需求 fork 並修改的版本，主要變更：
- ✅ 預設語言改為繁體中文（台灣）
- ✅ 移除 Telegram 整合功能
- ✅ 優化條件性 Prompt 行為
- ✅ 簡化 UI 操作流程

**原專案**：[imhuso/cunzhi](https://github.com/imhuso/cunzhi) - AI 對話互動管理工具

---

## 🌟 核心特性

- 🎨 **優雅交互**：Markdown 支援、多種輸入方式
- ⚡ **即裝即用**：快速安裝，跨平台支援
- 🌐 **完整國際化**：繁體中文（台灣）、簡體中文

## � 安裝方式

### 從原始碼編譯

```bash
# 1. Clone 此專案
git clone https://github.com/zoonderkins/cunzhi.git
cd cunzhi

# 2. 安裝前端依賴
pnpm install

# 3. 編譯 Tauri 應用程式
pnpm tauri:build

# 4. 編譯完成後，執行檔位於：
# macOS: src-tauri/target/release/bundle/macos/
# Linux: src-tauri/target/release/bundle/
# Windows: src-tauri/target/release/bundle/
```

### 系統需求

- **Node.js**: 22+
- **pnpm**: 10+
- **Rust**: 1.90+
- **Tauri CLI**: 2.0+

## ⚡ 快速上手

### 第一步：配置 MCP 客戶端

在您的 MCP 客戶端（如 Claude Desktop）配置檔案中新增：

```json
{
  "mcpServers": {
    "寸止": {
      "command": "寸止"
    }
  }
}
```

### 第二步：開啟設定介面

```bash
# 執行設定命令
等一下
```

### 第三步：配置提示詞

在設定介面的「參考提示詞」標籤頁：
1. 查看自動生成的提示詞
2. 點擊複製按鈕
3. 將提示詞新增到您的 AI 助手中

### 第四步：開始使用

現在您的 AI 助手就擁有了彈窗互動功能！

> 💡 **小提示**：您可以參考生成的提示詞進行個人化修改，打造專屬的 AI 互動體驗。

## 🛠️ 本地開發

```bash
git clone https://github.com/zoonderkins/cunzhi.git
cd cunzhi
pnpm install
pnpm tauri:dev
```

## 📄 開源協議

MIT License - 自由使用，歡迎貢獻！

---

**原專案**：[imhuso/cunzhi](https://github.com/imhuso/cunzhi)
