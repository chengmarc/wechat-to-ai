---
name: wechat
description: 在本地查询、导出微信 4.1+ 聊天记录为 AI 可读文本 —— 支持双人会话、群聊、以及按消息量排名的重要联系人排名。当用户想导出、搜索或分析自己的微信聊天记录，或为其他 skill（前任.skill / 同事.skill 等）准备聊天记录原材料时使用。
---

# 微信聊天记录导出

把微信 4.1+ 的聊天记录构建为本地可读数据库，再导出为 AI 可读文本。全程本机运行，数据不离机。
所有功能都是插件自带二进制 `dist\core.exe` 的子命令；完整参数用 `<命令> --help` 查看。

> **所有命令都是 PowerShell。** 若第一条命令被权限提示拦住，**不要**改用 Bash 重写（`&` 与 `$env:`
> 在 Bash 下不成立），而是告诉用户：「这个插件的命令要用 PowerShell 跑，请在权限提示里选『允许』
> （有『始终允许』就选它）。」获准后从第 1 步重新开始。

## 1. 前置检查（每次会话第一步）

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" preflight
```

看末尾的 `NEXT_STEP=`：

| `NEXT_STEP` | 做什么 |
|---|---|
| `ready` / `setup` | 进入第 2 步 |
| `reinstall` | 插件文件缺失（多半被杀毒软件删了）：请用户放行后重新安装插件 |

## 2. 准备数据（每个导出请求前必跑）

**每个导出请求开始前都先跑一次 `setup`，即使 `NEXT_STEP=ready`**：数据库是快照，
不重建就没有新消息。同一请求里连续导出多个对象，开头跑一次即可。

`setup` 从已登录的微信进程里现场恢复账号方案并重建数据库；方案只在内存里、不落盘，所以需要微信在运行且已登录
（无需管理员权限）。先告诉用户：

> 我先刷新一下数据：从你已登录的微信里重建可读数据库。保持微信登录即可，无需操作，稍等片刻。

然后**后台运行**（`run_in_background`），等完成通知：

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" setup
```

| 最后一行 `SETUP_RESULT=` | 处理 |
|---|---|
| `done` | 重跑 preflight，确认 `NEXT_STEP=ready` 后进入第 3 步 |
| `not_running` | 请用户打开电脑版微信并登录，完成后重跑 `setup` |
| `no_key` | 微信多半没登录：请用户确认已登录后重跑 `setup` |
| `no_db_dir` | 找不到微信数据目录：请用户在 微信设置 → 文件管理 找到 `...\xwechat_files\<wxid>\db_storage`，设为环境变量 `WECHAT_DB_DIR` 后重跑 |
| `build_failed` | 转述输出里「导出所需的库构建失败」那一行，请用户保持微信登录后重跑 `setup` |
| `error` | 插件不完整：请用户重新安装插件 |

没拿到 `done` 时**不要**用旧数据导出，除非用户明确同意。杀毒软件（如 Windows Defender）可能报警，需要用户手动放行。

## 3. 按意图导出

| 用户想要… | 流程 |
|---|---|
| 与某个人的双人聊天 | A |
| 某个群的聊天 | B（要做成发回微信群的中文总结，导出后接 `wechat-summary` skill） |
| 和谁聊得最多 / 联系人排名 | C |
| 只给了一个名字，没说是人还是群 / 只想看有哪些匹配 | 先 `find`（见下） |

时间参数（A、B）：**不加时间参数 = 最近 1 天**（昨天 0 点至今）。起点三选一：`--days N` ｜ `--since YYYY-MM-DD` ｜
`--all`（全部历史，用户要「全部」「完整」「所有」聊天记录时用）；`--until YYYY-MM-DD`（**包含当天**）可与任一组合。
时区默认 GMT+8，用 `--tz` 调整。`--threshold 秒数`：间隔超过它就另起一个时间段标题（默认 3600）。

两种导出的发送者都直接取自微信的记录：用户自己为 `【用户】`，系统消息（撤回、拍一拍等）为 `【系统】`；
双人会话的对方为 `【对方】`，群成员显示昵称，无法识别的为 `【?】`。

### A. 双人会话 / B. 群聊

`<关键词>` 取用户对此人 / 此群的称呼（匹配备注名或微信昵称的任一部分）：

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" export_private <关键词> [时间参数]    # A
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" export_chatroom <关键词> [时间参数]   # B
```

导出命令自己把聊天内容写进文件，屏幕上只有进度和结果行。按最后一行 `EXPORT_RESULT=` 处理：

| `EXPORT_RESULT=` | 处理 |
|---|---|
| `done` | 完成：把 `OUTPUT=` 的文件路径告诉用户 |
| `ambiguous` | 输出列出了全部匹配 `[序号]`：把名称、昵称 / 备注和消息分布列给用户选，再在**同一条命令**末尾加 `--pick <序号>` 重跑 |
| `not_found` | 告诉用户没找到，请换一个关键词（备注名或昵称的一部分） |
| `empty` | 告诉用户该会话在该时间段内没有消息 |

#### 先 `find`

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" find <关键词>
```

同时搜联系人和群聊，分两段列出匹配及消息分布，段内 `[序号]` 即对应导出命令的 `--pick`。按末尾两行处理：

| `PRIVATE_MATCHES=` / `CHATROOM_MATCHES=` | 处理 |
|---|---|
| 都是 0 | 同 `not_found` |
| 只有一个非 0 | 用该段标题里的导出命令（A 或 B）按上面流程导出 |
| 都非 0 | 把两段的名称列给用户选，再用所选段的导出命令加 `--pick <序号>` 导出 |

### C. 联系人排名

```powershell
& "$env:CLAUDE_PLUGIN_ROOT\dist\core.exe" export_contacts
```

`--threshold N`（最低消息数，默认 50）｜ `--include-chatrooms`（同时列出群聊）。
结果行同上：`done` → 告诉用户 `OUTPUT=` 的路径；`empty` → 没有达到阈值的联系人。

## 硬性规则

- 导出命令**不要**加重定向（`>`、`2>`、`2>&1`），也不要传 `--out`：文件路径由命令自己决定并打印在 `OUTPUT=`。
- 导出文件不要读进上下文，除非下一步（如 `wechat-summary`）需要它的内容。
- 准备 / 刷新数据一律用后台 `setup`。
