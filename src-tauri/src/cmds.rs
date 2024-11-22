use crate::process_csv::CSV_PROCESSOR;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn process_csv_data(csv: &str) -> CmdResult<Vec<(f32, f32)>> {
    CSV_PROCESSOR.lock().unwrap().set_path(csv).unwrap();

    match CSV_PROCESSOR.lock().unwrap().get_x_axis_data() {
        Ok(()) => {}
        Err(err) => {
            return Err(err.to_string());
        }
    }

    match CSV_PROCESSOR.lock().unwrap().get_y_axis_data() {
        Ok(()) => {}
        Err(err) => {
            return Err(err.to_string());
        }
    }

    match CSV_PROCESSOR.lock().unwrap().get_axis_data() {
        Ok(data) => Ok(data),
        Err(err) => Err(err.to_string()),
    }
}
