# 参考：命令与输出

`wechat` skill 的操作参考。这里只保留如何运行与如何理解输出。实现细节在 `src/` 内部文档。

## 路径约定

`preflight` 结束时会打印这些路径：

- `CORE_HOME=<路径>`：运行时数据根目录
- `BUILT_DB_DIR=<路径>`：生成后的数据库目录
- `ARTIFACTS_DIR=<路径>`：导出文本和中间映射文件目录

常见文件：

- `<BUILT_DB_DIR>\contact\contact.db`
- `<BUILT_DB_DIR>\message\message_*.db`
- `<ARTIFACTS_DIR>\sender_map_<名字>.json`
- `<ARTIFACTS_DIR>\id_map_<群名>.json`
- `<ARTIFACTS_DIR>\chat_<名字>.txt`
- `<ARTIFACTS_DIR>\contacts.txt`

## 查找命令

### `find_private` / `find_chatroom` / `find_contact`

| 参数 | 必填 | 说明 |
|------|:----:|------|
| `keyword` | ✅ | 搜索关键词（匹配 `nick_name` / `remark`） |
| `--built-db-dir` | | 数据库根目录；缺省读环境变量 `WECHAT_BUILT_DB_DIR`，否则使用 `preflight` 打印的 `BUILT_DB_DIR` |
| `--contact-db` | | 覆盖 `contact.db` 路径（默认由 `--built-db-dir` 推导） |
| `--msg-dbs` | | 消息库路径列表（默认由 `--built-db-dir` glob `message_*.db`） |
| `--tz` | | 时区偏移小时数，默认 8 |

`find_*` 输出可正常进入上下文。

查找结果里会包含：

- 联系人或群名
- 消息分布
- 可直接复制执行的导出命令

## 导出命令

### `export_private` / `export_chatroom`

| 参数 | 必填 | 说明 |
|------|:----:|------|
| `--db` | ✅ | `message_N.db` 路径（`export_private` 可传多个，`export_chatroom` 只接受单个） |
| `--table` | ✅ | 消息表名，如 `Msg_xxxx` |
| `--days` | | 最近 N 天；与 `--since` 互斥 |
| `--since` | | 起始日期 `YYYY-MM-DD` |
| `--until` | | 截止日期 `YYYY-MM-DD`（包含当天），默认当前时间 |
| `--threshold` | | 时段阈值（秒），默认 3600 |
| `--tz` | | 时区偏移小时数，默认 8 |

`export_private` 专有：`--sender-map`（✅，`sender_map.json` 路径；不存在时自动推断写出后退出供核对）。

`export_chatroom` 专有：`--id-map`（✅，`id_map.json` 路径，存 wxid→昵称）、`--contact-db`（提供时自动从消息表提取 wxid 生成 / 更新 id_map）。

### `export_contacts`

| 参数 | 必填 | 说明 |
|------|:----:|------|
| `--contact-db` | ✅ | `contact.db` 路径 |
| `--msg-dbs` | ✅ | 消息库路径列表（支持显式路径或 glob；脚本内部会展开，建议传全部库） |
| `--threshold` | | 最低消息数阈值，默认 50 |
| `--include-chatrooms` | | 同时输出群聊 |

## 输出判断

- `sender_map` 需要你核对“用户 / 对方”是否分配正确
- 群聊中出现 `【?】` 表示该发送者未被识别
- 没查到消息时，优先确认是否已经重新执行过 Step 0 更新数据
- 时间范围默认用本地时区，默认 `GMT+8`

**`export_*` 的 stdout + stderr 必须一并重定向到文件**（`> "<ARTIFACTS_DIR>\xxx.txt" 2>&1`），不得进入上下文窗口。
