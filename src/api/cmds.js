import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core';

function notify_info(notification, msg) {
  // notification.info({
  //   content: "说点啥呢",
  //   meta: msg,
  //   duration: 2500,
  //   keepAliveOnHover: true
  // });
}

function notify_failed(notification, msg) {
  // notification.error({
  //   content: "说点啥呢",
  //   meta: msg,
  //   duration: 2500,
  //   keepAliveOnHover: true
  // });
}

function cmd_process_csv_data(path) {
  return new Promise(function (resolve, reject) {
    invoke('process_csv_data', { csv: path })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        console.log(error)
        // notify_failed(error)
      })
  })
}

export default {
  notify_info,
  notify_failed,
  cmd_process_csv_data,
}