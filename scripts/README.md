# Scripts 目录

这个目录包含了 Morgan Code 的实用脚本。

## 可用脚本

### start.sh
一键启动脚本，自动完成以下操作：
- 检查 Rust 环境
- 检查 API Key 配置
- 编译项目（如需要）
- 初始化配置文件
- 启动聊天界面

**使用方法：**
```bash
# 设置 API 密钥
export DEEPSEEK_API_KEY=your-api-key-here

# 运行脚本
./scripts/start.sh

# 强制重新编译
./scripts/start.sh --rebuild
```

**功能特性：**
- ✅ 自动环境检查
- ✅ 智能编译（仅在需要时编译）
- ✅ 友好的彩色输出
- ✅ 配置状态显示
- ✅ 错误提示和帮助信息
