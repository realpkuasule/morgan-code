#!/bin/bash

# Morgan Code - 一键启动脚本
# 自动检查环境、编译项目并启动聊天

set -e

# 颜色定义
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# 项目根目录
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$PROJECT_ROOT"

echo -e "${BLUE}🚀 Morgan Code 启动脚本${NC}"
echo "================================"

# 检查 Rust 环境
echo -e "\n${YELLOW}[1/4]${NC} 检查 Rust 环境..."
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ 未找到 Cargo，请先安装 Rust${NC}"
    echo "访问: https://rustup.rs/"
    exit 1
fi
echo -e "${GREEN}✓${NC} Rust 环境正常"

# 检查 API Key
echo -e "\n${YELLOW}[2/4]${NC} 检查 API Key..."
if [ -z "$DEEPSEEK_API_KEY" ]; then
    echo -e "${YELLOW}⚠️  未设置 DEEPSEEK_API_KEY${NC}"
    echo "请设置环境变量："
    echo "  export DEEPSEEK_API_KEY=your-api-key"
    echo ""
    read -p "是否继续？(y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
else
    echo -e "${GREEN}✓${NC} DEEPSEEK_API_KEY 已设置"
fi

# 编译项目
echo -e "\n${YELLOW}[3/4]${NC} 编译项目..."
if [ ! -f "target/release/morgan" ] || [ "$1" == "--rebuild" ]; then
    echo "正在编译 release 版本..."
    cargo build --release
    echo -e "${GREEN}✓${NC} 编译完成"
else
    echo -e "${GREEN}✓${NC} 使用已有的二进制文件（使用 --rebuild 强制重新编译）"
fi

# 初始化配置
echo -e "\n${YELLOW}[4/4]${NC} 检查配置..."
if [ ! -f "$HOME/.morgan-code/config.toml" ]; then
    echo "初始化配置文件..."
    ./target/release/morgan init
    echo -e "${GREEN}✓${NC} 配置文件已创建"
else
    echo -e "${GREEN}✓${NC} 配置文件已存在"
fi

# 显示配置信息
echo -e "\n${BLUE}📋 当前配置：${NC}"
./target/release/morgan config

# 启动聊天
echo -e "\n${GREEN}================================${NC}"
echo -e "${GREEN}✨ 启动 Morgan Code 聊天界面${NC}"
echo -e "${GREEN}================================${NC}"
echo ""
echo "提示："
echo "  - 输入 'clear' 清空对话历史"
echo "  - 输入 'exit' 或 'quit' 退出"
echo ""

exec ./target/release/morgan chat
