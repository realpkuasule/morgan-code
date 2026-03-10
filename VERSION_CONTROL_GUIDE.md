# Morgan Code 版本控制系统使用指南

## 概述

Morgan Code 使用 **Git + 自动化脚本** 实现强大的版本控制系统，支持一键保存、回滚、查看历史等功能。

## 为什么选择 Git？

相比 tar.gz 备份，Git 提供：

✅ **增量存储** - 只保存变化部分，节省空间
✅ **完整历史** - 记录每次修改的详细信息
✅ **快速回滚** - 一键恢复到任意历史版本
✅ **分支管理** - 支持并行开发多个功能
✅ **差异对比** - 清晰查看版本间的变化
✅ **安全备份** - 自动创建备份分支防止误操作

## 快速开始

### 1. 保存当前版本

```bash
./vc save "描述你的更改"
```

示例：
```bash
./vc save "修复工具调用bug"
./vc save "添加新的配置选项"
./vc save "优化性能"
```

这会自动：
- 创建 Git commit
- 生成带时间戳的版本标签（如 `v20260311-062045`）
- 显示回滚命令

### 2. 查看所有版本

```bash
./vc list
```

输出示例：
```
📋 所有版本标签:

v20260311-062045 - 2026-03-11 06:20:45
  添加版本控制工具和快捷命令

v0.2.0-streaming - 2026-03-10 15:02:06
  Release v0.2.0: Streaming Response Implementation
```

### 3. 查看当前状态

```bash
./vc status
```

显示：
- 当前分支
- 最近的提交
- 最近的标签
- 工作区状态（是否有未提交的更改）

### 4. 回滚到指定版本

```bash
./vc rollback <版本标签>
```

示例：
```bash
./vc rollback v0.2.0-streaming
```

⚠️ **安全机制**：
- 回滚前会要求确认（输入 `yes`）
- 自动创建备份分支（如 `backup-20260311-062045`）
- 如果后悔，可以恢复到备份分支

### 5. 查看版本详情

```bash
./vc show <版本标签>
```

示例：
```bash
./vc show v0.2.0-streaming
```

显示该版本的：
- 完整提交信息
- 修改的文件列表
- 代码统计

### 6. 比较两个版本

```bash
./vc diff <版本1> <版本2>
```

示例：
```bash
./vc diff v0.2.0-streaming v20260311-062045
```

显示：
- 文件变化统计
- 提交历史

### 7. 创建 tar.gz 备份

```bash
./vc backup
```

在 `backup/` 目录下创建带时间戳的压缩包，适合：
- 离线归档
- 发送给他人
- 额外的安全备份

## 常见使用场景

### 场景 1：开发新功能前保存当前状态

```bash
# 保存当前稳定版本
./vc save "稳定版本 - 开始开发新功能前"

# 开发新功能...
# 编辑代码

# 保存新功能
./vc save "添加新功能：XXX"
```

### 场景 2：新功能有问题，回滚到之前版本

```bash
# 查看历史版本
./vc list

# 回滚到稳定版本
./vc rollback v20260310-150000

# 确认输入 yes
```

### 场景 3：查看两个版本之间做了什么改动

```bash
./vc diff v20260310-150000 v20260311-062045
```

### 场景 4：定期创建备份

```bash
# 每天或每周创建一次备份
./vc backup

# 备份文件保存在 backup/ 目录
```

## 高级用法

### 使用 Git 原生命令

版本控制工具是对 Git 的封装，你仍然可以使用所有 Git 命令：

```bash
# 查看详细历史
git log --oneline --graph --all

# 查看某个文件的修改历史
git log -p src/main.rs

# 创建新分支
git checkout -b feature-new-tool

# 合并分支
git merge feature-new-tool

# 查看某次提交的详细内容
git show <commit-hash>
```

### 创建有意义的版本标签

除了自动生成的时间戳标签，你也可以手动创建语义化版本：

