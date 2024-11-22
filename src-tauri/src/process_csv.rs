use anyhow::{bail, Context, Ok, Result};
use csv::ReaderBuilder;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::sync::Mutex;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct CsvProcessor {
    path: Option<String>,
    x_axis: Option<Vec<f32>>,
    y_axis: Option<Vec<f32>>,
}

trait DataProcessor {
    fn merge_axis(&self) -> Vec<(f32, f32)>;
}

pub static CSV_PROCESSOR: Lazy<Mutex<CsvProcessor>> = Lazy::new(|| Mutex::new(CsvProcessor::new()));

#[allow(dead_code)]
impl CsvProcessor {
    pub fn new() -> Self {
        CsvProcessor {
            path: None,
            x_axis: None,
            y_axis: None,
        }
    }

    pub fn set_path(&mut self, csv: &str) -> Result<()> {
        self.path = Some(csv.to_string());
        Ok(())
    }

    pub fn get_col_data(&mut self, head: &str) -> Result<Vec<f32>> {
        let file = File::open(&self.path.as_ref().unwrap())?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .delimiter(b';')
            .from_reader(file);

        // 获取列名的索引
        let headers = rdr.headers()?;
        let column_index = headers
            .iter()
            .position(|header| header == head)
            .context(format!("Column '{}' not found", head))?;

        let mut result = Vec::new();

        // 遍历每一行，提取特定列的数据
        for record in rdr.records() {
            let record = record.context("Failed to read record")?;

            // 从每行数据中取出特定列的值
            let column_value = &record[column_index];
            result.push(column_value.parse::<f32>().unwrap());
        }

        Ok(result)
    }

    pub fn get_x_axis_data(&mut self) -> Result<()> {
        match self.get_col_data("erpm") {
            std::result::Result::Ok(data) => {
                self.x_axis = Some(data);
                Ok(())
            }
            Err(err) => {
                bail!("{}", err)
            }
        }
    }

    pub fn get_y_axis_data(&mut self) -> Result<()> {
        match self.get_col_data("current_in") {
            std::result::Result::Ok(data) => {
                self.y_axis = Some(data);
                Ok(())
            }
            Err(err) => {
                bail!("{}", err)
            }
        }
    }

    pub fn get_axis_data(&mut self) -> Result<Vec<(f32, f32)>> {
        Ok(self.merge_axis())
    }
}

impl DataProcessor for CsvProcessor {
    fn merge_axis(&self) -> Vec<(f32, f32)> {
        let mut coordinates = Vec::new();

        for (x, y) in self
            .x_axis
            .as_ref()
            .unwrap()
            .iter()
            .zip(self.y_axis.as_ref().unwrap().iter())
        {
            coordinates.push((x.clone(), y.clone()));
        }

        coordinates
    }
}
