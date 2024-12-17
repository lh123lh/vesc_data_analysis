<script setup>
import { ref, watch } from "vue";
import { Window } from '@tauri-apps/api/window';

import {
  RemoveOutline as Minimize,
  SquareOutline as Maximize,
  Close
} from "@vicons/ionicons5";

const appWindow = new Window('main');

const max_state_name = ref('window-maximize');
const max_state = ref(false);

watch(max_state, async (newValue) => {
  if (newValue) { // 当前状态是最大化
    max_state_name.value = 'window-restore';
    await appWindow.maximize();
  } else {
    max_state_name.value = 'window-maximize';
    await appWindow.unmaximize();
  }
});

async function window_minimize() {
  await appWindow.minimize();
}

function window_maximize() {
  max_state.value = !max_state.value;
}

async function window_close() {
  await appWindow.close();
}
</script>

<template>
  <n-row class="titlebar">
    <n-col :span="15" >
    </n-col>
    <n-col :span="8" class="ms-auto">
      <div id="stage-button">
        <div class="min" @click="window_minimize">
          <n-icon size="20">
            <Minimize />
          </n-icon>
        </div>
        <div class="max" @click="window_maximize" :name="max_state_name">
          <n-icon size="20">
            <Maximize />
          </n-icon>
        </div>
        <div class="close" @click="window_close">
          <n-icon size="20">
            <Close />
          </n-icon>
        </div>
      </div>
    </n-col>
  </n-row>
</template>

<style scoped>
.titlebar {
  display: flex;
  flex-direction: row;
  height: 30px;
  user-select: none;
  top: 0px;
  left: 0;
  right: 0;
}

#stage-button {
  display: flex;
  flex-direction: row;
  justify-content: flex-end;
  margin-left: auto;
}

.min,
.max,
.close {
  width: 50px;
  height: 40px;
  /* margin-left: 25px; */
  padding: 10px;
  color: gray;
}

.min:hover,
.max:hover {
  color: black;
  /* padding: 10px; */
  background-color: #33303020;
}

.close:hover {
  fill: white;
  color: white;
  padding: 10px;
  background-color: red;
}
</style>