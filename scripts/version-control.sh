#!/bin/bash

# Morgan Code 版本控制工具
# 提供一键保存、回滚、查看历史等功能

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

cd "$PROJECT_DIR"

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 显示帮助信息
show_help() {
    cat << EOF
Morgan Code 版本控制工具

用法: $0 <命令> [参数]

命令:
  save <message>        保存当前版本（创建 commit 和 tag）
  list                  列出所有版本标签
  show <tag>            显示指定版本的详细信息
  rollback <tag>        回滚到指定版本
  diff <tag1> <tag2>    比较两个版本的差异
  status                显示当前状态
  backup                创建 tar.gz 备份
  help                  显示此帮助信息

示例:
  $0 save "添加新功能"
  $0 list
  $0 rollback v0.2.0-streaming
  $0 diff v0.1.0 v0.2.0-streaming

EOF
}

# 保存当前版本
save_version() {
    local message="$1"
    if [ -z "$message" ]; then
        echo -e "${RED}错误: 请提供提交信息${NC}"
        echo "用法: $0 save <message>"
        exit 1
    fi

    echo -e "${BLUE}📝 保存当前版本...${NC}"

    # 检查是否有更改
    if git diff --quiet && git diff --cached --quiet; then
        echo -e "${YELLOW}⚠️  没有检测到更改${NC}"
        exit 0
    fi

    # 显示将要提交的更改
    echo -e "\n${BLUE}将要提交的更改:${NC}"
    git status --short

    # 添加��有更改
    git add -A

    # 创建提交
    git commit -m "$message

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>"

    # 获取提交哈希
    local commit_hash=$(git rev-parse --short HEAD)

    # 生成版本号（基于日期和时间）
    local version="v$(date +%Y%m%d-%H%M%S)"

    # 创建 tag
    git tag -a "$version" -m "$message"

    echo -e "\n${GREEN}✅ 版本已保存！${NC}"
    echo -e "   Commit: ${commit_hash}"
    echo -e "   Tag: ${version}"
    echo -e "\n${BLUE}回滚命令:${NC}"
    echo -e "   $0 rollback $version"
}

# 列出所有版本
list_versions() {
    echo -e "${BLUE}📋 所有版本标签:${NC}\n"

    if ! git tag -l | grep -q .; then
        echo -e "${YELLOW}暂无版本标签${NC}"
        return
    fi

    git tag -l --sort=-version:refname | while read tag; do
        local date=$(git log -1 --format=%ai "$tag" | cut -d' ' -f1,2)
        local message=$(git tag -l --format='%(contents:subject)' "$tag")
        echo -e "${GREEN}$tag${NC} - $date"
        echo -e "  $message"
        echo ""
    done
}

# 显示版本详情
show_version() {
    local tag="$1"
    if [ -z "$tag" ]; then
        echo -e "${RED}错误: 请指定版本标签${NC}"
        echo "用法: $0 show <tag>"
        exit 1
    fi

    if ! git rev-parse "$tag" >/dev/null 2>&1; then
        echo -e "${RED}错误: 版本标签 '$tag' 不存在${NC}"
        exit 1
    fi

    echo -e "${BLUE}📦 版本详情: $tag${NC}\n"
    git show "$tag" --stat
}

