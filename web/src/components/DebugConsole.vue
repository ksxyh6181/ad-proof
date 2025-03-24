<template>
  <div class="debug-console" :class="{ minimized: !expanded }">
    <div class="debug-header" @click="expanded = !expanded">
      <h3>调试控制台 <span v-if="logCount">({{ logCount }})</span></h3>
      <div class="debug-actions">
        <el-button type="danger" size="small" @click.stop="clearLogs">清除</el-button>
        <el-button size="small" @click.stop="expanded = !expanded">
          {{ expanded ? '最小化' : '展开' }}
        </el-button>
      </div>
    </div>
    
    <div v-if="expanded" class="debug-content">
      <div class="filters">
        <el-checkbox v-model="filters.api" label="API"></el-checkbox>
        <el-checkbox v-model="filters.solana" label="Solana"></el-checkbox>
        <el-checkbox v-model="filters.error" label="错误"></el-checkbox>
        <el-checkbox v-model="filters.other" label="其他"></el-checkbox>
      </div>
      
      <div class="logs-container">
        <div v-for="(log, index) in filteredLogs" :key="index" 
             :class="['log-entry', log.type.toLowerCase()]">
          <div class="log-header">
            <span class="log-type">{{ log.type }}</span>
            <span class="log-time">{{ formatTime(log.timestamp) }}</span>
          </div>
          <div class="log-message">{{ log.message }}</div>
          <div v-if="log.data" class="log-data">
            <pre v-if="typeof log.data === 'object'">{{ JSON.stringify(log.data, null, 2) }}</pre>
            <span v-else>{{ log.data }}</span>
          </div>
        </div>
        
        <div v-if="filteredLogs.length === 0" class="no-logs">
          没有日志记录
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue';
import { debug } from '@/api/credential';

// 状态
const expanded = ref(false);
const filters = ref({
  api: true,
  solana: true,
  error: true,
  other: true
});

// 计算属性
const logs = computed(() => debug.getLogs());
const logCount = computed(() => logs.value.length);

const filteredLogs = computed(() => {
  return logs.value.filter(log => {
    const type = log.type.toLowerCase();
    if (type === 'api' && filters.value.api) return true;
    if (type === 'solana' && filters.value.solana) return true;
    if (type === 'error' && filters.value.error) return true;
    // 其他类型
    return filters.value.other && !['api', 'solana', 'error'].includes(type);
  });
});

// 格式化时间戳
function formatTime(timestamp: string) {
  const date = new Date(timestamp);
  return date.toLocaleTimeString() + '.' + date.getMilliseconds().toString().padStart(3, '0');
}

// 清除所有日志
function clearLogs(e: Event) {
  e.stopPropagation();
  debug.clear();
}

// 定时刷新日志
let refreshInterval: number | null = null;

onMounted(() => {
  // 每秒刷新一次日志视图，确保显示最新日志
  refreshInterval = window.setInterval(() => {
    // 触发计算属性重新计算
    logs.value;
  }, 1000);
});

onUnmounted(() => {
  if (refreshInterval !== null) {
    clearInterval(refreshInterval);
  }
});
</script>

<style scoped>
.debug-console {
  position: fixed;
  bottom: 0;
  right: 0;
  width: 600px;
  background-color: #1e1e1e;
  color: #d4d4d4;
  border-top-left-radius: 8px;
  z-index: 1000;
  box-shadow: 0 -2px 10px rgba(0, 0, 0, 0.2);
  max-height: 50vh;
  display: flex;
  flex-direction: column;
}

.debug-console.minimized {
  height: auto;
}

.debug-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  background-color: #333;
  padding: 8px 12px;
  cursor: pointer;
  border-top-left-radius: 8px;
}

.debug-header h3 {
  margin: 0;
  font-size: 16px;
  color: #fff;
}

.debug-actions {
  display: flex;
  gap: 8px;
}

.debug-content {
  flex: 1;
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.filters {
  display: flex;
  gap: 12px;
  padding: 8px 12px;
  background-color: #252525;
  border-bottom: 1px solid #444;
}

.logs-container {
  flex: 1;
  overflow-y: auto;
  padding: 8px;
}

.log-entry {
  margin-bottom: 8px;
  padding: 8px;
  border-radius: 4px;
  background-color: #252525;
  font-family: monospace;
}

.log-entry.api {
  border-left: 3px solid #42b983;
}

.log-entry.solana {
  border-left: 3px solid #409eff;
}

.log-entry.error {
  border-left: 3px solid #f56c6c;
  background-color: rgba(245, 108, 108, 0.1);
}

.log-header {
  display: flex;
  justify-content: space-between;
  margin-bottom: 4px;
}

.log-type {
  font-weight: bold;
  color: #9cdcfe;
}

.log-time {
  color: #888;
  font-size: 12px;
}

.log-message {
  color: #d4d4d4;
  margin-bottom: 4px;
  white-space: pre-wrap;
  word-break: break-word;
}

.log-data {
  font-size: 12px;
  background-color: #1e1e1e;
  padding: 6px;
  border-radius: 3px;
  overflow-x: auto;
  color: #ce9178;
}

.log-data pre {
  margin: 0;
  white-space: pre-wrap;
  word-break: break-word;
}

.no-logs {
  text-align: center;
  color: #888;
  padding: 20px;
}
</style>
