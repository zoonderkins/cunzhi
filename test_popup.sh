#!/bin/bash

# 寸止弹窗测试脚本
# 使用 target/release 中的 CLI 工具测试弹窗功能

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# 默认构建类型
BUILD_TYPE="release"
CLI_PATH="$PROJECT_ROOT/target/$BUILD_TYPE"

echo -e "${BLUE}🎯 寸止弹窗测试脚本${NC}"
echo -e "${BLUE}================================${NC}"

# 选择构建类型
select_build_type() {
    echo -e "${YELLOW}🔧 选择构建类型:${NC}"
    echo -e "  ${GREEN}1.${NC} Release (推荐，性能最佳)"
    echo -e "  ${GREEN}2.${NC} Debug (包含调试信息)"
    echo ""

    while true; do
        read -p "请选择构建类型 (1-2): " build_choice
        case $build_choice in
            1)
                BUILD_TYPE="release"
                CLI_PATH="$PROJECT_ROOT/target/$BUILD_TYPE"
                echo -e "${GREEN}✅ 已选择 Release 构建${NC}"
                break
                ;;
            2)
                BUILD_TYPE="debug"
                CLI_PATH="$PROJECT_ROOT/target/$BUILD_TYPE"
                echo -e "${GREEN}✅ 已选择 Debug 构建${NC}"
                break
                ;;
            *)
                echo -e "${RED}❌ 无效选项，请选择 1 或 2${NC}"
                ;;
        esac
    done
    echo ""
}

# 编译项目
compile_project() {
    echo -e "${YELLOW}🔨 开始编译项目...${NC}"

    # 检查Cargo.toml是否存在
    if [[ ! -f "$PROJECT_ROOT/Cargo.toml" ]]; then
        echo -e "${RED}❌ 未找到 Cargo.toml 文件${NC}"
        echo -e "${YELLOW}💡 请确保在Rust项目根目录中运行此脚本${NC}"
        exit 1
    fi

    # 检查cargo是否存在
    if ! command -v cargo &> /dev/null; then
        echo -e "${RED}❌ 未找到 cargo 命令${NC}"
        echo -e "${YELLOW}💡 请先安装 Rust: https://rustup.rs/${NC}"
        exit 1
    fi

    # 根据构建类型编译
    if [[ "$BUILD_TYPE" == "release" ]]; then
        echo -e "${BLUE}📦 编译 Release 版本...${NC}"
        if cargo build --release; then
            echo -e "${GREEN}✅ Release 编译完成${NC}"
        else
            echo -e "${RED}❌ Release 编译失败${NC}"
            exit 1
        fi
    else
        echo -e "${BLUE}📦 编译 Debug 版本...${NC}"
        if cargo build; then
            echo -e "${GREEN}✅ Debug 编译完成${NC}"
        else
            echo -e "${RED}❌ Debug 编译失败${NC}"
            exit 1
        fi
    fi
    echo ""
}

