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

    echo >&2
    echo "当前版本: $current_version" >&2
    echo >&2
    echo "请选择新版本类型:" >&2
    echo "1) Patch (修复): $(increment_version $current_version patch)" >&2
    echo "2) Minor (功能): $(increment_version $current_version minor)" >&2
    echo "3) Major (重大): $(increment_version $current_version major)" >&2
    echo "4) 自定义版本" >&2
    echo "5) 取消" >&2
    echo >&2
}

# 获取用户选择的版本
get_new_version() {
    local current_version=$1
    local new_version=""

    while true; do
        show_version_menu $current_version
        read -p "请选择 (1-5) [默认: 1]: " choice

        # 如果用户直接按回车，默认选择 1 (patch)
        if [[ -z "$choice" ]]; then
            choice=1
        fi

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
                    echo "版本号格式无效，请使用 x.y.z 格式" >&2
                fi
                ;;
            5)
                echo "取消发布" >&2
                echo "CANCEL"
                return 0
                ;;
            *)
                echo "无效选择，请重新选择" >&2
                ;;
        esac
    done

    echo $new_version
}

# 处理Cargo.lock文件
handle_cargo_lock() {
    if [ -f "Cargo.lock" ]; then
        echo "重新生成Cargo.lock文件..."
        cargo check --quiet 2>/dev/null || true
    fi
}

# 更新版本号文件
update_version_files() {
    local new_version=$1
    local current_date=$(date +"%Y-%m-%d")

    echo "更新版本号到 $new_version..."

    # 更新 version.json
    if [ -f "version.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" version.json
        sed -i.bak "s/\"build_date\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"build_date\": \"$current_date\"/" version.json
        rm -f version.json.bak
        echo "已更新 version.json"
    fi

    # 更新 Cargo.toml
    if [ -f "Cargo.toml" ]; then
        sed -i.bak "s/^version = \"[^\"]*\"/version = \"$new_version\"/" Cargo.toml
        rm -f Cargo.toml.bak
        echo "已更新 Cargo.toml"
    fi

    # 更新 package.json
    if [ -f "package.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" package.json
        rm -f package.json.bak
        echo "已更新 package.json"
    fi

    # 更新 tauri.conf.json
    if [ -f "tauri.conf.json" ]; then
        sed -i.bak "s/\"version\"[[:space:]]*:[[:space:]]*\"[^\"]*\"/\"version\": \"$new_version\"/" tauri.conf.json
        rm -f tauri.conf.json.bak
        echo "已更新 tauri.conf.json"
    fi
}

# 确认发布
confirm_release() {
    local new_version=$1
    local current_branch=$(git branch --show-current)

    echo
    print_info "即将发布版本 $new_version"
    echo "发布流程："
    if [[ "$current_branch" != "main" ]]; then
        echo "  1. 推送当前分支 $current_branch 到远程"
        echo "  2. 切换到main分支并拉取最新代码"
        echo "  3. 合并 $current_branch 分支到main"
        echo "  4. 在main分支上更新版本号文件"
        echo "  5. 提交版本更新并创建tag v$new_version"
        echo "  6. 推送main分支和tag到远程"
        echo "  7. 切换回 $current_branch 分支并同步版本更新"
        echo "  8. 触发 GitHub Actions 构建"
    else
        echo "  1. 拉取main分支最新代码"
        echo "  2. 在main分支上更新版本号文件"
        echo "  3. 提交版本更新并创建tag v$new_version"
        echo "  4. 推送main分支和tag到远程"
        echo "  5. 触发 GitHub Actions 构建"
    fi
    echo

    read -p "确认继续? (Y/n) [默认: Y]: " confirm
    # 如果用户直接按回车，默认为 Y
    if [[ -z "$confirm" ]]; then
        confirm="Y"
    fi
    if [[ ! $confirm =~ ^[Yy]$ ]]; then
        print_warning "取消发布"
        exit 0
    fi
}

# 执行发布
perform_release() {
    local new_version=$1
    local current_branch=$(git branch --show-current)

    print_info "开始发布流程..."
    print_info "当前分支: $current_branch"

    # 如果当前不在main分支，先推送当前分支的更改
    if [[ "$current_branch" != "main" ]]; then
        print_info "推送当前分支 $current_branch 到远程..."
        git push origin "$current_branch"
    fi

    # 切换到main分支
    print_info "切换到main分支..."
    git checkout main

    # 拉取main分支最新代码
    print_info "拉取main分支最新代码..."
    git pull origin main --no-rebase

    # 如果不是从main分支执行的，合并当前分支到main
    if [[ "$current_branch" != "main" ]]; then
        print_info "合并 $current_branch 分支到main..."
        if ! git merge "$current_branch" --no-ff -m "feat: merge $current_branch for version $new_version"; then
            print_error "合并失败，可能存在冲突。请手动解决冲突后重新运行脚本。"
        fi
    fi

    # 在main分支上更新版本号
    print_info "在main分支上更新版本号到 $new_version..."
    update_version_files $new_version

    # 处理Cargo.lock文件
    handle_cargo_lock

    # 提交版本更新
    print_info "提交版本更新..."
    git add .
    git commit -m "release: Release $new_version"

    # 检查并提交任何剩余的更改（如Cargo.lock）
    if ! git diff-index --quiet HEAD --; then
        print_info "提交剩余的更改（如Cargo.lock）..."
        git add .
        git commit -m "chore: update Cargo.lock after version bump"
    fi

    # 获取当前commit的标题作为tag消息（Release xxx）
    release_commit_title=$(git log --format=%s -1)

    # 创建tag（不包含颜色代码）
    print_info "创建tag v$new_version..."
    git tag -a "v$new_version" -m "$release_commit_title"

    # 推送main分支和tag
    print_info "推送main分支和tag到远程仓库..."
    git push origin main
    git push origin "v$new_version"

    # 切换回原分支（如果不是main）
    if [[ "$current_branch" != "main" ]]; then
        print_info "切换回 $current_branch 分支..."
        git checkout "$current_branch"

        # 将main分支的版本更新合并回当前分支
        print_info "将版本更新合并回 $current_branch 分支..."
        git merge main --no-ff -m "chore: sync version update from main"
        git push origin "$current_branch"
    fi

    print_success "发布完成！"
    print_info "GitHub Actions 将自动构建并发布到 Releases"
    print_info "查看构建状态: https://github.com/$(git config --get remote.origin.url | sed 's/.*github.com[:/]\([^/]*\/[^/]*\).*/\1/' | sed 's/\.git$//')/actions"
}

# 主函数
main() {
    echo "寸止 MCP 工具发布脚本"
    echo "=========================="
    
    # 检查环境
    check_git_repo
    check_clean_working_dir
    
    # 获取当前版本
    current_version=$(get_current_version)
    
    # 获取新版本
    new_version=$(get_new_version $current_version)

    # 检查是否取消
    if [[ "$new_version" == "CANCEL" ]]; then
        exit 0
    fi

    # 确认发布
    confirm_release $new_version
    
    # 执行发布
    perform_release $new_version
}

# 运行主函数（仅在直接执行脚本时运行）
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
