<template>
  <div class="home-container">
    <!-- 动态背景 -->
    <div class="animated-bg">
      <div class="blob blob-1"></div>
      <div class="blob blob-2"></div>
      <div class="blob blob-3"></div>
    </div>

    <div class="content-wrapper">
      <!-- 顶栏介绍 -->
      <header class="hero-section">
        <div class="hero-content">
          <div class="badge">🚀 AdProof Web3 Edition</div>
          <h1 class="hero-title">
            下一代 <span class="gradient-text">隐私凭证网络</span>
          </h1>
          <p class="hero-subtitle">
            融合零知识证明与 Solana 区块链，为数字机构、个人及 AI 智能体打造可验证、不可篡改、保护隐私的身份与资产凭证体系。
          </p>
          <div class="hero-actions">
            <button class="glow-btn" @click="scrollToApps">
              探索应用生态
            </button>
            <button class="outline-btn" @click="router.push('/identity')">
              <el-icon class="el-icon--left" style="margin-right: 8px;"><User /></el-icon> 创建 AI 身份
            </button>
          </div>
        </div>
        
        <div class="hero-visual">
          <div class="glass-orb">
            <el-icon class="orb-icon"><Lock /></el-icon>
          </div>
        </div>
      </header>

      <!-- 核心应用区 -->
      <section class="app-section" id="apps">
        <div class="section-header">
          <h2 class="section-title">核心应用矩阵</h2>
          <p class="section-desc">三大应用场景，满足不同领域的凭证需求</p>
        </div>

        <div class="app-grid">
          <!-- AI 数字身份系统 -->
          <div class="glass-card app-card" @click="router.push('/identity')">
            <div class="card-glow"></div>
            <div class="app-icon-wrapper agent-icon">
              <el-icon><Cpu /></el-icon>
            </div>
            <h3 class="app-card-title">AI 数字身份体系</h3>
            <p class="app-card-desc">为 AI 智能体生成具有防伪、授权和可验证能力的专属链上身份。</p>
            <ul class="feature-list">
              <li><el-icon><Check /></el-icon> 智能体身份绑定</li>
              <li><el-icon><Check /></el-icon> ZK 签名授权</li>
              <li><el-icon><Check /></el-icon> 操作可审计追踪</li>
            </ul>
            <div class="app-card-action">
              进入系统 <el-icon><ArrowRight /></el-icon>
            </div>
          </div>

          <!-- 学历凭证系统 -->
          <div class="glass-card app-card" @click="router.push('/credential')">
            <div class="card-glow"></div>
            <div class="app-icon-wrapper edu-icon">
              <el-icon><School /></el-icon>
            </div>
            <h3 class="app-card-title">可信学历凭证</h3>
            <p class="app-card-desc">颁发与验证不可篡改的链上学历证明，消除学历造假。</p>
            <ul class="feature-list">
              <li><el-icon><Check /></el-icon> 院校一键颁发</li>
              <li><el-icon><Check /></el-icon> 用人单位扫码验证</li>
              <li><el-icon><Check /></el-icon> 数据全程防伪</li>
            </ul>
            <div class="app-card-action">
              进入系统 <el-icon><ArrowRight /></el-icon>
            </div>
          </div>

          <!-- 金融凭证系统 -->
          <div class="glass-card app-card" @click="navigateToFinancial">
            <div class="card-glow"></div>
            <div class="app-icon-wrapper fin-icon">
              <el-icon><Money /></el-icon>
            </div>
            <h3 class="app-card-title">去中心化金融凭证</h3>
            <p class="app-card-desc">在不泄露具体金额的情况下，证明个人的收入与信用资质。</p>
            <ul class="feature-list">
              <li><el-icon><Check /></el-icon> 零知识证明信用分数</li>
              <li><el-icon><Check /></el-icon> 跨境资产证明流转</li>
              <li><el-icon><Check /></el-icon> 资金流水加密验证</li>
            </ul>
            <div class="app-card-action">
              进入系统 <el-icon><ArrowRight /></el-icon>
            </div>
          </div>
        </div>
      </section>

      <!-- 优势区 -->
      <section class="advantage-section">
        <div class="section-header">
          <h2 class="section-title">技术优势</h2>
        </div>
        <div class="advantage-grid">
          <div class="adv-item">
            <div class="adv-icon"><el-icon><Aim /></el-icon></div>
            <h4>极致隐私</h4>
            <p>基于zk-SNARKs，验证者仅需得知"是/否"，无需接触原始敏感数据。</p>
          </div>
          <div class="adv-item">
            <div class="adv-icon"><el-icon><Connection /></el-icon></div>
            <h4>去中心化抗审查</h4>
            <p>依托Solana高性能公链记录哈希流水，无单点故障风险，凭证永久有效。</p>
          </div>
          <div class="adv-item">
            <div class="adv-icon"><el-icon><Lightning /></el-icon></div>
            <h4>毫秒级验证</h4>
            <p>结合链上确认与本地ZK校验机制，保证金融级安全的同时提供极致流畅体验。</p>
          </div>
        </div>
      </section>
      
      <footer class="footer">
        <p>© 2026 AdProof Network. Powering the Trust Economy.</p>
      </footer>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { useRouter } from 'vue-router'
