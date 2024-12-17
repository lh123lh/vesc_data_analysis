<script setup>
import { ref } from 'vue';
import { NCard, NForm, NFormItem, NSelect, NButton, NSpace } from 'naive-ui';

const emit = defineEmits(['connect', 'disconnect']);

const props = defineProps({
  isConnected: {
    type: Boolean,
    default: false
  },
  comPorts: {
    type: Array,
    default: ['']
  }
});

// const comPorts = ref(['COM1', 'COM2', 'COM3', 'COM4', 'COM5']);
const bitRates = ref([
  { label: '10 Kbps', value: 'Setup10Kbit' },
  { label: '20 Kbps', value: 'Setup20Kbit' },
  { label: '50 Kbps', value: 'Setup50Kbit' },
  { label: '100 Kbps', value: 'Setup100Kbit' },
  { label: '125 Kbps', value: 'Setup125Kbit' },
  { label: '250 Kbps', value: 'Setup250Kbit' },
  { label: '500 Kbps', value: 'Setup500Kbit' },
  { label: '800 Kbps', value: 'Setup800Kbit' },
  { label: '1 Mbps', value: 'Setup1Mbit' }
]);

const formValue = ref({
  comPort: null,
  bitRate: 'Setup500Kbit'
});

const handleConnect = () => {
  if (formValue.value.comPort && formValue.value.bitRate) {
    emit('connect', formValue.value);
  }
};

const handleDisconnect = () => {
  emit('disconnect');
};
</script>

<template>
  <NCard title="CAN 接口" :bordered="true">
    <NForm :model="formValue" label-placement="left" label-width="80">
      <NFormItem label="COM Port">
        <NSelect v-model:value="formValue.comPort" :options="comPorts.map(port => ({ label: port, value: port }))" :disabled="isConnected" />
      </NFormItem>
      <NFormItem label="Bit Rate">
        <NSelect v-model:value="formValue.bitRate" :options="bitRates" :disabled="isConnected" />
      </NFormItem>
      <NSpace justify="right" :wrap="false">
        <NButton type="primary" @click="handleConnect"
          :disabled="isConnected || !formValue.comPort || !formValue.bitRate" size="large" ghost>
          Start
        </NButton>
        <NButton type="error" @click="handleDisconnect" :disabled="!isConnected" size="large" ghost>
          Stop
        </NButton>
      </NSpace>
    </NForm>
  </NCard>
</template>