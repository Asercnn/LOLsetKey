<script setup lang="ts">

import { invoke } from '@tauri-apps/api/core'
import UserInfo from './interface/UserInfo.ts';
import { ref, VueElement } from 'vue'
import { reactive, toRefs } from 'vue'
const state = reactive({
  circleUrl:
    'https://cube.elemecdn.com/3/7c/3ea6beec64369c2642b92c6726f1epng.png',
  squareUrl:
    'https://cube.elemecdn.com/9/c2/f0ee8a3c7c9638a54940382568c9dpng.png',
  sizeList: ['small', '', 'large'] as const,
})

let userInfo = ref<UserInfo|null>(null)
const gamePath = ref('')
const successMessage = ref('')
const errorMessage = ref('')

function clearMessages() {
  successMessage.value = ''
  errorMessage.value = ''
}

async function backupConfigs() {
  clearMessages()
  try {
    await invoke('backup_configs', { gamePath: gamePath.value })
    successMessage.value = '配置文件已保存到当前目录'
  } catch (e) {
    errorMessage.value = '保存失败：' + e
  }
}

async function init() {
  clearMessages()
  try {
    userInfo = await invoke('init')
    console.log(`testTag userInfo:${userInfo}`);
    
  } catch (e) {
    errorMessage.value = '初始化失败：' + e
  }
}
async function restoreConfigs() {
  clearMessages()
  try {
    await invoke('restore_configs', { gamePath: gamePath.value })
    successMessage.value = '配置文件已恢复到游戏目录'
  } catch (e) {
    errorMessage.value = '恢复失败：' + e
  }

}

// 后面会帮你写自动检测路径逻辑
async function detectPath() {
  clearMessages()
  try {
    const path = await invoke('find_lol_root') as string;
    if (path) {
      gamePath.value = path
      successMessage.value = '自动检测到游戏路径：' + path
    } else {
      errorMessage.value = '自动检测失败，请手动填写'
    }
  } catch (e) {
    errorMessage.value = '检测路径失败：' + e
  }
}

</script>

<template>
  <main class="container">
    <div class="block">
      <el-avatar :size="50" :src="state.circleUrl" />
    </div>
    <span>
      <el-text :title="name">欢迎！</el-text>
      <h1>英雄联盟配置管理工具</h1>
    </span>


    <button type="button" @click="init">init</button>

    <span style="box-sizing: border-box;width: 100%; ">
      <label style="">英雄联盟安装目录：</label>
      <input style="display: inline-block; width: 60%; " v-model="gamePath"
        placeholder="例如 C:\Riot Games\League of Legends" />
    </span>

    <div>
      <button @click="detectPath" style="flex-shrink: 0;">自动检测路径</button>
    </div>



    <div>
      <button @click="backupConfigs" :disabled="!gamePath">保存配置（备份）</button>
      <button @click="restoreConfigs" :disabled="!gamePath" style="margin-left: 20px;">恢复配置（覆盖）</button>
    </div>

    <div style="margin-top: 20px; color: green;">{{ successMessage }}</div>
    <div style="margin-top: 20px; color: red;">{{ errorMessage }}</div>
  </main>
</template>

<style scoped></style>

<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: auto;
  display: flex;
  flex-direction: column;
  text-align: center;
  gap: 10px;
  width: 80vw;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>