<script setup>
import { ref, onMounted } from 'vue';
import ScatterChart from '../components/ScatterChart.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';
// import { useNotification } from "naive-ui";
import cmds from '../api/cmds';

// const notification = useNotification();

const data = ref()
const csvPath = ref("")

const handleDataProcessed = (event) => {
  // console.log(event.payload);
};

onMounted(() => {
  // window.setInterval(function timer() {
  //   data.value = (
  //     [0, 0],
  //     [1, 3]
  // )
  // }, 1000);

  listen('data-processed', handleDataProcessed);
})

async function selectCsv() {
  await open({
    multiple: true,
    filters: [{
      name: 'CSV',
      extensions: ['csv']
    }]
  }).then((file) => {
    csvPath.value = file.toString()
  })
}

function notify_test() {
  // cmds.notify_info(notification, "test")
}

async function process_csv_data() {
  await cmds.cmd_process_csv_data(csvPath.value).then((axis) => {
    data.value = axis
  })
}

</script>

<template>
  <n-card title="" class="card">

    <n-grid x-gap="4" :cols="48">
      <n-gi span="38">
        <n-input v-model:value="csvPath" type="text" readonly="true" placeholder="" />
      </n-gi>
      <n-gi :offset="1" span="4">
        <n-button @click="selectCsv">导入CSV</n-button>
      </n-gi>
      <n-gi :offset="2" span="1">
        <n-button @click="process_csv_data">处理</n-button>
      </n-gi>
    </n-grid>

    <ScatterChart :value=data height="77vh" />
  </n-card>
</template>

<style scoped>
/* .n-card {
  max-height: 300px;
} */

.card {
  height: 90vh;
}

@media (min-height: 760px) {
  .card {
    height: 92vh;
  }
}
</style>