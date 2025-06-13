#!/bin/bash

# 寸止 MCP 工具发布脚本
# 支持版本选择、自动同步版本号、打tag并合并到main

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 打印带颜色的消息
print_info() {
    echo -e "${BLUE}ℹ️  $1${NC}"
}

print_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

print_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

print_error() {
    echo -e "${RED}❌ $1${NC}"
    exit 1
}

# 检查是否在git仓库中
check_git_repo() {
    if ! git rev-parse --git-dir > /dev/null 2>&1; then
        print_error "当前目录不是git仓库"
    fi
}

# 检查工作目录是否干净
check_clean_working_dir() {
    if ! git diff-index --quiet HEAD --; then
        print_error "工作目录有未提交的更改，请先提交或暂存"
    fi
}

# 获取当前版本
get_current_version() {
    if [ -f "version.json" ]; then
        grep -o '"version"[[:space:]]*:[[:space:]]*"[^"]*"' version.json | cut -d'"' -f4
    elif [ -f "Cargo.toml" ]; then
        grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2
    else
        echo "0.0.0"
    fi
}

# 解析版本号
parse_version() {
    local version=$1
    echo $version | sed 's/v//' | tr '.' ' '
}

# 增加版本号
increment_version() {
    local current_version=$1
    local increment_type=$2
    
    read -r major minor patch <<< $(parse_version $current_version)
    
    case $increment_type in
        "major")
            major=$((major + 1))
            minor=0
            patch=0
            ;;
        "minor")
            minor=$((minor + 1))
            patch=0
            ;;
        "patch")
            patch=$((patch + 1))
            ;;
        *)
            print_error "无效的版本增量类型: $increment_type"
            ;;
    esac
    
    echo "$major.$minor.$patch"
}

# 显示版本选择菜单
show_version_menu() {
    local current_version=$1
    
    echo
    print_info "当前版本: $current_version"
    echo
    echo "请选择新版本类型:"
    echo "1) Patch (修复): $(increment_version $current_version patch)"
    echo "2) Minor (功能): $(increment_version $current_version minor)"
    echo "3) Major (重大): $(increment_version $current_version major)"
    echo "4) 自定义版本"
    echo "5) 取消"
    echo
}

# 获取用户选择的版本
get_new_version() {
    local current_version=$1
    local new_version=""
    
    while true; do
        show_version_menu $current_version
        read -p "请选择 (1-5): " choice
        
        case $choice in
            1)
                new_version=$(increment_version $current_version patch)
                break
                ;;
            2)
                new_version=$(increment_version $current_version minor)
                break
                ;;
            3)
                new_version=$(increment_version $current_version major)
                break
                ;;
            4)
                read -p "请输入自定义版本号 (格式: x.y.z): " custom_version
                if [[ $custom_version =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
                    new_version=$custom_version
                    break
                else
                    print_error "版本号格式无效，请使用 x.y.z 格式"
                fi
                ;;
            5)
                print_info "取消发布"
                exit 0
                ;;
            *)
                print_warning "无效选择，请重新选择"
                ;;
        esac
    done
    
    echo $new_version
}

# 更新版本号文件
update_version_files() {
    local new_version=$1
    
    print_info "更新版本号到 $new_version..."
    
    # 更新 version.json
    if [ -f "version.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" version.json
        rm -f version.json.bak
        print_success "已更新 version.json"
    fi
    
    # 更新 Cargo.toml
    if [ -f "Cargo.toml" ]; then
        sed -i.bak "s/^version = \"[^\"]*\"/version = \"$new_version\"/" Cargo.toml
        rm -f Cargo.toml.bak
        print_success "已更新 Cargo.toml"
    fi
    
    # 更新 package.json
    if [ -f "package.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" package.json
        rm -f package.json.bak
        print_success "已更新 package.json"
    fi
    
    # 更新 tauri.conf.json
    if [ -f "tauri.conf.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" tauri.conf.json
        rm -f tauri.conf.json.bak
        print_success "已更新 tauri.conf.json"
    fi
}

# 确认发布
confirm_release() {
    local new_version=$1
    
    echo
    print_warning "即将发布版本 $new_version"
    echo "这将会："
    echo "  1. 更新所有版本号文件"
    echo "  2. 提交更改"
    echo "  3. 创建并推送 tag v$new_version"
    echo "  4. 触发 GitHub Actions 构建"
    echo
    
    read -p "确认继续? (y/N): " confirm
    if [[ ! $confirm =~ ^[Yy]$ ]]; then
        print_info "取消发布"
        exit 0
    fi
}

# 执行发布
perform_release() {
    local new_version=$1
    
    print_info "开始发布流程..."
    
    # 更新版本号
    update_version_files $new_version
    
    # 提交更改
    print_info "提交版本更新..."
    git add .
    git commit -m "chore: bump version to $new_version"
    
    # 创建tag
    print_info "创建tag v$new_version..."
    git tag -a "v$new_version" -m "Release version $new_version"
    
    # 推送到远程
    print_info "推送到远程仓库..."
    git push origin main
    git push origin "v$new_version"
    
    print_success "发布完成！"
    print_info "GitHub Actions 将自动构建并发布到 Releases"
    print_info "查看构建状态: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^/]*\/[^/]*\).*/\1/' | sed 's/\.git$//')/actions"
}

# 主函数
main() {
    echo "🚀 寸止 MCP 工具发布脚本"
    echo "=========================="
    
    # 检查环境
    check_git_repo
    check_clean_working_dir
    
    # 获取当前版本
    current_version=$(get_current_version)
    
    # 获取新版本
    new_version=$(get_new_version $current_version)
    
    # 确认发布
    confirm_release $new_version
    
    # 执行发布
    perform_release $new_version
}

# 运行主函数
main "$@"