# 检查CLI工具是否存在
check_cli_tools() {
    echo -e "${YELLOW}📋 检查CLI工具 (${BUILD_TYPE})...${NC}"

    if [[ ! -f "$CLI_PATH/寸止" ]]; then
        echo -e "${RED}❌ 未找到 寸止 CLI工具${NC}"
        if [[ "$BUILD_TYPE" == "release" ]]; then
            echo -e "${YELLOW}💡 请先编译项目: cargo build --release${NC}"
        else
            echo -e "${YELLOW}💡 请先编译项目: cargo build${NC}"
        fi

        # 提供自动编译选项
        echo -e "${BLUE}🔨 是否现在编译项目？ (y/n)${NC}"
        read -p "请选择: " compile_choice
        if [[ "$compile_choice" =~ ^[Yy]$ ]]; then
            compile_project
        else
            exit 1
        fi
    fi

    if [[ ! -f "$CLI_PATH/等一下" ]]; then
        echo -e "${RED}❌ 未找到 等一下 CLI工具${NC}"
        if [[ "$BUILD_TYPE" == "release" ]]; then
            echo -e "${YELLOW}💡 请先编译项目: cargo build --release${NC}"
        else
            echo -e "${YELLOW}💡 请先编译项目: cargo build${NC}"
        fi

        # 提供自动编译选项
        echo -e "${BLUE}🔨 是否现在编译项目？ (y/n)${NC}"
        read -p "请选择: " compile_choice
        if [[ "$compile_choice" =~ ^[Yy]$ ]]; then
            compile_project
        else
            exit 1
        fi
    fi

    # 检查执行权限
    if [[ ! -x "$CLI_PATH/寸止" ]]; then
        echo -e "${YELLOW}⚠️  寸止 CLI工具没有执行权限，正在添加...${NC}"
        chmod +x "$CLI_PATH/寸止"
    fi

    if [[ ! -x "$CLI_PATH/等一下" ]]; then
        echo -e "${YELLOW}⚠️  等一下 CLI工具没有执行权限，正在添加...${NC}"
        chmod +x "$CLI_PATH/等一下"
    fi

    echo -e "${GREEN}✅ CLI工具检查完成 (${BUILD_TYPE})${NC}"
    echo -e "   构建类型: ${BUILD_TYPE}"
    echo -e "   寸止: $CLI_PATH/寸止"
    echo -e "   等一下: $CLI_PATH/等一下"
}

# 检查测试JSON文件
check_test_files() {
    echo -e "${YELLOW}📋 检查测试文件...${NC}"
    
    if [[ ! -f "$PROJECT_ROOT/test_simple_popup.json" ]]; then
        echo -e "${RED}❌ 未找到 test_simple_popup.json${NC}"
        exit 1
    fi
    
    if [[ ! -f "$PROJECT_ROOT/test_markdown_popup.json" ]]; then
        echo -e "${RED}❌ 未找到 test_markdown_popup.json${NC}"
        exit 1
    fi
    
    echo -e "${GREEN}✅ 测试文件检查完成${NC}"
}

# 显示测试选项
show_test_options() {
    echo -e "${BLUE}🎨 可用的测试选项:${NC}"
    echo -e "${YELLOW}当前构建类型: ${BUILD_TYPE}${NC}"
    echo ""
    echo -e "  ${GREEN}1.${NC} 测试简单弹窗 (test_simple_popup.json)"
    echo -e "  ${GREEN}2.${NC} 测试Markdown弹窗 (test_markdown_popup.json)"
    echo -e "  ${GREEN}3.${NC} 测试自定义弹窗"
    echo -e "  ${GREEN}4.${NC} 启动前端测试环境"
    echo -e "  ${GREEN}5.${NC} 查看CLI工具帮助"
    echo -e "  ${GREEN}6.${NC} 切换构建类型 (当前: ${BUILD_TYPE})"
    echo -e "  ${GREEN}7.${NC} 重新编译项目"
    echo -e "  ${GREEN}q.${NC} 退出"
    echo ""
}

# 显示JSON文件内容（兼容有无jq的情况）
show_json_content() {
    local file_path="$1"
    if command -v jq &> /dev/null; then
        cat "$file_path" | jq '.'
    else
        echo "JSON内容:"
        cat "$file_path"
    fi
}

# 测试简单弹窗
test_simple_popup() {
    echo -e "${YELLOW}🚀 启动简单弹窗测试...${NC}"
    echo -e "${BLUE}使用文件: test_simple_popup.json${NC}"

    # 显示文件内容
    echo -e "${YELLOW}📄 文件内容:${NC}"
    show_json_content "$PROJECT_ROOT/test_simple_popup.json"
    echo ""

    # 启动弹窗
    echo -e "${GREEN}🎯 启动弹窗...${NC}"
    if "$CLI_PATH/等一下" --mcp-request "$PROJECT_ROOT/test_simple_popup.json"; then
        echo -e "${GREEN}✅ 弹窗测试完成${NC}"
    else
        echo -e "${RED}❌ 弹窗测试失败${NC}"
        echo -e "${YELLOW}💡 请检查CLI工具是否正常工作${NC}"
    fi
}

