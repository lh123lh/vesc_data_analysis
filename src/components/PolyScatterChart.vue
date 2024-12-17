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
  dataset: [
    {
      source: [[0, 0]],
    },
    {
      transform: {
        type: 'ecStat:regression',
        config: { method: 'polynomial', order: 2 }
      }
    }
  ],
  title: {
    text: 'ERPM - Current',
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
    splitNumber: 20
  },
  yAxis: {
    splitLine: {
      lineStyle: {
        type: 'dashed'
      }
    },
    splitNumber: 15,
    // min: 0,
  },
  series: [
    {
      name: 'scatter',
      type: 'scatter',
      symbolSize: 6,
      color: "#5B9BD5",
    },
    {
      name: 'line',
      type: 'line',
      smooth: true,
      datasetIndex: 1,
      symbolSize: 0.1,
      symbol: 'circle',
      label: { show: false, fontSize: 16 }, // 多项式公式, 目前存在浮点数精度丢失的问题, 暂不使用
      labelLayout: { dx: -60 },
      encode: { label: 2, tooltip: 0 },
      color: "#9FE080",
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
  xMin: {
    type: Number,
    default: '0',
  },
  yMin: {
    type: Number,
    default: '0',
  },
});

onMounted(() => {
  echarts.registerTransform(ecStat.transform.regression);

  if (chartContainer.value) {
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
  }
});

onActivated(() => {
  chartInstance.resize();
})

watch(() => props.value, () => {
  // console.log(props.value)
  const regressionResult = ecStat.regression('polynomial', props.value, {
    order: 2 // 二次多项式回归
  });

  const coefficients = regressionResult.parameter; // [a, b, c]

  // 格式化回归系数
  const aFormatted = coefficients[2].toExponential(2);
  const bFormatted = coefficients[1].toExponential(2);
  const cFormatted = coefficients[0].toFixed(4);

  let formula = ``;
  if (bFormatted < 0) {
    formula = `y = ${aFormatted}x² - ${-bFormatted}x + ${cFormatted}`;
  } else {
    formula = `y = ${aFormatted}x² + ${bFormatted}x + ${cFormatted}`;
  }

  if (chartInstance) {
    chartInstance.setOption({
      title: [{
        subtext: `${formula}`
      }],
      dataset: [{
        source: props.value
      }],
      xAxis: [{
        min: props.xMin
      }],
      yAxis: [{
        min: props.yMin
      }],
    });
  }
});
</script>

<template>
  <div ref="chartContainer" :style="{ width: props.width, height: props.height, textAlign: 'center' }"></div>
</template>
