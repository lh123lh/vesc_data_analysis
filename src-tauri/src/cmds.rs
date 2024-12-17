use crate::{
    motor::{self, MOTOR},
    process_csv::{self, DataProcessor, CSV_PROCESSOR},
};
use serialport;
use tauri::{AppHandle, Emitter};

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn process_csv_data(csv: process_csv::CsvFile) -> CmdResult<Vec<(f32, f32)>> {
    match CSV_PROCESSOR.lock().unwrap().get_axis_data(&csv) {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub fn calc_csv_residual(
    csv: process_csv::CsvFile,
    poly: process_csv::PolyParam,
) -> CmdResult<Vec<(i32, f32)>> {
    match CSV_PROCESSOR.lock().unwrap().calc_residual(&csv, &poly) {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn init_can_port(port: &str, bitrate: slcan::BitRate) -> CmdResult {
    println!("init ...");
    let mut motor = MOTOR.lock().await;
    match motor.open(port.to_string(), bitrate) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn deinit_can_port() -> CmdResult {
    println!("deinit ...");
    let mut motor = MOTOR.lock().await;

    if let Err(e) = motor.tx.send(false) {
        // 处理可能的错误
        eprintln!("Failed to send stop signal: {:?}", e);
    }
    motor.port = None;
    drop(motor);
    println!("deinit done");
    Ok(())
}

#[tauri::command]
pub async fn list_avaliable_ports() -> CmdResult<Vec<String>> {
    let mut ports = Vec::new();
    let ports_info = serialport::available_ports().expect("No ports found!");
    for p in ports_info {
        ports.push(p.port_name);
    }

    Ok(ports)
}

#[tauri::command]
pub async fn read_can_data() -> CmdResult {
    let mut motor = MOTOR.lock().await;

    let _ = motor.tx.send(true);
    motor.start_listening().await;
    drop(motor);

    Ok(())
}

#[tauri::command]
pub async fn write_can_data(id: u32, data: Vec<u8>) -> CmdResult {
    let mut motor = MOTOR.lock().await;
    let ret = motor.write_can_data(id, &data).await;
    drop(motor);

    match ret {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[tauri::command]
pub async fn set_recorder_path(csv: String) -> CmdResult {
    let mut motor = MOTOR.lock().await;
    let _ = motor.set_recorder_path(Some(csv)).await;
    Ok(())
}

#[tauri::command]
pub async fn start_recording(id: u32, msec: u64) -> CmdResult {
    let mut motor = MOTOR.lock().await;
    let _ = motor.tx.send(true);
    motor.start_listening().await;
    let _ = motor.auto_get_vesc_status(id, msec).await;

    Ok(())
}

#[tauri::command]
pub async fn stop_recording() -> CmdResult {
    let motor = MOTOR.lock().await;
    if let Err(e) = motor.tx.send(false) {
      // 处理可能的错误
      eprintln!("Failed to send stop signal: {:?}", e);
  }

    Ok(())
}

#[tauri::command]
pub async fn get_record_size(app: AppHandle, csv: String) -> CmdResult {
    app.emit("record-size", motor::get_file_size(&csv))
        .unwrap();

    Ok(())
}
