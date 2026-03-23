# Ad Proof Web

这是 `ad-proof` 的前端演示层，定位为一个围绕真实准入场景的 selective disclosure VC demo，而不是泛凭证平台。

当前只保留两个页面：

- 收入门槛证明：证明 `income tier >= required tier`
- KYC 等级证明：证明 `kyc level >= required level`

对应后端接口：

- `POST /api/vc/income/issue`
- `POST /api/vc/income/present`
- `POST /api/vc/income/verify`
- `POST /api/vc/kyc/issue`
- `POST /api/vc/kyc/present`
- `POST /api/vc/kyc/verify`

本地开发：

```bash
pnpm install
pnpm dev
```

默认通过 Vite 代理到 `http://127.0.0.1:8090`。前端会在请求头中自动携带 `X-Role`，其中：

- 收入签发使用 `financial_institution`
- KYC 签发使用 `kyc_provider`
- presentation 生成与验证分别可切换为 `holder`、`verifier`
