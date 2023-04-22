<template>
  <div class="select-container">
    <div class="select-item">
      <el-icon class="select-img"><DataAnalysis /></el-icon>
      <span class="select-text">解释</span>
    </div>
    <div class="select-item">
      <el-icon class="select-img"><MagicStick /></el-icon>
      <span class="select-text">润色</span>
    </div>
    <div class="down-row"></div>
    <div class="select-item">
      <el-icon class="select-img"><ChatDotSquare /></el-icon>
      <span class="select-text">提问</span>
    </div>
    <div class="select-item"  v-on:click="copySelectContent">
      <el-icon class="select-img"><CopyDocument /></el-icon>
      <span class="select-text">复制</span>
    </div>

    <div class="select-item-min" v-on:click="openSettingsWindow">  
      <el-icon class="el-icon--right">
        <More />
      </el-icon>
    </div>
  </div>
</template>

    
<script lang="ts" setup>
import {
  More,
  CopyDocument,
  ChatDotSquare,
  MagicStick,
  DataAnalysis,
} from "@element-plus/icons-vue";
import { invoke } from "@tauri-apps/api";
import { ElMessage } from "element-plus";

// window.addEventListener("DOMContentLoaded", () => {
//   const body = document.body;
//   const width = body.clientWidth;
//   const height = body.clientHeight;
//   console.log(width, height);
//   invoke("set_size", { payload: { width, height } });
// });

const closeSelectWindow = () => {
  invoke("hide_select_window")
}

const openSettingsWindow = () => {
  console.log('open settings window');
  invoke("open_setting_window").then(() => {
    closeSelectWindow()
  })
}

const copySelectContent = () => {
  invoke("copy_select_content");
  ElMessage({
    message: "已复制",
    type: "success",
    duration: 800,
  });
  setTimeout(() => {
    closeSelectWindow()
  }, 1000)
}
</script>

<style scoped>
.el-icon--right {
  color: #fff;
  display: flex;
  align-items: center;
}

.select-item-min {
  width: 30px;
  height: 64px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  position: relative;
  border-radius: 8px;
  text-align: center;
  margin-right: 8px;
}

.select-container {
  display: flex;
  flex-direction: row;
  flex-wrap: nowrap;
  justify-content: center;
  align-items: center;
  border-radius: 8px;
  width: 100%;
  position: absolute;
  background: #484848;
}

.select-item {
  width: 58px;
  height: 64px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  cursor: pointer;
  position: relative;
  border-radius: 8px;
  text-align: center;
}

.select-item:hover {
  background: #353535;
}

.down-row {
  opacity: 1;
  position: absolute;
  width: 29px;
  height: 9px;
  bottom: -9px;
  background: url(data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAADkAAAASCAYAAADhc3ZwAAAAAXNSR0IArs4c6QAAAERlWElmTU0AKgAAAAgAAYdpAAQAAAABAAAAGgAAAAAAA6ABAAMAAAABAAEAAKACAAQAAAABAAAAOaADAAQAAAABAAAAEgAAAAABgJS1AAACZklEQVRYCdVWS0gbURR1Jh8waZqoIQEhIG7LKO6tuFHMQpNJSFGycqebUmhduAu4FiyhKFmEWRg3YfJBAxYKXZR2l0WwXbhwJagIolhSW5JJPG8x8BSdzOeF6oNh7rufc+95901yuR6sZDL5sl6vL0J82263h4nuuS6O425Qu8zzfMHv93/OZDJ/OJpMKpXia7XaLIi+wzNJ256BfAli66hzq1gsXtD13iFJG2KxmKgoyiZ0QVr/FGV0bxvPe5A7f6i+R0kSZ1EUB9DRNJ6Fh4KfgO7QZrMtFwqFr1q1aJJUA6PRaByyBLIvVN3/fqNzUigUWkqn0/861aKLJAGJx+NjzWZzD+JgJ9Bu2kFOAf6HUqm0oTePbpIEMBKJhJCkgo4KehMw9ru02+1vZFn+YgSXN+JcLpeP3W73OIh+MxLHyPcEBF8bJUhyGyJJAnK53LXH4wlD1PzYiS+rhUM9cjqd4yD4ywymoetKJ0gkEr2NRqOMqztF61nLIHjgcDim8/n8mVlsw51UEyHpjc/nm8O+oupYv0HwBzAnrBAkNZkmSYIlSfobDAZFFCOTPeO1HwgEpvAremUV12YVoFqttgRBkFut1jCwRqzikXgc2jYObx5zZ8f/QD35TH+T98HxbXIYGlZR4BpkUzcEsQ3grqB7H+/jW9kzI6kWgVFwBl3dwb5P1el5g+ApBuwERrTvevyN+Jg6ca0EGJL3YR9F0XktP8rWhPzJ5XK96gZBkod5J6niyYQkgOwydGFc4SHaBv1P7Heh3yRDBm1jLXeVJF0s/lf7cY0D0Cler/csm83+pu3dlG8BBYC58B/A2JYAAAAASUVORK5CYII=)
    repeat-x 50% / auto 9px;
}

.select-img {
  color: #c5c2c2;
  width: 20px;
  margin-bottom: 5px;
}

.select-text {
  display: block;
  text-align: center;
  line-height: 16px;
  font-size: 12px;
  font-weight: 500;
  transform: scale(0.84);
  -webkit-font-smoothing: antialiased;
  -webkit-text-stroke-width: 0.2px;
  color: #fff;
}
</style>