import { 
  User, Cpu, School, Money, Check, ArrowRight, Lock, 
  Aim, Connection, Lightning
} from '@element-plus/icons-vue'

const router = useRouter()

const scrollToApps = () => {
  const el = document.getElementById('apps')
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' })
  }
}

// 导航到金融系统
const navigateToFinancial = () => {
  const currentRole = localStorage.getItem('role') || ''
  if (!['financial_institution', 'individual', 'verifier'].includes(currentRole)) {
    localStorage.setItem('role', 'financial_institution')
  }
  router.push('/financial')
}
</script>

<style scoped>
/* Base Layout */
.home-container {
  min-height: calc(100vh - 50px);
  margin: -20px; /* Negate the el-main padding */
  background-color: #0F172A; /* Dark mode style */
  color: #F8FAFC;
  font-family: 'Inter', -apple-system, system-ui, sans-serif;
  overflow-x: hidden;
  position: relative;
}

.content-wrapper {
  position: relative;
  z-index: 10;
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 24px;
}

/* Animated Background Blobs */
.animated-bg {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  overflow: hidden;
  z-index: 0;
  pointer-events: none;
}

.blob {
  position: absolute;
  filter: blur(80px);
  opacity: 0.5;
  border-radius: 50%;
  animation: float 20s infinite ease-in-out alternate;
}

.blob-1 {
  top: -10%; left: -10%;
  width: 50vw; height: 50vw;
  background: radial-gradient(circle, rgba(79,70,229,0.3) 0%, rgba(15,23,42,0) 70%);
}

.blob-2 {
  top: 20%; right: -10%;
  width: 60vw; height: 60vw;
  background: radial-gradient(circle, rgba(236,72,153,0.2) 0%, rgba(15,23,42,0) 70%);
  animation-delay: -5s;
}

.blob-3 {
  bottom: -20%; left: 30%;
  width: 40vw; height: 40vw;
  background: radial-gradient(circle, rgba(56,189,248,0.2) 0%, rgba(15,23,42,0) 70%);
  animation-delay: -10s;
}

@keyframes float {
  0% { transform: translate(0, 0) scale(1); }
  50% { transform: translate(5%, 10%) scale(1.1); }
  100% { transform: translate(-5%, 5%) scale(0.95); }
}

/* Hero Section */
.hero-section {
  display: flex;
  align-items: center;
  justify-content: space-between;
  min-height: 80vh;
  gap: 40px;
  padding-top: 40px;
}

.hero-content {
  flex: 1;
  max-width: 650px;
  animation: fadeUp 1s cubic-bezier(0.25, 0.8, 0.25, 1);
}

@keyframes fadeUp {
  0% { opacity: 0; transform: translateY(30px); }
  100% { opacity: 1; transform: translateY(0); }
}

.badge {
  display: inline-block;
  padding: 6px 16px;
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 999px;
  font-size: 0.875rem;
  font-weight: 500;
  color: #E2E8F0;
  margin-bottom: 24px;
  backdrop-filter: blur(10px);
}