# 回滚到指定版本
rollback_version() {
    local tag="$1"
    if [ -z "$tag" ]; then
        echo -e "${RED}错误: 请指定版本标签${NC}"
        echo "用法: $0 rollback <tag>"
        exit 1
    fi

    if ! git rev-parse "$tag" >/dev/null 2>&1; then
        echo -e "${RED}错误: 版本标签 '$tag' 不存在${NC}"
        exit 1
    fi

    echo -e "${YELLOW}⚠️  警告: 这将丢弃所有未提交的更改！${NC}"
    echo -e "即将回滚到版本: ${GREEN}$tag${NC}"
    echo -n "确认继续? (yes/no): "
    read -r confirm

    if [ "$confirm" != "yes" ]; then
        echo -e "${BLUE}已取消${NC}"
        exit 0
    fi

    echo -e "\n${BLUE}🔄 回滚中...${NC}"

    # 创建备份分支
    local backup_branch="backup-$(date +%Y%m%d-%H%M%S)"
    git branch "$backup_branch" 2>/dev/null || true
    echo -e "   已创建备份分支: $backup_branch"

    # 回滚到指定版本
    git reset --hard "$tag"

    echo -e "\n${GREEN}✅ 已回滚到版本 $tag${NC}"
    echo -e "\n${BLUE}如需恢复，可以使用:${NC}"
    echo -e "   git checkout $backup_branch"
}

# 比较两个版本
diff_versions() {
    local tag1="$1"
    local tag2="$2"

    if [ -z "$tag1" ] || [ -z "$tag2" ]; then
        echo -e "${RED}错误: 请指定两个版本标签${NC}"
        echo "用法: $0 diff <tag1> <tag2>"
        exit 1
    fi

    if ! git rev-parse "$tag1" >/dev/null 2>&1; then
        echo -e "${RED}错误: 版本标签 '$tag1' 不存在${NC}"
        exit 1
    fi

    if ! git rev-parse "$tag2" >/dev/null 2>&1; then
        echo -e "${RED}错误: 版本标签 '$tag2' 不存在${NC}"
        exit 1
    fi

    echo -e "${BLUE}📊 版本差异: $tag1 → $tag2${NC}\n"
    git diff "$tag1".."$tag2" --stat
    echo -e "\n${BLUE}详细差异:${NC}"
    git log "$tag1".."$tag2" --oneline
}

# 显示当前状态
show_status() {
    echo -e "${BLUE}📊 当前状态${NC}\n"

    echo -e "${GREEN}分支:${NC}"
    git branch --show-current

    echo -e "\n${GREEN}最近的提交:${NC}"
    git log -1 --oneline

    echo -e "\n${GREEN}最近的标签:${NC}"
    git describe --tags --abbrev=0 2>/dev/null || echo "无标签"

    echo -e "\n${GREEN}工作区状态:${NC}"
    if git diff --quiet && git diff --cached --quiet; then
        echo -e "${GREEN}✓ 工作区干净${NC}"
    else
        git status --short
    fi
}

# 创建备份
create_backup() {
    echo -e "${BLUE}📦 创建备份...${NC}"

    local backup_dir="$PROJECT_DIR/backup"
    mkdir -p "$backup_dir"

    local backup_file="$backup_dir/$(date +%Y%m%d-%H%M%S).tar.gz"

    tar -czf "$backup_file" \
        --exclude='target' \
        --exclude='backup' \
        --exclude='.git' \
        --exclude='*.png' \
        --exclude='Screenshots' \
        src/ \
        Cargo.toml \
        Cargo.lock \
        README*.md \
        QUICKSTART*.md \
        *.md \
        scripts/ \
        tests/ \
        *.sh 2>/dev/null || true

    local size=$(ls -lh "$backup_file" | awk '{print $5}')
    echo -e "${GREEN}✅ 备份已创建${NC}"
    echo -e "   文件: $backup_file"
    echo -e "   大小: $size"
}

# 主函数
main() {
    if [ $# -eq 0 ]; then
        show_help
        exit 0
    fi

    local command="$1"
    shift

    case "$command" in
        save)
            save_version "$@"
            ;;
        list)
            list_versions
            ;;
        show)
            show_version "$@"
            ;;
        rollback)
            rollback_version "$@"
            ;;
        diff)
            diff_versions "$@"
            ;;
        status)
            show_status
            ;;
        backup)
            create_backup
            ;;
        help|--help|-h)
            show_help
            ;;
        *)
            echo -e "${RED}错误: 未知命令 '$command'${NC}"
            echo ""
            show_help
            exit 1
            ;;
    esac
}

main "$@"
