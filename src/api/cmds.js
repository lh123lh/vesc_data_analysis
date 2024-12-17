import { ref, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core';

function cmd_process_csv_data(path, xName, yName, xfilter, yfilter) {
  return new Promise(function (resolve, reject) {
    invoke('process_csv_data', {
      csv: {
        file: path,
        x_name: xName,
        y_name: yName,
        x_filter: xfilter,
        y_filter: yfilter
      }
    })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_calc_csv_residual(path, xName, yName, xfilter, yfilter, a, b, c) {
  return new Promise(function (resolve, reject) {
    invoke('calc_csv_residual', {
      csv: {
        file: path,
        x_name: xName,
        y_name: yName,
        x_filter: xfilter,
        y_filter: yfilter
      },
      poly: {
        a: a, b: b, c: c,
      }
    })
      .then((data) => {
        resolve(data)
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_get_avaliable_ports() {
  return new Promise(function (resolve, reject) {
    invoke('list_avaliable_ports', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_init_can_port(port, bitrate) {
  return new Promise(function (resolve, reject) {
    invoke('init_can_port', { port: port, bitrate: bitrate })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_deinit_can_port() {
  return new Promise(function (resolve, reject) {
    invoke('deinit_can_port', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_read_can_data() {
  return new Promise(function (resolve, reject) {
    invoke('read_can_data', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_write_can_data(id, data) {
  return new Promise(function (resolve, reject) {
    invoke('write_can_data', { id: id, data: data })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_start_recording(id, msec) {
  return new Promise(function (resolve, reject) {
    invoke('start_recording', { id: id, msec: msec })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_stop_recording() {
  return new Promise(function (resolve, reject) {
    invoke('stop_recording', {})
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_set_recorder_path(csv) {
  return new Promise(function (resolve, reject) {
    invoke('set_recorder_path', { csv: csv })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

function cmd_get_record_size(csv) {
  return new Promise(function (resolve, reject) {
    invoke('get_record_size', { csv: csv })
      .then((data) => {
        resolve(data);
      })
      .catch((error) => {
        reject(error);
      })
  })
}

export default {
  cmd_process_csv_data,
  cmd_calc_csv_residual,
  cmd_get_avaliable_ports,
  cmd_init_can_port,
  cmd_deinit_can_port,
  cmd_read_can_data,
  cmd_write_can_data,
  cmd_start_recording,
  cmd_stop_recording,
  cmd_set_recorder_path,
  cmd_get_record_size,
}