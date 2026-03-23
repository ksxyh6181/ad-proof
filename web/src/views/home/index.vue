<template>
  <div class="page-shell">
    <section class="hero-panel home-hero">
      <div class="hero-copy">
        <span class="eyebrow">最小可解释 VC Demo</span>
        <h1 class="section-title">只证明平台真正想知道的条件，不多给一行明文。</h1>
        <p class="section-copy">
          这个前端只围绕两个真实准入问题展开：收入是否达到门槛，以及用户是否达到 KYC 等级。
          Issuer 签发凭证，Holder 生成最小披露 presentation，Verifier 只校验业务条件、签发方、有效期与状态。
        </p>

        <div class="hero-actions">
          <button class="accent-button" type="button" @click="router.push('/income')">
            打开收入门槛工作台
          </button>
          <button class="ghost-button" type="button" @click="router.push('/kyc')">
            打开 KYC 工作台
          </button>
        </div>
      </div>

      <div class="hero-stack">
        <div class="proof-tile">
          <span class="stage-badge">Issuer</span>
          <strong>签发带有效期与签名的凭证头</strong>
          <p>收入签发方使用 `bank_demo_001`，KYC 签发方使用 `kyc_demo_001`。</p>
        </div>
        <div class="proof-tile">
          <span class="stage-badge secondary">Holder</span>
          <strong>生成条件证明</strong>
          <p>证明 `income tier >= threshold` 或 `kyc level >= required level`，不暴露精确值。</p>
        </div>
        <div class="proof-tile">
          <span class="stage-badge">Verifier</span>
          <strong>只检查必要事实</strong>
          <p>验证签发者可信、证明有效、凭证未过期，而不是查看完整收入或身份信息。</p>
        </div>
      </div>
    </section>

    <section class="card-grid">
      <article class="content-card card-span-6 scenario-card income">
        <p class="card-kicker">Scenario A</p>
        <h2 class="card-heading">收入门槛证明</h2>
        <p class="card-copy">
          适用于租房、授信预筛、商家准入。平台只要知道申请人是否达到收入等级门槛，不需要看到精确月收入或银行流水全文。
        </p>
        <div class="pill-row">
          <span class="pill">月收入等级</span>
          <span class="pill">Issuer 签发</span>
          <span class="pill">未过期</span>
          <span class="pill">选择性披露</span>
        </div>
        <button class="ghost-button" type="button" @click="router.push('/income')">
          进入收入门槛页面
        </button>
      </article>

      <article class="content-card card-span-6 scenario-card kyc">
        <p class="card-kicker">Scenario B</p>
        <h2 class="card-heading">KYC 等级证明</h2>
        <p class="card-copy">
          适用于高风险交易、提现额度升级与合规准入。Verifier 只需要知道“是否达到某一 KYC 级别”，不需要拿走完整身份档案。
        </p>
        <div class="pill-row">
          <span class="pill">KYC Level</span>
          <span class="pill">可信签发方</span>
          <span class="pill">状态可检查</span>
          <span class="pill">避免过度收集</span>
        </div>
        <button class="ghost-button" type="button" @click="router.push('/kyc')">
          进入 KYC 页面
        </button>
      </article>
    </section>

    <section class="card-grid">
      <article class="info-card card-span-4">
        <p class="card-kicker">角色</p>
        <h3 class="card-heading">谁在做什么</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label">Issuer</span>
            <span class="summary-value">签发凭证头，承担可信来源责任</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">Holder</span>
            <span class="summary-value">持有凭证并生成针对 verifier 的 presentation</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">Verifier</span>
            <span class="summary-value">只验证业务条件，不保留不必要的明文数据</span>
          </div>
        </div>
      </article>

      <article class="info-card card-span-4">
        <p class="card-kicker">为什么不用明文</p>
        <h3 class="card-heading">因为 verifier 的问题很窄</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label">收入场景</span>
            <span class="summary-value">平台只想知道是否达标，不需要看到精确数额</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">KYC 场景</span>
            <span class="summary-value">平台只想知道等级是否满足要求，不需要完整身份档案</span>
          </div>
          <div class="summary-row">
            <span class="summary-label">结果</span>
            <span class="summary-value">减少过度采集，降低泄露面和复用风险</span>
          </div>
        </div>
      </article>

      <article class="info-card card-span-4">
        <p class="card-kicker">当前后端接口</p>
        <h3 class="card-heading">前端对齐的 API</h3>
        <div class="summary-list">
          <div class="summary-row">
            <span class="summary-label mono">POST /api/vc/income/issue</span>
            <span class="summary-value">收入签发</span>
          </div>
          <div class="summary-row">
            <span class="summary-label mono">POST /api/vc/income/present</span>
            <span class="summary-value">收入证明生成</span>
          </div>
          <div class="summary-row">
            <span class="summary-label mono">POST /api/vc/income/verify</span>
            <span class="summary-value">收入证明校验</span>
          </div>
          <div class="summary-row">
            <span class="summary-label mono">POST /api/vc/kyc/*</span>
            <span class="summary-value">KYC 同构三步流</span>
          </div>
        </div>
      </article>
    </section>

    <section class="content-card card-span-12">
      <p class="card-kicker">公开与私有</p>
      <h2 class="card-heading">Verifier 实际看到的内容已经被刻意压缩</h2>
      <div class="metric-grid">
        <div class="metric-box">
          <strong>公开信息</strong>
          <p class="card-copy">凭证 ID、issuer、有效期、所证明的条件类型、阈值或所需等级、presentation 中的公共输入。</p>
        </div>
        <div class="metric-box">
          <strong>私有信息</strong>
          <p class="card-copy">精确收入、KYC 原始明细、完整身份字段、issuer 原始内部记录。</p>
        </div>
      </div>
    </section>
  </div>
</template>

<script lang="ts" setup>
import { useRouter } from 'vue-router'

const router = useRouter()
</script>

<style scoped>
.home-hero {
  display: grid;
  grid-template-columns: minmax(0, 1.2fr) minmax(320px, 0.8fr);
  gap: 24px;
  overflow: hidden;
  position: relative;
}

.home-hero::after {
  content: '';
  position: absolute;
  inset: auto -8% -20% auto;
  width: 340px;
  height: 340px;
  border-radius: 50%;
  background: radial-gradient(circle, rgba(56, 108, 103, 0.18), transparent 68%);
  pointer-events: none;
}

.hero-copy {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.hero-actions {
  display: flex;
  flex-wrap: wrap;
  gap: 12px;
}

.hero-stack {
  display: grid;
  gap: 16px;
}

.proof-tile {
  padding: 20px;
  border-radius: 24px;
  background: rgba(255, 248, 239, 0.72);
  border: 1px solid rgba(99, 77, 55, 0.1);
}

.proof-tile strong {
  display: block;
  margin: 12px 0 10px;
  color: var(--ap-text-strong);
}

.proof-tile p {
  margin: 0;
  color: var(--ap-text-body);
  line-height: 1.7;
}

.scenario-card {
  display: flex;
  flex-direction: column;
  gap: 18px;
}

.scenario-card.income {
  background:
    linear-gradient(180deg, rgba(255, 251, 244, 0.9), rgba(255, 242, 232, 0.85)),
    var(--ap-surface);
}

.scenario-card.kyc {
  background:
    linear-gradient(180deg, rgba(255, 251, 244, 0.9), rgba(235, 246, 243, 0.88)),
    var(--ap-surface);
}

@media (max-width: 980px) {
  .home-hero {
    grid-template-columns: 1fr;
  }
}
</style>
