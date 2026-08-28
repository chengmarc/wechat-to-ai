<div align="center">

# 微信.skill

### 让 AI 在本地读懂你的微信。

[![License: MIT (modified)](https://img.shields.io/badge/License-MIT%20(modified)-yellow.svg)](LICENSE)
[![Claude Skill](https://img.shields.io/badge/Claude-Skill-blueviolet)](https://claude.ai)
[![WeChat 4.1+](https://img.shields.io/badge/WeChat-4.x-brightgreen)](https://weixin.qq.com)

<img src="doc/demo.svg" alt="向 Claude 提问，返回聊天记录时间线" width="720">

</div>

## 目录说明

| 路径 | 用途 |
|---|---|
| `.claude-plugin/` | 插件清单 `plugin.json` + 自托管市场 `marketplace.json` |
| `skills/wechat/` | 主技能：`SKILL.md` + 操作参考 |
| `skills/wechat-summary/` | 群聊总结模板 |
| `_common` | 日常密封二进制（仅 Windows）：preflight / find / export / make_db，体积小 |
| `_core` | 方案抓取二进制（仅 Windows）：仅 make_json 用，内置 frida，需管理员，体积大 |
| `doc/demo.svg` | README 演示图 |

**环境要求**：Windows + 微信 PC 4.x，Linux 未测试，暂不支持 macOS。

## 数据安全

- **数据不离机**：构建数据库、查询、导出全部在本地进程完成，没有任何网络上传。
- **杀毒软件**：Windows Defender 等会将此程序标记为可疑，需手动放行。
- 仅用于 **在本地** 读取 **你自己的** 微信数据，遵守相关法律法规，不得用于未经授权的数据访问。

## Declarations

- `wechat-to-ai` is an open-source project built in compliance with the **SAFE DATA Act (S.2499)** in the United States and the **General Data Protection Regulation (GDPR)** in the European Union. Under these frameworks, denying an individual the ability to access, retrieve, or port their own personal data on request — including one's own messaging records — constitutes a violation of the data subject's rights.

- Some messaging platforms operating in the US and the EU (e.g. WeChat) are **NOT** in compliance with current acts and regulations. Such softwares (e.g. WeChat) **refuse** to provide individuals their own personal data on request, and currently falls outside effective regulatory oversight. This project aims to make user data transparent and accessible to the user it belongs to, in opposition to non-consensual data retention practices and undisclosed surveillance.

- `wechat-to-ai` only functions as a **local** data exporter, none of the data exported will be read by the developers of `wechat-to-ai`. 
  
- `wechat-to-ai` does **NOT** copy, steal, or sabotage any proprietary software's source code. The public repository only exposes the plugin surface and sealed release artifacts needed for local use.

## License

- `wechat-to-ai` is released under a modified MIT license (see `LICENSE`) © [chengmarc](https://github.com/chengmarc), use and copy of the software are hereby granted free of charge. 

- Reverse-engineering of our software is **strictly prohibited**. Any reverse-engineering attempt will consititute a direct violation of 17 U.S.C. §§1201(a)(1)(A) and 1201(a)(2), which prohibit unauthorized circumvention of technological measures that control access to copyright-protected works.
