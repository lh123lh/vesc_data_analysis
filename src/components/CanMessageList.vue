<script setup>
import { NDataTable } from 'naive-ui';
import { format } from 'date-fns';

defineProps({
  messages: {
    type: Array,
    default: () => []
  }
});

const columns = [
  {
    title: 'Time',
    key: 'timestamp',
    width: 120,
    render: (row) => format(new Date(row.timestamp), 'HH:mm:ss.SSS')
  },
  {
    title: 'ID',
    key: 'id',
    width: 80,
    render: (row) => `0x${row.id.toString(16).toUpperCase().padStart(3, '0')}`
  },
  {
    title: 'DLC',
    key: 'dlc',
    width: 60
  },
  {
    title: 'Data',
    key: 'data',
    render: (row) => row.data.map(byte => byte.toString(16).toUpperCase().padStart(2, '0')).join(' ')
  }
];
</script>

<template>
  <NDataTable
    :columns="columns"
    :data="messages"
    :bordered="true"
    :single-line="false"
    size="small"
    :max-height="525"
    :scroll-x="700"
  />
</template>