.hero-title {
  font-size: 4rem;
  font-weight: 800;
  line-height: 1.2;
  margin-bottom: 24px;
  color: #FFFFFF;
  letter-spacing: -0.02em;
}

.gradient-text {
  background: linear-gradient(135deg, #A78BFA 0%, #F472B6 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.hero-subtitle {
  font-size: 1.25rem;
  line-height: 1.6;
  color: #94A3B8;
  margin-bottom: 40px;
}

.hero-actions {
  display: flex;
  gap: 16px;
}

.glow-btn, .outline-btn {
  padding: 14px 28px;
  font-size: 1.1rem;
  border-radius: 8px;
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.3s ease;
  font-family: inherit;
}

.glow-btn {
  background: linear-gradient(135deg, #4F46E5 0%, #EC4899 100%);
  border: none;
  font-weight: 600;
  color: white;
  box-shadow: 0 10px 25px -5px rgba(236, 72, 153, 0.4);
}

.glow-btn:hover {
  transform: translateY(-3px);
  box-shadow: 0 15px 35px -5px rgba(236, 72, 153, 0.6);
}

.outline-btn {
  background: rgba(255, 255, 255, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.2);
  color: #FFFFFF;
  font-weight: 500;
  backdrop-filter: blur(10px);
}

.outline-btn:hover {
  background: rgba(255, 255, 255, 0.1);
  border-color: rgba(255, 255, 255, 0.4);
}

.hero-visual {
  flex: 1;
  display: flex;
  justify-content: center;
  align-items: center;
  animation: float-y 6s ease-in-out infinite;
}

.glass-orb {
  width: 320px;
  height: 320px;
  background: linear-gradient(135deg, rgba(255,255,255,0.05) 0%, rgba(255,255,255,0.01) 100%);
  border: 1px solid rgba(255, 255, 255, 0.1);
  border-radius: 50%;
  backdrop-filter: blur(20px);
  display: flex;
  justify-content: center;
  align-items: center;
  box-shadow: inset 0 0 60px rgba(255,255,255,0.05), 0 20px 50px rgba(0,0,0,0.5);
  position: relative;
}

.glass-orb::before {
  content: '';
  position: absolute;
  top: 10%;
  left: 10%;
  width: 80%;
  height: 80%;
  background: radial-gradient(circle, rgba(167, 139, 250, 0.2) 0%, transparent 70%);
  border-radius: 50%;
  z-index: -1;
}

@keyframes float-y {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-20px); }
}

.orb-icon {
  font-size: 100px;
  background: linear-gradient(135deg, #38BDF8 0%, #818CF8 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  filter: drop-shadow(0 0 20px rgba(129, 140, 248, 0.5));
}

/* App Section */
.app-section {
  padding: 80px 0;
}

.section-header {
  text-align: center;
  margin-bottom: 60px;
}

.section-title {
  font-size: 2.5rem;
  font-weight: 700;
  margin-bottom: 16px;
  color: #F8FAFC;
}

.section-desc {
  font-size: 1.1rem;
  color: #94A3B8;
  max-width: 600px;
  margin: 0 auto;
}

.app-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
  gap: 30px;
}

.glass-card {
  background: rgba(30, 41, 59, 0.4);
  border: 1px solid rgba(255, 255, 255, 0.08);
  border-radius: 24px;
  padding: 40px 30px;
  position: relative;
  overflow: hidden;
  backdrop-filter: blur(12px);
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.25, 0.8, 0.25, 1);
  display: flex;
  flex-direction: column;
}

.glass-card:hover {
  transform: translateY(-10px);
  border-color: rgba(255, 255, 255, 0.2);
  background: rgba(30, 41, 59, 0.7);
  box-shadow: 0 20px 40px rgba(0,0,0,0.4);
}

.card-glow {
  position: absolute;
  top: -50px;
  right: -50px;
  width: 150px;
  height: 150px;
  border-radius: 50%;
  filter: blur(50px);
  opacity: 0.2;
  transition: opacity 0.3s ease;
  z-index: 0;
}

