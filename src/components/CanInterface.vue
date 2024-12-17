<script setup>
import { ref, onMounted } from 'vue';
import { listen } from '@tauri-apps/api/event'
import { NSpace, NGrid, NGridItem, useMessage, NCard } from 'naive-ui';
import CanConfig from './CanConfig.vue';
import CanMessageList from './CanMessageList.vue';
import CanStatistics from './CanStatistics.vue';
import CanGetvescData from './CanGetvescData.vue';
import cmds from '../api/cmds';
import utils from '../api/utils';

const isConnected = ref(false);
const isStart = ref(false);
const targetId = ref(8);
const csvPath = ref("");
const sampleFreq = ref("200");
const serialPorts = ref();
const message = useMessage();
const recordDuration = ref(0);
const recordDurationFormated = ref("");
const recordSize = ref("");

onMounted(() => { //组件挂载时的生命周期执行的方法
  cmds.cmd_deinit_can_port()

  get_avaliable_ports();

  listen('record-size', handleRecordSize);

  window.setInterval(function timer() {
    get_avaliable_ports()
  }, 3000);

  window.setInterval(function timer() {
    if (isStart.value == true) {
      get_record_size(csvPath.value);
      recordDuration.value += 0.5;
      recordDurationFormated.value = utils.formatSeconds(recordDuration.value);
    }
  }, 500);
})

const handleConnect = (config) => {
  isConnected.value = true;

  init_can_port(config.comPort, config.bitRate);
};

const handleDisconnect = () => {
  isConnected.value = false;
  deinit_can_port();
};

const handleSendMessage = (canMessage) => {
  write_can_data(canMessage.id, canMessage.data);
};

const handleStart = (data) => {
  targetId.value = data.id;
  sampleFreq.value = data.freq;
  csvPath.value = data.csvPath;
  isStart.value = true;

  set_recorder_path(csvPath.value);
  start_recording(targetId.value, parseInt(1000 / sampleFreq.value));
};

const handleStop = (data) => {
  isStart.value = false;
  recordDuration.value = 0;
  stop_recording();
};

const handleRecordSize = (event) => {
  recordSize.value = utils.formatFileSize(event.payload);
};

async function get_avaliable_ports() {
  await cmds.cmd_get_avaliable_ports()
    .then((data) => {
      serialPorts.value = data;

      // 在setInterval中定时查询可用端口时需要去掉下面的代码
      // serialPort.value = serialPorts.value[0];
    }).catch(err => {
      message.error(err);
    });
}

async function init_can_port(port, bitrate) {
  await cmds.cmd_init_can_port(port, bitrate)
    .then((data) => {
      message.success('CAN communication started');
    }).catch(err => {
      message.error(err);
    });
}

async function deinit_can_port() {
  await cmds.cmd_deinit_can_port()
    .then((data) => {
      message.warning('CAN communication stopped');
    }).catch(err => {
      message.error(err);
    });
}

async function read_can_data() {
  await cmds.cmd_read_can_data()
    .then((data) => {
    }).catch(err => {
      message.error(err);
    });
}

async function write_can_data(id, data) {
  await cmds.cmd_write_can_data(id, data)
    .then((data) => {
    })
}

async function set_recorder_path(csv) {
  await cmds.cmd_set_recorder_path(csv)
    .then((data) => {
    }).catch(err => {
      message.error(err);
    });
}

async function get_record_size(csv) {
  await cmds.cmd_get_record_size(csv)
    .then((data) => {
    }).catch(err => {
      message.error(err);
    });
}

async function start_recording(id, msec) {
  await cmds.cmd_start_recording(id, msec)
    .then((data) => {
      message.success('Sample started');
    }).catch(err => {
      message.error(err);
    });
}

async function stop_recording() {
  await cmds.cmd_stop_recording()
    .then((data) => {
      message.warning('Sample stopped');
    }).catch(err => {
      message.error(err);
    });
}
</script>

<template>
  <div class="container">
    <NGrid :cols="24" :x-gap="12">
      <NGridItem :span="11">
        <NSpace vertical>
          <CanConfig :is-connected="isConnected" :comPorts="serialPorts" @connect="handleConnect"
            @disconnect="handleDisconnect" />
          <CanStatistics :elapsed="recordDurationFormated" :recordSize="recordSize" />
        </NSpace>
      </NGridItem>
      <NGridItem :span="13">
        <!-- <NCard title="CAN Messages" :bordered="true">
          <CanMessageList :messages="messages" />
        </NCard> -->
        <CanGetvescData :is-connected="isConnected" @startCollect="handleStart" @stopCollect="handleStop" />
      </NGridItem>
    </NGrid>
  </div>
</template>

<style scoped>
/* :deep(.n-card) {
  background-color: #242424;
} */

.container {
  min-height: 89vh;
}
</style>