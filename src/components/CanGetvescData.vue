<script setup>
import { save } from '@tauri-apps/plugin-dialog';
import { ref } from 'vue';
import { NCard, NForm, NFormItem, NInput, NInputNumber, NButton, NSpace, useMessage } from 'naive-ui';

const emit = defineEmits(['startCollect', 'stopCollect']);
const message = useMessage();

const props = defineProps({
  isConnected: {
    type: Boolean,
    // default: false
  }
});

const formValue = ref({
  id: '8',
  freq: '200',
  csvPath: '',
});

const isStarted = ref(false);

const validateHex = (value, pattern) => {
  if (!value) return true;
  return new RegExp(pattern).test(value);
};

const rules = {
  id: [
    { required: true, message: 'Please input CAN ID' },
    { validator: (rule, value) => validateHex(value, '^[0-9A-Fa-f]{1,8}$'), message: 'Invalid hex format (max 8 digits)' }
  ],
  freq: [
    { required: true, message: '请输入采集频率' },
    { validator: (rule, value) => validateHex(value, '^[0-9A-Fa-f]{1,8}$'), message: 'Invalid hex format (max 8 digits)' }
  ],
  csvPath: [
    { required: true, message: '请选择CSV路径' },
    { validator: (rule, value) => validateHex(value, '^[0-9A-Fa-f]{1,8}$'), message: 'Invalid string format' }
  ],
};

const handleStart = () => {
  try {
    const id = parseInt(formValue.value.id, 16);
    const csvPath = formValue.value.csvPath;
    const freq = formValue.value.freq;

    if (!csvPath || csvPath.trim() === '') {
      message.error("Invalid CSV Path")
      return;
    }

    isStarted.value = true;

    emit('startCollect', {
      id,
      freq,
      csvPath,
    });
  } catch (error) {
    message.error(error);
  }
};

function handleStop() {
  try {
    isStarted.value = false;

    emit('stopCollect');
  } catch (error) {
    message.error(error);
  }
};

async function selectCsv() {
  await save({
    multiple: true,
    filters: [{
      name: 'CSV',
      extensions: ['csv']
    }]
  }).then((file) => {
    formValue.value.csvPath = file.toString()
  })
}

</script>

<template>
  <NCard title="VESC状态采集" :bordered="true">
    <NForm ref="formRef" :model="formValue" :rules="rules" label-placement="left" label-width="80"
      require-mark-placement="right-hanging">
      <NFormItem label="VESC ID" path="id">
        <NInput v-model:value="formValue.id" placeholder="Enter ID in hex (e.g., 18DB33F1)" :disabled="!isConnected || isStarted" />
      </NFormItem>

      <NFormItem label="采样频率" path="freq">
        <n-input v-model:value="formValue.freq" placeholder="采样频率" :disabled="!isConnected || isStarted">
          <template #suffix>
            Hz
          </template>
        </n-input>
      </NFormItem>

      <NFormItem label="CSV路径" path="csvPath">
        <n-input v-model:value="formValue.csvPath" type="text" placeholder="Select csv path" :disabled="!isConnected || isStarted"
          @click="selectCsv" />
      </NFormItem>

      <NSpace justify="right">
        <NButton type="primary" @click="handleStart" :disabled="!isConnected || isStarted" size="large" ghost>
          Start
        </NButton>
        <NButton type="error" @click="handleStop" :disabled="!isStarted" size="large" ghost>
          Stop
        </NButton>
      </NSpace>
    </NForm>
  </NCard>
</template>