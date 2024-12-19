<script setup>
import { ref, h, onMounted } from "vue";
import { RouterLink, RouterView, useRouter } from 'vue-router'
import { NIcon, darkTheme, lightTheme } from "naive-ui";
import {
  HomeOutline as HomeIcon,
  TrendingUpSharp as TrendIcon,
  MoonOutline as Moon,
  SunnyOutline as Sunny,
  PulseOutline,
  BarChartOutline,
  CheckmarkDoneOutline
} from "@vicons/ionicons5";
import TitleBar from "./components/TitleBar.vue";
import AppUpdateDialog from "./components/AppUpdateDialog.vue";

import { check } from '@tauri-apps/plugin-updater';

const collapsed = ref(false);
const theme = ref(lightTheme);
const isDarkMode = ref(false);
const hasUpdate = ref(false);
const updateDialogShow = ref(false);
const versionInfo = ref("");

onMounted(() => {
  checkAppUpdate()
})

function changeTheme() {
  if (isDarkMode.value) {
    isDarkMode.value = false;
    theme.value = lightTheme;
  }
  else {
    isDarkMode.value = true;
    theme.value = darkTheme;
  }
}

function renderIcon(icon) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

async function checkAppUpdate() {
  const update = await check();

  if (update.available) {
    // console.log(update)
    hasUpdate.value = true;
    versionInfo.value = `## v${update.version}\r\n` + update.body;
  }
}

const menuOptions = [
  {
    label: () => h(
      RouterLink,
      {
        to: {
          name: "home",
        }
      },
      { default: () => "数据采集" }
    ),
    key: "home", // 唯一key
    icon: renderIcon(BarChartOutline)
  },
  {
    label: () => h(
      RouterLink,
      {
        to: {
          name: "dataProcess",
        }
      },
      { default: () => "数据分析" }
    ),
    key: "dataProcess", // 唯一key
    icon: renderIcon(PulseOutline)
  },
  {
    label: () => h(
      RouterLink,
      {
        to: {
          name: "checkGoodnessOfFit",
        }
      },
      { default: () => "拟合度验证" }
    ),
    key: "checkGoodnessOfFit", // 唯一key
    icon: renderIcon(CheckmarkDoneOutline)
  },
];

</script>

<template>
  <n-config-provider :theme="theme">
    <n-space vertical>
      <!-- 页面顶部标题栏 -->
      <n-layout-header bordered class="title-bar" style="margin-top: 0px;" @copy.prevent="" data-tauri-drag-region>
        <n-flex justify="space-between" data-tauri-drag-region>
          <n-flex class="title" justify="space-between">
            <n-icon data-tauri-drag-region style="margin-right: 10px;">
              <img class="custom-icon" src="./assets/icon.png" data-tauri-drag-region />
            </n-icon>
            <n-badge v-if="hasUpdate" :offset="[-5, 25]" value="New" @click="updateDialogShow = true"
              style="cursor: pointer;">
              <h1>Vesc 数据分析</h1>
            </n-badge>
            <h1 v-else data-tauri-drag-region>Vesc 数据分析</h1>
          </n-flex>

          <div>
            <n-flex vertical>
              <TitleBar style="margin-top: -10px;" />
              <n-switch @update:value="changeTheme" style="margin-right: 10px; margin-left: auto; margin-top: 25px;">
                <template #icon>
                  <n-icon size="15">
                    <Sunny v-if="!isDarkMode" />
                    <Moon v-else />
                  </n-icon>
                </template>
              </n-switch>
            </n-flex>
          </div>
        </n-flex>
      </n-layout-header>

      <n-layout has-sider>
        <n-layout-sider bordered collapse-mode="width" :collapsed-width="64" :width="240" :collapsed="collapsed"
          show-trigger @collapse="collapsed = true" @expand="collapsed = false">
          <n-menu :collapsed="collapsed" :collapsed-width="64" :collapsed-icon-size="22" :options="menuOptions"
            default-value="home" />
        </n-layout-sider>

        <n-layout style="margin-left: 15px; margin-right: 15px; min-height: 89.3vh;">
          <NMessageProvider>
            <!-- <RouterView /> -->
            <router-view v-slot="{ Component }">
              <keep-alive>
                <component :is="Component" />
              </keep-alive>
            </router-view>
          </NMessageProvider>
        </n-layout>
      </n-layout>
    </n-space>
    <n-global-style />
    <n-modal-provider>
      <AppUpdateDialog v-model="updateDialogShow" :content="versionInfo" />
    </n-modal-provider>
  </n-config-provider>
</template>

<style scoped>
.title-bar {
  height: 80px;
  line-height: 60px;
  text-align: center;
  width: 100%;
}

.title-bar .title {
  text-align: start;
  margin-left: 10px;
  margin-top: -5px;
}

.title-bar .custom-icon {
  width: 32px;
  height: 32px;
  margin-left: 0px;
  margin-top: 32px;
}
</style>

<!-- <template>
  <n-notification-provider>
    <Main />
  </n-notification-provider>

</template> -->

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

/* .container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
} */

/* .logo {
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
} */

/* a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
} */

/* h1 {
  text-align: center;
} */

/* input,
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
} */

/* button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
} */

/* input,
button {
  outline: none;
} */

/* @media (prefers-color-scheme: dark) {
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
} */
</style>
