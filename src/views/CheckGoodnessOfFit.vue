<script setup>
import { ref, onMounted } from 'vue';
import ScatterChart from '../components/ScatterChart.vue';
import { open } from '@tauri-apps/plugin-dialog';
import cmds from '../api/cmds';
import { useMessage } from "naive-ui";

const data = ref()

const formRef = ref(null);
const message = useMessage();
const formVal = ref({
  csvPath: "",
  cfg: {
    xAxis: "6500",
    yAxis: "6",
    xName: "erpm",
    yName: "current_in",
  },
  param: {
    a: "1.03E-06",
    b: "-0.0128",
    c: "46.51659091"
  },
});

const formRules = {
  csvPath: {
    required: true,
    message: "请选择CSV文件",
    trigger: ["input"]
  },
  cfg: {
    xAxis: {
      required: true,
      message: "请输入姓名",
      trigger: "blur"
    },
    yAxis: {
      required: true,
      message: "请输入年龄",
      trigger: ["input", "blur"]
    },
    xName: {
      required: true,
      message: "x轴名称",
      // trigger: "blur"
    },
    yName: {
      required: true,
      message: "y轴名称",
      // trigger: "blur"
    },
  },
  param: {
    a: {
      required: true,
      message: "请输入a值",
      // trigger: "blur"
    },
    b: {
      required: true,
      message: "请输入b值",
      // trigger: ["input", "blur"]
    },
    c: {
      required: true,
      message: "请输入c值",
      // trigger: ["input", "blur"]
    }
  },
};

async function selectCsv() {
  await open({
    multiple: true,
    filters: [{
      name: 'CSV',
      extensions: ['csv']
    }]
  }).then((file) => {
    formVal.value.csvPath = file.toString()
  })
}

async function calc_csv_residual(e) {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      // message.success("Valid");
    } else {
      console.log(errors);
      message.error("Invalid");
    }
  });

  if (formVal.value.csvPath.length == 0) {
    message.error("请选择CSV文件")
    return;
  }

  await cmds.cmd_calc_csv_residual(
    formVal.value.csvPath,
    formVal.value.cfg.xName,
    formVal.value.cfg.yName,
    parseFloat(formVal.value.cfg.xAxis),
    parseFloat(formVal.value.cfg.yAxis),
    parseFloat(formVal.value.param.a),
    parseFloat(formVal.value.param.b),
    parseFloat(formVal.value.param.c),)
    .then((axis) => {
      data.value = axis
    }).catch(err => {
      message.error(err);
    });
}

</script>

<template>
  <n-card title="" class="card">

    <n-form ref="formRef" label-width="auto" :model="formVal" :formRules="formRules" label-placement="left"
      require-mark-placement="right-hanging" style="margin-top: -1rem; margin-bottom: -1rem;">

      <n-form-item label="CSV" path="csvPath">
        <n-grid x-gap="6" :cols="25">
          <n-gi span="22">
            <n-input v-model:value="formVal.csvPath" readonly="true" placeholder="请选择CSV文件" />
          </n-gi>
          <n-gi>
            <n-button @click="selectCsv">导入CSV</n-button>
          </n-gi>
        </n-grid>
      </n-form-item>

    </n-form>

    <n-form ref="formRef" label-width="6rem" :model="formVal" :formRules="formRules"
      require-mark-placement="right-hanging" style="margin-top: 1rem; margin-bottom: -1rem;">

      <n-grid x-gap="6" :cols="55">
        <n-gi span="7">
          <n-form-item label="X轴名称" path="cfg.xName" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.xName" placeholder="X轴名称" />
          </n-form-item>
        </n-gi>
        <n-gi span="7">
          <n-form-item label="x最小值" path="cfg" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.xAxis" placeholder="X轴最小值" />
          </n-form-item>
        </n-gi>
        <n-gi span="7">
          <n-form-item label="Y轴名称" path="cfg.yName" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.yName" placeholder="X轴名称" />
          </n-form-item>
        </n-gi>
        <n-gi span="7">
          <n-form-item label="y最小值" path="cfg" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.yAxis" placeholder="Y轴最小值" />
          </n-form-item>
        </n-gi>

        <n-gi span="7">
          <n-form-item label="多项式a值" path="param.a" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.param.a" placeholder="请输入a值" />
          </n-form-item>
        </n-gi>
        <n-gi span="7">
          <n-form-item label="多项式b值" path="param.b" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.param.b" placeholder="请输入b值" />
          </n-form-item>
        </n-gi>
        <n-gi span="7">
          <n-form-item label="多项式c值" path="param.c" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.param.c" placeholder="请输入c值" />
          </n-form-item>
        </n-gi>

        <n-gi style="margin-left: auto;">
          <n-form-item path="param" style="margin-top: -1rem; margin-bottom: -1rem;">
            <n-button attr-type="button" @click="calc_csv_residual">
              处理
            </n-button>
          </n-form-item>
        </n-gi>
      </n-grid>

    </n-form>

    <ScatterChart title="误差" :value=data height="72vh" />
  </n-card>
</template>

<style scoped>
/* .n-card {
  max-height: 300px;
} */

/* .card {
  height: 90vh;
}

@media (min-height: 760px) {
  .card {
    height: 92vh;
  }
} */
</style>