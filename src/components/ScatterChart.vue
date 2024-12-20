<script setup>
import { onMounted, ref, onUpdated, computed, watch, onActivated } from 'vue';
import * as echarts from 'echarts/core';
import {
  DatasetComponent,
  TitleComponent,
  TooltipComponent,
  GridComponent,
  TransformComponent
} from 'echarts/components';
import { ScatterChart, LineChart } from 'echarts/charts';
import { UniversalTransition, LabelLayout } from 'echarts/features';
import { CanvasRenderer } from 'echarts/renderers';
import ecStat from 'echarts-stat';

echarts.use([
  DatasetComponent,
  TitleComponent,
  TooltipComponent,
  GridComponent,
  TransformComponent,
  ScatterChart,
  LineChart,
  CanvasRenderer,
  UniversalTransition,
  LabelLayout
]);

const chartContainer = ref(null);
let chartInstance = null;

const option = {
  title: {
    text: '',
    subtext: '',
    left: 'center',
    top: 0
  },
  tooltip: {
    trigger: 'axis',
    axisPointer: {
      type: 'cross'
    }
  },
  xAxis: {
    splitLine: {
      lineStyle: {
        type: 'dashed'
      }
    },
    // splitNumber: 20
  },
  yAxis: {
    splitLine: {
      lineStyle: {
        type: 'dashed'
      }
    },
    // splitNumber: 15,
    // min: 0,
  },
  series: [
    {
      type: 'scatter',
      symbolSize: 6,
      color: "#5B9BD5",
      data: []
    }
  ],
  grid: {
    left: '2%',   // 绘制区域距离左侧的距离
    right: '2%',  // 绘制区域距离右侧的距离
    top: '5%',    // 绘制区域距离上侧的距离
    bottom: '5%'  // 绘制区域距离下侧的距离
  }
};

const props = defineProps({
  title: {
    type: String,
    default: ""
  },
  value: {
    type: Array,
    required: true,
    default: () => []
  },
  width: {
    type: String,
    default: '100%' // 默认宽度
  },
  height: {
    type: String,
    default: '50%' // 默认高度
  },
});

onMounted(() => {
  chartInstance = echarts.init(chartContainer.value);
  chartInstance.setOption(option);

  chartInstance.setOption({
    title: [{
      text: props.title
    }],
  });

  window.addEventListener("resize", () => {
    chartInstance.resize();
  });
});

onActivated(() => {
  chartInstance.resize();
})

watch(() => props.value, () => {

  if (chartInstance) {
    chartInstance.setOption({
      // title: [{
      //   subtext: `${formula}`
      // }],
      series: [{
        data: props.value
      }]
    });
  }
});
</script>

<template>
  <div ref="chartContainer" :style="{ width: props.width, height: props.height, textAlign: 'center' }"></div>
</template>
