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

> **本 skill 的每一条命令都是 PowerShell。** 全新的工作目录默认不带 PowerShell 执行权限，
> 第一条命令就会被权限提示拦住。这时**不要**改用 Bash 重写（`&` 调用与 `$env:` 语法在 Bash 下
> 不成立），也不要放弃，而是用自然语言告诉用户：
>
> > 这个插件的命令都要用 PowerShell 跑。请在弹出的权限提示里选「允许」——
> > 如果有「始终允许 / 不再询问」，选它就不会再被打断。也可以用 `/permissions` 管理。
>
> 得到许可后从 `preflight` 重新开始。

- 结尾打印 `READY` 和 `BUILT_DB_DIR=<路径>`：记住该路径，下文所有 `<BUILT_DB_DIR>` 都用它。
- 同时打印 `ARTIFACTS_DIR=<路径>`：记住该路径，下文所有 `<ARTIFACTS_DIR>` 都用它。
- 同时打印 `BUILD_DB_CMD=<命令>`：`make_db` 的命令内容，路径已拼好。**但不要直接跑**，见 Step 0b。
- 结尾打印 `NOT READY`：先确认 `<CORE_HOME>\solution.json` 是否存在 —— preflight **不检查**这个文件，
  直接跑 `BUILD_DB_CMD` 会以 `[ERROR] 方案文件不存在` 失败。不存在 → Step 0a；存在 → Step 0b。
  其余提示（缺依赖 / 缺构建核心）按脚本逐条处理，处理完重跑。

### Step 0：准备数据

两个子步骤，**都需要管理员权限**，而且**都必须用下面的「提权新窗口」方式启动**：

- `make_json` —— 生成方案清单 `solution.json`（首次、或方案失效时才需要）
- `make_db` —— 构建 / 更新数据库（首次必做；之后每次想拿到最新消息都要重跑）

其余子命令（`preflight` / `find_*` / `export_*`）**都不需要**管理员权限，正常运行即可。

#### 为什么必须开新窗口

不要在当前会话里直接跑 `make_json` / `make_db`：

- 两者都需要管理员权限，当前会话通常没有；
- 它们的输出在非终端环境下会被**完全缓冲**，你一个字节都读不到，看起来就是卡死；
- `make_json` **不会自己结束**，必须由用户在窗口里按 `Ctrl+C` 才写出 `solution.json`。

正确做法：弹一个提权 PowerShell 窗口，用户点一下「是」，然后在那个窗口里看进度、自己按 `Ctrl+C`。

#### Step 0a —— 生成方案清单（`solution.json` 不存在时）

**微信必须已登录运行。** 先用自然语言告诉用户「接下来会弹出一个 UAC 提权提示，请点『是』」，再执行：

```powershell
$inner = @'
$env:WECHAT_CORE_HOME='<CORE_HOME>'
& '<PLUGIN_ROOT>\_core.exe' make_json
'@
Start-Process powershell -Verb RunAs -ArgumentList '-NoExit','-NoProfile','-Command',$inner
```

`<CORE_HOME>` / `<PLUGIN_ROOT>` 必须替换成 preflight 打印的**绝对路径**。新窗口不继承当前会话的
环境变量，`$env:CLAUDE_PLUGIN_ROOT` 在里面是空的，**不要**照抄变量引用。

`Start-Process` 会立刻返回，这**不代表成功**。接着告诉用户：

> 弹出的窗口里会不断打印 `[SOLUTION] <库名>` 和 `已捕获 N/27`。请现在到微信里**逐个点开聊天
> （新旧都开）、通讯录、收藏、朋友圈，并用一次搜索** —— 每个库只有在相关内容被打开时才会被抓到。
> 等计数不再增长（至少要出现 `contact\contact.db` 和 `message\message_*.db`），
> 在那个窗口里按 **Ctrl+C**，就会写出 `solution.json`。

然后**等用户确认已按下 Ctrl+C**，再检查文件确实生成：

```powershell
Test-Path '<CORE_HOME>\solution.json'
```

若为 `False`：用户可能点了 UAC 的「否」，或还没按 Ctrl+C。问清楚再处理，不要盲目重跑。

