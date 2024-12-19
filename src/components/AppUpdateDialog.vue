<!-- CustomModal.vue -->
<script setup>
import { ref, watch } from 'vue'
import MarkDownDisplay from './MarkDownDisplay.vue';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';

// 定义props
const props = defineProps({
  title: {
    type: String,
    default: '检测到新版本'
  },
  content: {
    type: String,
    default: ''
  },
  positiveText: {
    type: String,
    default: '升级'
  },
  negativeText: {
    type: String,
    default: '取消'
  },
  maskClosable: {
    type: Boolean,
    default: false
  },
})

const visable = defineModel();
const progress = ref(0);
const startUpdate = ref(false);

async function handleUpdate() {
  startUpdate.value = true;
  const update = await check();

  let downloaded = 0;
  let contentLength = 0;
  // alternatively we could also call update.download() and update.install() separately
  await update.downloadAndInstall((event) => {
    switch (event.event) {
      case 'Started':
        contentLength = event.data.contentLength;
        break;
      case 'Progress':
        downloaded += event.data.chunkLength;
        progress.value = parseInt((downloaded / contentLength) * 100)
        break;
      case 'Finished':
        break;
    }
  });

  await relaunch();
}

async function handleCancle() {
  // console.log("cancle")
}

</script>

<template>
  <n-modal v-model:show="visable" :mask-closable="maskClosable" preset="dialog" :title="title"
    :positive-text="positiveText" :negative-text="negativeText" @positive-click="handleUpdate"
    @negative-click="handleCancle">
    <div>
      <MarkDownDisplay :markdown="props.content" />
      <n-progress v-if="startUpdate" type="line" :percentage="progress" indicator-placement="inside" processing />
    </div>
  </n-modal>
</template>