.app-card:nth-child(1) .card-glow { background: #38BDF8; }
.app-card:nth-child(2) .card-glow { background: #10B981; }
.app-card:nth-child(3) .card-glow { background: #F59E0B; }

.glass-card:hover .card-glow {
  opacity: 0.5;
}

.app-icon-wrapper {
  width: 64px;
  height: 64px;
  border-radius: 18px;
  display: flex;
  justify-content: center;
  align-items: center;
  font-size: 32px;
  margin-bottom: 24px;
  color: white;
  position: relative;
  z-index: 1;
}

.agent-icon { background: linear-gradient(135deg, #0284C7, #38BDF8); box-shadow: 0 10px 20px rgba(2, 132, 199, 0.3); }
.edu-icon { background: linear-gradient(135deg, #047857, #10B981); box-shadow: 0 10px 20px rgba(4, 120, 87, 0.3); }
.fin-icon { background: linear-gradient(135deg, #B45309, #F59E0B); box-shadow: 0 10px 20px rgba(180, 83, 9, 0.3); }

.app-card-title {
  font-size: 1.5rem;
  font-weight: 600;
  margin-bottom: 12px;
  color: #F8FAFC;
  position: relative;
  z-index: 1;
}

.app-card-desc {
  color: #94A3B8;
  line-height: 1.6;
  margin-bottom: 24px;
  min-height: 50px;
  position: relative;
  z-index: 1;
}

.feature-list {
  list-style: none;
  padding: 0;
  margin: 0 0 30px 0;
  position: relative;
  z-index: 1;
  flex: 1;
}

.feature-list li {
  display: flex;
  align-items: center;
  gap: 10px;
  color: #CBD5E1;
  margin-bottom: 12px;
  font-size: 0.95rem;
}

.feature-list li .el-icon {
  color: #10B981;
}

.app-card-action {
  font-weight: 600;
  color: #FFF;
  display: flex;
  align-items: center;
  gap: 8px;
  transition: gap 0.3s;
  position: relative;
  z-index: 1;
}

.app-card:hover .app-card-action {
  gap: 12px;
  color: #F472B6;
}

/* Advantage Section */
.advantage-section {
  padding: 60px 0 100px;
}

.advantage-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
  gap: 40px;
  margin-top: 50px;
}

.adv-item {
  text-align: center;
  padding: 30px;
  background: rgba(255, 255, 255, 0.02);
  border-radius: 20px;
  border: 1px solid rgba(255, 255, 255, 0.05);
  transition: all 0.3s ease;
}

.adv-item:hover {
  background: rgba(255, 255, 255, 0.05);
  transform: translateY(-5px);
}

.adv-icon {
  width: 70px;
  height: 70px;
  background: rgba(255,255,255,0.05);
  border: 1px solid rgba(255,255,255,0.1);
  border-radius: 50%;
  display: flex;
  justify-content: center;
  align-items: center;
  margin: 0 auto 20px;
  font-size: 28px;
  color: #A78BFA;
  box-shadow: inset 0 0 20px rgba(255,255,255,0.02);
}

.adv-item h4 {
  font-size: 1.25rem;
  font-weight: 600;
  margin-bottom: 12px;
  color: #E2E8F0;
}

.adv-item p {
  color: #94A3B8;
  line-height: 1.6;
}

/* Footer */
.footer {
  text-align: center;
  padding: 30px 0;
  border-top: 1px solid rgba(255,255,255,0.1);
  color: #64748B;
  font-size: 0.9rem;
  margin-top: 40px;
}

/* Responsive */
@media (max-width: 992px) {
  .hero-section {
    flex-direction: column;
    text-align: center;
    padding-top: 60px;
  }
  
  .hero-actions {
    justify-content: center;
  }
  
  .hero-visual {
    margin-top: 40px;
  }
}

@media (max-width: 640px) {
  .hero-title {
    font-size: 2.5rem;
  }
  
  .hero-actions {
    flex-direction: column;
  }
  
  .glow-btn, .outline-btn {
    width: 100%;
  }
  
  .glass-orb {
    width: 250px;
    height: 250px;
  }
}
</style>
