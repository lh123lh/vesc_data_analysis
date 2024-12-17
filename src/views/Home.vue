<script setup>
import { ref, onMounted } from 'vue';
import CanInterface from '../components/CanInterface.vue';

const availableSp = ref([
  {
    label: "COM7",
    value: "COM7"
  }
])

const currSp = ref('COM0');

</script>

<template>
  <!-- <n-card title="" class="card" style="min-height: 90vh;">

    <n-grid x-gap="2" :cols="24">
      <n-gi span="16">
        <n-select v-model:value="currSp" :options="availableSp" />
      </n-gi>
      <n-gi :offset="1" span="6">
        <n-select v-model:value="currSp" :options="availableSp" />
      </n-gi>
    </n-grid>
  </n-card> -->

  <!-- <NMessageProvider> -->
      <CanInterface />
    <!-- </NMessageProvider> -->
</template>