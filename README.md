<div align="center">

# 微信.skill

### 让 AI 在本地读懂你的微信。

[![License: Source-Available](https://img.shields.io/badge/License-Source--Available-yellow.svg)](LICENSE)
[![Claude Skill](https://img.shields.io/badge/Claude-Skill-blueviolet)](https://claude.ai)
![WeChat 4.1+](https://img.shields.io/badge/WeChat-4.1%2B-brightgreen)

<img src="doc/demo.svg" alt="向 Claude 提问，返回聊天记录时间线" width="720">

</div>

## 目录说明

| 路径 | 用途 |
|---|---|
| `.claude-plugin/` | 插件清单 `plugin.json` + 自托管市场 `marketplace.json` |
| `skills/wechat/` | 主技能：`SKILL.md` + 操作参考 |
| `skills/wechat-summary/` | 群聊总结模板 |
| `_common.exe` | 通用构建工具，体积小 |
| `_core.exe` | 核心构建工具，体积大，需管理员 |
| `doc/demo.svg` | README 演示图 |

**环境要求**：Windows + 微信 PC 4.1+，Linux 未测试，暂不支持 macOS。

## 数据安全

- **数据不离机**：构建数据库、查询、导出全部在本地进程完成，没有任何网络上传。
- **杀毒软件**：Windows Defender 等会将此程序标记为可疑，需手动放行。
- 仅用于 **在本地** 读取 **你自己的** 微信数据，遵守相关法律法规，不得用于未经授权的数据访问。

---

# Declarations

*This statement is made by the maintainer of `wechat-to-ai` and forms part of the
documentation of this project. Defined terms have the meanings given in `LICENSE`.*

### §1 — Nature and operation of the Software

1.1 `wechat-to-ai` is a **local-only** data-portability utility. Database
construction, query, and export are performed entirely within processes on the
operator's own machine.

1.2 The Software performs **no network transmission of Operator Data**. It
contains no telemetry, no analytics, no crash reporting, and no update channel.

1.3 The maintainer **has no technical means of access** to any data the Software
produces. No data is received, relayed, stored, or processed by the maintainer at
any time.

1.4 The Software reads only data already resident on the operator's own device,
belonging to the operator's own account. It is not a network tool, not a remote
access tool, and has no capability to reach any account other than the one signed
in on the local machine.

### §2 — Purpose and legal basis

2.1 The right of a natural person to obtain a copy of their own personal data is
recognised in every jurisdiction in which this project is offered, including
Articles 15 and 20 of Regulation (EU) 2016/679 (**GDPR**), Principle 9 (Individual
Access) of Schedule 1 to the **Personal Information Protection and Electronic
Documents Act**, S.C. 2000, c. 5 (Canada), and Title 1798.100 *et seq.* of the
**California Consumer Privacy Act**.

2.2 This project exists because that right is, in the case of the platform
concerned, unavailable in practice. The Software supplies by local means what the
platform declines to supply on request: a portable, machine-readable copy of the
operator's own communications records.

2.3 Nothing in this section is asserted as authorisation for any act. It states
the purpose for which the Software is furnished. Lawfulness of use in any given
jurisdiction rests with the operator, per §6.2 of `LICENSE`.

### §3 — Findings of record concerning the WeChat platform

*The following are matters of published record by named institutions. They are
reproduced here, with citation, to document the conditions that make a local
data-portability tool necessary. Each is the finding of its author and is
attributed as such.*

| # | Source | Finding |
|---|---|---|
| 3.1 | **The Citizen Lab**, Munk School of Global Affairs & Public Policy, University of Toronto — *We Chat, They Watch* (Report No. 127, May 2020) | Documents and images transmitted **entirely among non-China-registered accounts** are subject to content surveillance, and that content is used to train and build up the censorship system applied to China-registered users. |
| 3.2 | **The Citizen Lab** — *Should We Chat, Too? Security Analysis of WeChat's MMTLS Encryption Protocol* (15 October 2024) | "The majority of network data sent by WeChat is not forward-secret between connections." The legacy business-layer encryption "does not encrypt metadata such as the user ID and request URI," transmitting them in plaintext. The report further records that "WeChat servers can certainly see chat messages since WeChat can censor them according to their content." |
| 3.3 | **Government of Canada**, Treasury Board Secretariat (30 October 2023) | The Chief Information Officer of Canada determined that WeChat presents "an unacceptable level of risk to privacy and security," and that "the data collection methods of these applications provide considerable access to the device's contents." WeChat was removed from all government-issued mobile devices. |
| 3.4 | **Directly verifiable by any user** | The platform provides no facility by which a user may obtain a complete, machine-readable copy of their own message history. |

3.5 Taken together, the record describes a platform that routes its users'
communications through servers able to read their contents; that applies political
content analysis to files exchanged entirely between users situated outside the
People's Republic of China, and uses those files to train a censorship system
applied to others; that has been assessed by a G7 government as presenting an
unacceptable level of risk to privacy and security; and that affords those same
users no means of retrieving their own records.

3.6 The maintainer takes no position beyond the record cited. No characterisation
in this section extends past what its cited source states.

**Sources:** [3.1](https://citizenlab.ca/research/we-chat-they-watch/) ·
[3.2](https://citizenlab.ca/2024/10/should-we-chat-too-security-analysis-of-wechats-mmtls-encryption-protocol/) ·
[3.3](https://www.canada.ca/en/treasury-board-secretariat/news/2023/10/minister-anand-announces-a-ban-on-the-use-of-wechat-and-kaspersky-suite-of-applications-on-government-mobile-devices.html)

### §4 — Jurisdiction, governing law, and service

4.1 Copyright in the Software is held by a natural person **resident in Canada**.
No entity organised under, or subject to, the laws of the People's Republic of
China holds any interest in, or exercises any control over, this project.

4.2 `LICENSE` is governed by the laws of the **Province of Ontario and the federal
laws of Canada** applicable therein. The courts of Ontario have exclusive
jurisdiction over any dispute arising from it (`LICENSE` §12).

4.3 This repository is hosted in the United States by GitHub, Inc., a subsidiary
of Microsoft Corporation, and is subject to that provider's terms and to United
States law.

4.4 The Software is not offered for distribution within the People's Republic of
China, incorporates no component licensed under the laws of that jurisdiction, and
is not represented as compliant with its requirements.

4.5 **Service of process.** The maintainer does not accept service by electronic
mail, repository issue, pull request, or any other informal channel. Service upon
a party resident in Canada must be effected in accordance with the *Convention of
15 November 1965 on the Service Abroad of Judicial and Extrajudicial Documents in
Civil or Commercial Matters* (Hague Service Convention), to which both Canada and
the People's Republic of China are contracting parties, and the Ontario *Rules of
Civil Procedure*. Correspondence must be in English.

### §5 — Non-affiliation and trademarks

5.1 This project is **not affiliated with, endorsed by, sponsored by, or in any
way officially connected to** Tencent Holdings Limited, Shenzhen Tencent Computer
Systems Company Limited, or any of their subsidiaries or affiliates.

5.2 "WeChat", "Weixin", and "微信" are trademarks of their respective owners. All
references to those marks are **nominative** — made solely to identify the data
format with which the Software interoperates — and assert no origin, association,
sponsorship, or endorsement.

5.3 This project contains **no source code, object code, asset, or other
copyrightable material** owned by Tencent Holdings Limited or any of its
affiliates. It does not modify, patch, redistribute, or interfere with the
operation of any third-party application.

### §6 — Scope of permitted use

6.1 The Software is furnished for use **on the operator's own machine**, against
**the operator's own account data**, in compliance with applicable law.

6.2 Use against any account, device, or data store that the operator is not
authorised to access is prohibited, falls outside the licence grant, and
terminates it (`LICENSE` §4.3, §11.2).

### §7 — Notice to complainants

7.1 The maintainer will consider any good-faith notice of claimed infringement or
circumvention on its merits, and will act promptly where a claim is substantiated.

7.2 Notices concerning claimed circumvention under 17 U.S.C. § 1201 are evaluated
under the host provider's published anti-circumvention policy, which provides for
technical and legal review and for an opportunity to remediate before any
disabling action.

7.3 Where the maintainer has a good-faith belief that material has been disabled
as a result of mistake or misidentification, a **counter-notice will be filed**
under 17 U.S.C. § 512(g).

7.4 **All takedown notices received in respect of this repository will be
published in full**, redacted only as to the personal contact details of natural
persons, in `doc/notices/`. Filing a notice constitutes acknowledgement of this
practice.

---

## License

`wechat-to-ai` is released under the **wechat-to-ai Source-Available License,
Version 1.0** (`LicenseRef-wechat-to-ai-1.0`) — see [`LICENSE`](LICENSE) —
© 2026 [chengmarc](https://github.com/chengmarc).

The licence is derived from the MIT License and retains its permissive grant to
use, copy, modify, publish, distribute, and sell. It is **not** the OSI-approved
MIT License and must not be catalogued as "MIT".

It adds, in summary: attribution and notice-retention conditions (§3); a
restriction on reverse-engineering the distributed binaries, **expressly subject
to the statutory reservations in §4.2** — including the non-waivable
interoperability right under Article 6 of Directive 2009/24/EC, the exceptions at
17 U.S.C. §§ 1201(f), (g) and (j), and sections 30.6, 30.61, 41.12 and 41.13 of
the Canadian *Copyright Act*; a prohibition on use against data the operator is
not authorised to access (§4.3); trademark and non-affiliation terms (§5);
warranty disclaimer, liability limitation, and indemnity (§§7–9); and Ontario
governing law and forum (§12).

By downloading, installing, executing, cloning, or forking this repository, you
accept that licence in full.