# 测试Markdown弹窗
test_markdown_popup() {
    echo -e "${YELLOW}🚀 启动Markdown弹窗测试...${NC}"
    echo -e "${BLUE}使用文件: test_markdown_popup.json${NC}"

    # 显示文件内容
    echo -e "${YELLOW}📄 文件内容:${NC}"
    show_json_content "$PROJECT_ROOT/test_markdown_popup.json"
    echo ""

    # 启动弹窗
    echo -e "${GREEN}🎯 启动弹窗...${NC}"
    if "$CLI_PATH/等一下" --mcp-request "$PROJECT_ROOT/test_markdown_popup.json"; then
        echo -e "${GREEN}✅ Markdown弹窗测试完成${NC}"
    else
        echo -e "${RED}❌ Markdown弹窗测试失败${NC}"
        echo -e "${YELLOW}💡 请检查CLI工具是否正常工作${NC}"
    fi
}

# 测试自定义弹窗
test_custom_popup() {
    echo -e "${YELLOW}🚀 创建自定义弹窗测试...${NC}"
    
    # 创建临时测试文件
    TEMP_FILE="/tmp/custom_popup_test.json"
    
    cat > "$TEMP_FILE" << 'EOF'
{
  "id": "custom-test-001",
  "message": "# 🎨 自定义弹窗测试\n\n这是一个自定义的弹窗测试，用于验证弹窗功能的完整性。\n\n## ✨ 测试功能\n- 头部固定显示\n- 工具栏固定显示\n- 图片组件渲染\n- 输入框组件\n- 禁止选中非内容区域\n- Markdown紧凑渲染\n\n## 🔧 操作说明\n1. 测试主题切换按钮\n2. 测试打开主界面按钮\n3. 测试预定义选项选择\n4. 测试文本输入功能\n5. 测试图片粘贴功能\n\n```typescript\n// 示例代码\ninterface PopupTest {\n  header: 'fixed'\n  toolbar: 'fixed'\n  content: 'scrollable'\n  images: 'component-rendered'\n  input: 'component-based'\n}\n```\n\n> **注意**: 请测试所有交互功能以确保弹窗工作正常。",
  "predefined_options": [
    "🎨 测试主题切换",
    "🏠 测试主界面按钮", 
    "📝 测试文本输入",
    "🖼️ 测试图片功能",
    "⚡ 测试快捷键",
    "✅ 测试完成",
    "❌ 发现问题"
  ],
  "is_markdown": true
}
EOF
    
    echo -e "${YELLOW}📄 自定义测试内容:${NC}"
    show_json_content "$TEMP_FILE"
    echo ""
    
    # 启动弹窗
    echo -e "${GREEN}🎯 启动自定义弹窗...${NC}"
    if "$CLI_PATH/等一下" --mcp-request "$TEMP_FILE"; then
        echo -e "${GREEN}✅ 自定义弹窗测试完成${NC}"
    else
        echo -e "${RED}❌ 自定义弹窗测试失败${NC}"
        echo -e "${YELLOW}💡 请检查CLI工具是否正常工作${NC}"
    fi

    # 清理临时文件
    rm -f "$TEMP_FILE"
}

# 启动前端测试环境
start_frontend_test() {
    echo -e "${YELLOW}🚀 启动前端测试环境...${NC}"
    echo -e "${BLUE}测试环境将在 http://localhost:5174 启动${NC}"
    echo -e "${YELLOW}💡 按 Ctrl+C 停止测试环境${NC}"
    echo ""

    # 检查pnpm是否存在
    if ! command -v pnpm &> /dev/null; then
        echo -e "${RED}❌ 未找到 pnpm 命令${NC}"
        echo -e "${YELLOW}💡 请先安装 pnpm: npm install -g pnpm${NC}"
        return 1
    fi

    # 检查package.json是否存在
    if [[ ! -f "$PROJECT_ROOT/package.json" ]]; then
        echo -e "${RED}❌ 未找到 package.json 文件${NC}"
        return 1
    fi

    # 启动测试环境
    cd "$PROJECT_ROOT" && pnpm test:ui
}