```bash
# 创建语义化版本标签
git tag -a v0.3.0 -m "Release v0.3.0: 添加新工具系统"

# 查看所有标签
git tag -l
```

### 恢复误删的文件

```bash
# 查看被删除的文件
git log --diff-filter=D --summary

# 恢复文件
git checkout <commit-hash>^ -- <file-path>
```

### 查看某个文件在特定版本的内容

```bash
git show v0.2.0-streaming:src/main.rs
```

## 版本命名规范

### 自动生成的版本（时间戳）
格式：`vYYYYMMDD-HHMMSS`
- 示例：`v20260311-062045`
- 用途：日常开发的快照

### 手动创建的版本（语义化）
格式：`vMAJOR.MINOR.PATCH`
- 示例：`v0.2.0-streaming`
- 用途：重要的里程碑版本

推荐策略：
- 日常开发使用 `./vc save` 自动生成时间戳版本
- 重要功能完成后手动创建语义化版本标签

## 目录结构

```
morgan-code/
├── .git/              # Git 仓库数据
├── backup/            # tar.gz 备份文件
│   ├── 20260310.tar.gz
│   └── 20260311-062045.tar.gz
├── scripts/
│   └── version-control.sh  # 版本控制脚本
├── vc                 # 快捷命令
└── src/               # 源代码
```

## 最佳实践

### 1. 频繁保存
```bash
# 每完成一个小功能就保存
./vc save "实现 XXX 功能"
```

### 2. 清晰的提交信息
```bash
# ✅ 好的提交信息
./vc save "修复流式响应中的工具调用解析错误"

# ❌ 不好的提交信息
./vc save "修复bug"
```

### 3. 重要操作前先保存
```bash
# 在进行重构、删除代码等操作前
./vc save "重构前的稳定版本"
```

### 4. 定期创建备份
```bash
# 每周创建一次 tar.gz 备份
./vc backup
```

### 5. 使用分支开发新功能
```bash
# 创建功能分支
git checkout -b feature-new-api

# 开发完成后合并
git checkout main
git merge feature-new-api
```

## 故障恢复

### 如果回滚后想恢复

```bash
# 查看备份分支
git branch | grep backup

# 切换到备份分支
git checkout backup-20260311-062045

# 如果确认要恢复，合并回 main
git checkout main
git merge backup-20260311-062045
```

### 如果误删了重要代码

```bash
# 查看最近的提交
git log --oneline

# 恢复到某个提交
git checkout <commit-hash> -- <file-path>
```

### 如果工作区很乱想重置

```bash
# 丢弃所有未提交的更改
git reset --hard HEAD

# 或者使用版本控制工具回滚到最近的标签
./vc rollback $(git describe --tags --abbrev=0)
```

## 与远程仓库同步

如果你有 GitHub/GitLab 等远程仓库：

```bash
# 推送所有提交和标签
git push origin main --tags

# 从远程拉取更新
git pull origin main

# 克隆到其他机器
git clone <repository-url>
```

## 总结

Morgan Code 的版本控制系统提供：

| 功能 | 命令 | 说明 |
|------|------|------|
| 保存版本 | `./vc save "message"` | 创建提交和标签 |
| 查看历史 | `./vc list` | 列出所有版本 |
| 查看状态 | `./vc status` | 显示当前状态 |
| 回滚版本 | `./vc rollback <tag>` | 恢复到指定版本 |
| 查看详情 | `./vc show <tag>` | 显示版本详细信息 |
| 比较版本 | `./vc diff <tag1> <tag2>` | 对比两个版本 |
| 创建备份 | `./vc backup` | 生成 tar.gz 备份 |

**核心优势**：
- ✅ 一键操作，简单易用
- ✅ 完整历史，永不丢失
- ✅ 快速回滚，安全可靠
- ✅ 自动备份，防止误操作
- ✅ 增量存储，节省空间

现在你可以放心地开发和实验，随时回滚到任何历史版本！