#### Step 0b —— 构建 / 更新数据库

同样开提权窗口（内容即 preflight 打印的 `BUILD_DB_CMD`）：

```powershell
$inner = @'
$env:WECHAT_CORE_HOME='<CORE_HOME>'
& '<PLUGIN_ROOT>\_common.exe' make_db
'@
Start-Process powershell -Verb RunAs -ArgumentList '-NoExit','-NoProfile','-Command',$inner
```

`make_db` 会自己跑完，**不需要** Ctrl+C。跑完后重跑 `preflight`，应打印 `READY`。

> Windows Defender 等杀毒软件可能对 `make_json` / `make_db` 报警，需要用户手动放行。

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

stdout 与 stderr 必须**分别**重定向到文件，**绝不能让内容进入上下文窗口**：

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" export_private `
  --db <各库路径…> --table <Msg_...> --sender-map "<ARTIFACTS_DIR>\sender_map_<名字>.json" `
  > "<ARTIFACTS_DIR>\chat_<名字>.txt" 2> "<ARTIFACTS_DIR>\chat_<名字>.err.txt"
```

**不要用 `2>&1`。** Windows PowerShell 5.1 会把原生程序的 stderr 包装成 `NativeCommandError`
错误记录写进聊天文件开头（`At line:1 char:1` / `+ CategoryInfo …`），让导出结果看起来像报错。
分开重定向即可保持聊天文件干净，stderr 同样不进上下文。

> `find_private` 会打印一条「导出命令」：**参数部分可以复制，但开头的可执行文件路径不能用** ——
> 它指向 Nuitka onefile 的临时解包目录（`…\Temp\onefile_*\python.exe`），进程一退出就被删除，
> 照抄必定失败。开头一律换成 `& "$env:CLAUDE_PLUGIN_ROOT\_common.exe"`。

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
  > "<ARTIFACTS_DIR>\chat_<群名>.txt" 2> "<ARTIFACTS_DIR>\chat_<群名>.err.txt"
```

范围：`--days N` ｜ `--since` ｜ `--until`。其中 `--until` **包含当天**。`--contact-db` 存在时自动生成 / 更新 id_map，后续导出可省略该参数。

---

## C. 联系人扫描

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" find_contact <关键词> --built-db-dir "<BUILT_DB_DIR>"

& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" export_contacts `
  --contact-db "<BUILT_DB_DIR>\contact\contact.db" `
  --msg-dbs "<BUILT_DB_DIR>\message\message_*.db" `
  > "<ARTIFACTS_DIR>\contacts.txt" 2> "<ARTIFACTS_DIR>\contacts.err.txt"
```

`--threshold N`（最低消息数，默认 50）｜ `--include-chatrooms`（同时列出群聊）。

---

## 硬性规则

- `export_*` 的输出**必须**重定向到文件：`> "<ARTIFACTS_DIR>\xxx.txt" 2> "<ARTIFACTS_DIR>\xxx.err.txt"`，
  禁止裸跑让内容进上下文。**不要用 `2>&1`**，它会把 PowerShell 的 `NativeCommandError` 噪声写进聊天文件。
- `find_*` 的输出可以正常读取、展示给用户。
- `make_json` / `make_db` 必须用 Step 0 的提权新窗口启动，禁止在当前会话里直接运行（会因缺权限、输出缓冲、无法发送 Ctrl+C 而表现为静默卡死）。
- 子命令分布在两个二进制：`preflight` / `find_*` / `export_*` / `make_db` 在 `_common.exe`，**`make_json` 在 `_core.exe`**。当前会话里统一用 `& "$env:CLAUDE_PLUGIN_ROOT\_common.exe" <命令>` 调用。
- 不要手写相对 `output\` 路径；路径优先取 `preflight` / `find_*` 打印出的绝对路径。
  但 `find_*` 打印的导出命令，**开头那段 `…\Temp\onefile_*\python.exe` 是失效路径**，
  必须换成 `& "$env:CLAUDE_PLUGIN_ROOT\_common.exe"`。
- 时区默认 GMT+8，`--tz` 调整。
