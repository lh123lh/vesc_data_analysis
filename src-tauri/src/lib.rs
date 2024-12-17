// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
extern crate slcan;

mod cmds;
mod process_csv;
mod motor;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmds::process_csv_data,
            cmds::calc_csv_residual,
            cmds::init_can_port,
            cmds::deinit_can_port,
            cmds::list_avaliable_ports,
            cmds::read_can_data,
            cmds::write_can_data,
            cmds::set_recorder_path,
            cmds::start_recording,
            cmds::stop_recording,
            cmds::get_record_size,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
