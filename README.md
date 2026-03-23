# ad-proof

`ad-proof` 现在是一个面向真实准入场景的选择性披露（Selective Disclosure）VC Demo，而不是一个“大而全”的凭证平台。

它只回答两个现实问题：

- 用户如何向平台证明“收入达到某个门槛”，但不暴露精确收入？
- 用户如何向平台证明“KYC 等级达到某个要求”，但不暴露完整身份材料？

这个项目的定位是：

- 学习导向的 MVP Demo
- 业务语义优先的最小可解释原型
- 不是通用凭证基础设施
- 不是链上注册表产品

## 当前保留的场景

### 1. 收入门槛证明

验证方真正想知道的是：

- 这个用户是否满足某个收入门槛
- 该凭证是否由可信金融机构签发
- 该凭证是否未过期、未失效

验证方不应该知道的是：

- 用户的精确收入
- 用户的完整金融资料

### 2. KYC 等级证明

验证方真正想知道的是：

- 这个用户是否达到要求的 KYC 等级
- 该凭证是否由可信 KYC 签发方签发
- 该凭证是否未过期、未失效

验证方不应该知道的是：

- 用户完整身份档案
- 原始 KYC 材料

## 三个角色

### Issuer

签发方，负责：

- 签发凭证
- 对凭证头部信息做签名
- 给出过期时间与状态信息

### Holder

持有者，负责：

- 保存凭证
- 按验证方要求生成 presentation
- 只披露最少必要信息

### Verifier

验证方，负责检查：

- 签发方是否可信
- 凭证是否未过期
- 凭证状态是否有效
- 条件证明是否成立

## 为什么这里需要 ZK

这里的零知识证明不是为了证明“字段哈希一致”，而是为了证明一个有业务意义的条件成立。

当前 Demo 里的证明语义是：

- `income_tier >= required_tier`
- `kyc_level >= required_level`

也就是说，验证方获得的是“够不够资格”的答案，而不是完整明文数据。

## 当前系统设计

### 公开信息

以下信息可以公开给验证方：

- `credential_id`
- `issuer_id`
- `kind`
- `expires_at`
- `status`
- 请求的门槛值
- proof 本身

### 私有信息

以下信息默认不向验证方暴露：

- 精确收入
- 原始 KYC 数据
- 完整身份信息

### MVP 里的简化

为了保持 Demo 足够小、足够清楚，当前实现采用了“等级/档位”而不是完整数值电路：

- 收入证明基于 `income_tier`
- KYC 证明基于 `kyc_level`

这能保证验证目标真实，同时把电路复杂度控制在可解释范围内。

## 后端接口

当前网关只保留一组 `/api/vc/*` 接口。

### 收入场景

- `POST /api/vc/income/issue`
- `POST /api/vc/income/present`
- `POST /api/vc/income/verify`

### KYC 场景

- `POST /api/vc/kyc/issue`
- `POST /api/vc/kyc/present`
- `POST /api/vc/kyc/verify`

## 项目结构

当前核心代码主要集中在以下位置：

- [vc-refactor-plan.md](discuss/vc-refactor-plan.md)：重构方案文档
- [vc.rs](server/crates/lib/zkp/src/vc.rs)：VC 核心逻辑与证明流程
- [zkp.rs](server/crates/lib/zkp/src/zkp.rs)：通用 ZK 数据结构与辅助方法
- [vc_controller.rs](server/crates/bin/gateway/src/controller/vc_controller.rs)：网关控制器
- [router.rs](server/crates/bin/gateway/src/router/router.rs)：VC 路由入口

## 已明确删除的方向

本次重构已经把以下内容从主叙事中移除：

- 学历证书主线
- 泛金融凭证全家桶
- personhood / agent binding
- 以“字段拼接后哈希一致”为核心价值的证明逻辑
- 以“为了上链而上链”为目标的注册表叙事

## 运行与检查

后端位于 `server/`。

当前已验证：

```bash
cargo check -p gateway
```

如果你继续开发，建议优先保证以下顺序：

1. 先定义验证方到底想知道什么
2. 再决定哪些信息应公开
3. 最后再决定哪些逻辑值得进入 ZK 电路

## 非目标

这个项目当前不追求：

- 多场景凭证平台化
- 完整 DID/VC 协议栈
- 通用链上注册表
- 花哨但不解决真实问题的 ZK Demo

当前优先级只有三个：

- 靶子准
- 语义硬
- 结构清楚