# 显示CLI帮助
show_cli_help() {
    echo -e "${YELLOW}📖 CLI工具帮助信息:${NC}"
    echo ""

    echo -e "${BLUE}寸止 CLI:${NC}"
    if "$CLI_PATH/寸止" --help 2>/dev/null; then
        echo -e "${GREEN}✅ 帮助信息显示完成${NC}"
    else
        echo -e "${YELLOW}⚠️  寸止 CLI 无帮助信息或不支持 --help 参数${NC}"
        echo -e "${BLUE}尝试直接运行:${NC} $CLI_PATH/寸止"
    fi
    echo ""

    echo -e "${BLUE}等一下 CLI:${NC}"
    if "$CLI_PATH/等一下" --help 2>/dev/null; then
        echo -e "${GREEN}✅ 帮助信息显示完成${NC}"
    else
        echo -e "${YELLOW}⚠️  等一下 CLI 无帮助信息或不支持 --help 参数${NC}"
        echo -e "${BLUE}尝试直接运行:${NC} $CLI_PATH/等一下"
        echo -e "${BLUE}MCP请求参数:${NC} $CLI_PATH/等一下 --mcp-request <json_file>"
    fi
}

# 切换构建类型
switch_build_type() {
    echo -e "${YELLOW}🔄 切换构建类型${NC}"
    echo -e "当前构建类型: ${BUILD_TYPE}"
    echo ""

    if [[ "$BUILD_TYPE" == "release" ]]; then
        BUILD_TYPE="debug"
        echo -e "${GREEN}✅ 已切换到 Debug 构建${NC}"
    else
        BUILD_TYPE="release"
        echo -e "${GREEN}✅ 已切换到 Release 构建${NC}"
    fi

    CLI_PATH="$PROJECT_ROOT/target/$BUILD_TYPE"
    echo -e "新的CLI路径: $CLI_PATH"
    echo ""

    # 重新检查CLI工具
    check_cli_tools
}

# 重新编译项目
recompile_project() {
    echo -e "${YELLOW}🔨 重新编译项目 (${BUILD_TYPE})...${NC}"
    compile_project

    # 重新检查CLI工具
    check_cli_tools
}

# 主函数
main() {
    # 选择构建类型
    select_build_type

    # 检查依赖
    check_cli_tools
    check_test_files

    echo ""
    
    # 主循环
    while true; do
        show_test_options
        read -p "请选择测试选项 (1-7, q): " choice
        echo ""

        case $choice in
            1)
                test_simple_popup
                ;;
            2)
                test_markdown_popup
                ;;
            3)
                test_custom_popup
                ;;
            4)
                start_frontend_test
                ;;
            5)
                show_cli_help
                ;;
            6)
                switch_build_type
                ;;
            7)
                recompile_project
                ;;
            q|Q)
                echo -e "${GREEN}👋 测试结束，再见！${NC}"
                exit 0
                ;;
            *)
                echo -e "${RED}❌ 无效选项，请重新选择${NC}"
                ;;
        esac
        
        echo ""
        echo -e "${YELLOW}按回车键继续...${NC}"
        read
        echo ""
    done
}

# 检查依赖工具
echo -e "${BLUE}🔍 检查依赖工具...${NC}"
if ! command -v jq &> /dev/null; then
    echo -e "${YELLOW}⚠️  建议安装 jq 以获得更好的JSON显示效果${NC}"
    echo -e "${YELLOW}   macOS: brew install jq${NC}"
    echo -e "${YELLOW}   Ubuntu/Debian: sudo apt install jq${NC}"
    echo -e "${YELLOW}   CentOS/RHEL: sudo yum install jq${NC}"
    echo ""
else
    echo -e "${GREEN}✅ jq 已安装${NC}"
fi

if ! command -v pnpm &> /dev/null; then
    echo -e "${YELLOW}⚠️  建议安装 pnpm 以使用前端测试环境${NC}"
    echo -e "${YELLOW}   安装命令: npm install -g pnpm${NC}"
    echo ""
else
    echo -e "${GREEN}✅ pnpm 已安装${NC}"
fi
echo ""

# 运行主函数
main "$@"
