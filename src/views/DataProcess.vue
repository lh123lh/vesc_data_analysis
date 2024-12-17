<script setup>
import { ref, onMounted } from 'vue';
import ScatterChart from '../components/PolyScatterChart.vue';
import { open } from '@tauri-apps/plugin-dialog';
import { listen } from '@tauri-apps/api/event';
import cmds from '../api/cmds';
import { useMessage } from "naive-ui";

const data = ref()

const formRef = ref(null);
const message = useMessage();
const formVal = ref({
  csvPath: "",
  cfg: {
    xAxis: "0",
    yAxis: "0",
    xName: "erpm",
    yName: "current_in",
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
      trigger: "blur"
    },
    yName: {
      required: true,
      message: "y轴名称",
      trigger: "blur"
    },
  },
};

const handleDataProcessed = (event) => {
  // console.log(event.payload);
};

onMounted(() => {
  listen('data-processed', handleDataProcessed);
})

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

async function process_csv_data(e) {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      // message.success("Valid");
    } else {
      message.error("Invalid");
    }
  });
  if (formVal.value.csvPath.length == 0) {
    message.error("请选择CSV文件")
    return;
  }

  await cmds.cmd_process_csv_data(
    formVal.value.csvPath,
    formVal.value.cfg.xName,
    formVal.value.cfg.yName,
    parseFloat(formVal.value.cfg.xAxis),
    parseFloat(formVal.value.cfg.yAxis))
    .then((axis) => {
      data.value = axis
    }).catch(err => {
      message.error(err);
    });
}

</script>

<template>
  <n-card title="" class="card">

    <!-- <n-grid x-gap="4" :cols="48">
      <n-gi span="38">
        <n-input v-model:value="csvPath" type="text" readonly="true" placeholder="" />
      </n-gi>
      <n-gi :offset="1" span="4">
        <n-button @click="selectCsv">导入CSV</n-button>
      </n-gi>
      <n-gi :offset="2" span="1">
        <n-button @click="process_csv_data">处理</n-button>
      </n-gi>
    </n-grid> -->

    <!-- <n-grid x-gap="6" :cols="48">
      <n-gi span="43">
        <n-input v-model:value="csvPath" type="text" readonly="true" placeholder="" />
      </n-gi>
      <n-gi :offset="0">
        <n-button @click="selectCsv">导入CSV</n-button>
      </n-gi>
    </n-grid> -->

    <n-form ref="formRef" label-width="5rem" :model="formVal" :formRules="formRules" label-placement="left"
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

      <!-- <n-form-item label="设置" path="cfg" style="margin-top: -1rem; margin-bottom: -1rem;">
        <n-grid x-gap="6" :cols="25">
          <n-gi span="11">
            <n-input v-model:value="formVal.cfg.xAxis" placeholder="X轴最小值" />
          </n-gi>
          <n-gi span="11">
            <n-input v-model:value="formVal.cfg.yAxis" placeholder="Y轴最小值" />
          </n-gi>

          <n-gi style="margin-left: auto;">
            <n-button attr-type="button" @click="process_csv_data">
              处理
            </n-button>
          </n-gi>
        </n-grid>
      </n-form-item> -->

      <n-grid x-gap="6" :cols="50">
        <n-gi span="11">
          <n-form-item label="X轴名称" path="cfg.xName" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.xName" placeholder="X轴最小值" />
          </n-form-item>
        </n-gi>
        <n-gi span="11">
          <n-form-item label="x最小值" path="cfg" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.xAxis" placeholder="X轴最小值" />
          </n-form-item>
        </n-gi>
        <n-gi span="11">
          <n-form-item label="Y轴名称" path="cfg.yName" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.yName" placeholder="X轴最小值" />
          </n-form-item>
        </n-gi>
        <n-gi span="11">
          <n-form-item label="y最小值" path="cfg" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-input v-model:value="formVal.cfg.yAxis" placeholder="Y轴最小值" />
          </n-form-item>
        </n-gi>

        <n-gi style="margin-left: auto;">
          <n-form-item path="cfg" style="margin-top: -1rem; margin-bottom: 0rem;">
            <n-button attr-type="button" @click="process_csv_data">
              处理
            </n-button>
          </n-form-item>
        </n-gi>
      </n-grid>

    </n-form>


    <ScatterChart title="Current - ERPM" :value=data :xMin="parseInt(formVal.cfg.xAxis)"
      :yMin="parseInt(formVal.cfg.yAxis)" height="73vh" />
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