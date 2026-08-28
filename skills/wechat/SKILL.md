---
name: wechat
description: 在本地查询、导出微信 4.1+ 聊天记录为 AI 可读文本 —— 支持双人会话、群聊、以及按消息量排名的重要联系人扫描。当用户想导出、搜索或分析自己的微信聊天记录，或为其他 skill（前任.skill / 同事.skill 等）准备聊天记录原材料时使用。
---

# 微信聊天记录导出

在本地把微信 4.1+ 的聊天记录构建为可查询、可导出的 AI 可读文本。所有操作在本机运行，数据不离机。

所有功能都是密封二进制的子命令（`preflight` / `find_*` / `export_*` / `make_json` / `make_db`）。操作参考见 `reference/db-schema.md`。

## 前置检查（每次会话第一步）

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" preflight
```

- 结尾打印 `READY` 和 `BUILT_DB_DIR=<路径>`：记住该路径，下文所有 `<BUILT_DB_DIR>` 都用它。
- 同时打印 `ARTIFACTS_DIR=<路径>`：记住该路径，下文所有 `<ARTIFACTS_DIR>` 都用它。
- 同时打印 `BUILD_DB_CMD=<命令>`：Step 0 直接用它，路径/环境变量都已拼好。
- 结尾打印 `NOT READY`：按脚本逐条提示处理（缺依赖 / 缺构建核心 / 需构建数据库），处理完重跑。

### Step 0：构建数据库 / 更新数据

首次必做；之后每次想拿到最新消息也要重跑。微信保持登录，**以管理员权限**运行 preflight 打印的 `BUILD_DB_CMD`。不要手写环境变量格式，直接复制 preflight 的输出。

> 首次使用且无方案文件时，先生成方案清单。**微信必须已登录运行**，且以**管理员权限**执行：
> ```powershell
> $env:WECHAT_CORE_HOME = "<CORE_HOME>"
> & "$env:CLAUDE_PLUGIN_ROOT\_core.exe" make_json
> ```
> 运行后**在微信里逐个点开聊天（新旧都开）、联系人、收藏、朋友圈并搜索**——每个库的
> 数据准备只会在相关内容被打开时逐步覆盖，浏览得越全，抓到的库越全。它会实时打印 `[SOLUTION] <库名>`，
> 覆盖够了或 180 秒后自动写出 `solution.json`（落在 `<CORE_HOME>`）。
> `<CORE_HOME>` 用 preflight 打印的 `CORE_HOME=` 值；`solution.json` 与 `built_db/` 都落在这里。
> Windows Defender 可能对 `make_json` 报警，需手动放行。

## 按意图选择流程

| 用户想要… | 用哪个流程 |
|---|---|
| 导出 / 搜索与某个人的双人聊天 | **A. 双人会话** |
| 导出 / 总结某个群 | **B. 群聊** |
| 知道"和谁聊得最多" / 重要联系人排名 | **C. 联系人扫描** |

导出结果若要做成规避微信折叠的中文总结，接着用 `wechat-summary` skill。

---

## A. 双人会话

### A1 — 查找联系人 + 生成 sender_map

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" find_private <关键词> --built-db-dir "<BUILT_DB_DIR>"
```

输出：wxid、消息表名、各库消息量与日期范围、sender_map 推断结果（含采样消息），以及一条即用的 `export_private` 命令。

### A2 — 与用户核对 sender_map

find_private 会为每个库打印推断出的「用户 / 对方」及其样本消息。**用自然语言和用户确认**，例如：

> 我从消息内容推断：`my_id=2`（样本「晚点到」「在开会」）是你，`other_id=7`（样本「好的」「路上注意」）是对方。对吗？

- 用户确认 → 进入 A3。
- 用户说反了 → 编辑对应的 `<ARTIFACTS_DIR>\sender_map_<名字>.json`，把该库条目的 `my_id` 与 `other_id` 对调，再进入 A3。

### A3 — 运行导出命令

stdout + stderr 必须一起重定向到文件，**绝不能让内容进入上下文窗口**：

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" export_private `
  --db <各库路径…> --table <Msg_...> --sender-map "<ARTIFACTS_DIR>\sender_map_<名字>.json" `
  > "<ARTIFACTS_DIR>\chat_<名字>.txt" 2>&1
```

优先直接复制 `find_private` 打印的完整导出命令；其中已带绝对路径。

时间范围（默认全量）：`--days N` ｜ `--since YYYY-MM-DD` ｜ `--since … --until YYYY-MM-DD`。其中 `--until` **包含当天**。

---

## B. 群聊

群聊导出会自动处理发送者映射；少数无法识别的发送者会显示为 `【?】`。

### B1 — 查找群

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" find_chatroom <关键词> --built-db-dir "<BUILT_DB_DIR>"
```

### B2 — 运行导出命令（默认最近 1 天）

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" export_chatroom `
  --db <单个库路径> --table <Msg_...> `
  --contact-db "<BUILT_DB_DIR>\contact\contact.db" `
  --id-map "<ARTIFACTS_DIR>\id_map_<群名>.json" `
  > "<ARTIFACTS_DIR>\chat_<群名>.txt" 2>&1
```

范围：`--days N` ｜ `--since` ｜ `--until`。其中 `--until` **包含当天**。`--contact-db` 存在时自动生成 / 更新 id_map，后续导出可省略该参数。

---

## C. 联系人扫描

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" find_contact <关键词> --built-db-dir "<BUILT_DB_DIR>"

& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" export_contacts `
  --contact-db "<BUILT_DB_DIR>\contact\contact.db" `
  --msg-dbs "<BUILT_DB_DIR>\message\message_*.db" `
  > "<ARTIFACTS_DIR>\contacts.txt" 2>&1
```

`--threshold N`（最低消息数，默认 50）｜ `--include-chatrooms`（同时列出群聊）。

---

## 硬性规则

- `export_*` 的输出**必须** `> "<ARTIFACTS_DIR>\xxx.txt" 2>&1`，禁止裸跑让内容进上下文。
- `find_*` 的输出可以正常读取、展示给用户。
- 所有子命令用 `& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" <命令>` 调用。
- 不要手写相对 `output\` 路径；优先直接复制 `preflight` / `find_*` 打印出的完整命令。
- 时区默认 GMT+8，`--tz` 调整。
