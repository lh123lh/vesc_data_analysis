<script setup>
import { ref } from 'vue';
import { NCard, NStatistic, NSpace, NNumberAnimation } from 'naive-ui';

const props = defineProps({
  elapsed: {
    type: String,
    default: "00:00:00"
  },
  recordSize: {
    type: String,
    default: "0 KB"
  }
});
</script>

<template>
  <NCard title="统计" :bordered="true">
    <n-grid x-gap="4" :cols="24">
        <n-gi :offset="1" span="10">
          <n-statistic label="采集时长">
              {{ elapsed }}
            </n-statistic>
        </n-gi>
        <n-gi :offset="2" span="10">
          <n-statistic label="文件大小">
              {{ recordSize }}
            </n-statistic>
        </n-gi>
      </n-grid>
  </NCard>
</template>