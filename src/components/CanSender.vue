<script setup>
import { ref } from 'vue';
import { NCard, NForm, NFormItem, NInput, NInputNumber, NButton, NSpace, useMessage } from 'naive-ui';

const emit = defineEmits(['send']);
const message = useMessage();

const props = defineProps({
  isConnected: {
    type: Boolean,
    default: false
  }
});

const formValue = ref({
  id: '',
  dlc: 8,
  data: ''
});

const validateHex = (value, pattern) => {
  if (!value) return true;
  return new RegExp(pattern).test(value);
};

const rules = {
  id: [
    { required: true, message: 'Please input CAN ID' },
    { validator: (rule, value) => validateHex(value, '^[0-9A-Fa-f]{1,8}$'), message: 'Invalid hex format (max 8 digits)' }
  ],
  data: [
    { required: true, message: 'Please input data bytes' },
    { validator: (rule, value) => validateHex(value, '^[0-9A-Fa-f\\s]{1,23}$'), message: 'Invalid hex format (space-separated bytes)' }
  ]
};

const handleSend = () => {
  try {
    const id = parseInt(formValue.value.id, 16);
    const data = formValue.value.data.split(/\s+/).map(byte => parseInt(byte, 16));
    
    if (data.length !== formValue.value.dlc) {
      message.error(`Data length (${data.length}) does not match DLC (${formValue.value.dlc})`);
      return;
    }
    
    if (data.some(byte => byte < 0 || byte > 255)) {
      message.error('Data bytes must be between 00 and FF');
      return;
    }

    emit('send', {
      timestamp: Date.now(),
      id,
      dlc: formValue.value.dlc,
      data
    });
    
    message.success('Message sent');
  } catch (error) {
    message.error('Invalid message format');
  }
};
</script>

<template>
  <NCard title="Send CAN Message" :bordered="true">
    <NForm
      ref="formRef"
      :model="formValue"
      :rules="rules"
      label-placement="left"
      label-width="80"
      require-mark-placement="right-hanging"
    >
      <NFormItem label="ID (hex)" path="id">
        <NInput
          v-model:value="formValue.id"
          placeholder="Enter ID in hex (e.g., 18DB33F1)"
          :disabled="!isConnected"
        />
      </NFormItem>
      <NFormItem label="DLC" path="dlc">
        <NInputNumber
          v-model:value="formValue.dlc"
          :min="0"
          :max="8"
          :disabled="!isConnected"
        />
      </NFormItem>
      <NFormItem label="Data" path="data">
        <NInput
          v-model:value="formValue.data"
          placeholder="Space-separated hex bytes (e.g., FF 00 A1 B2)"
          :disabled="!isConnected"
        />
      </NFormItem>
      <NSpace justify="center">
        <NButton
          type="primary"
          @click="handleSend"
          :disabled="!isConnected"
          size="large"
        >
          Send Message
        </NButton>
      </NSpace>
    </NForm>
  </NCard>
</template>