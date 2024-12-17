use anyhow::{bail, Ok, Result};
use bytes::{Buf, BufMut, BytesMut};
use crc::{self, Crc, CRC_16_XMODEM};
use csv;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use slcan::{self, Id, SocketCan};
use std::fs::{metadata, OpenOptions};
use std::{path::Path, sync::Arc};
use tokio::sync::{watch, Mutex};

pub fn get_file_size(file: &str) -> u64 {
    let path = Path::new(&file);

    match metadata(path) {
        std::result::Result::Ok(meta) => meta.len(),
        Err(_) => 0,
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Status {
    pub ext_pkt_id: u8,
    pub temp_fet: f32,
    pub temp_motor: f32,
    pub motor_current: f32,
    pub current_in: f32,
    pub id: f32,
    pub iq: f32,
    pub duty: f32,
    pub erpm: f32,
    pub input_voltage: f32,
    pub amp_hours: f32,
    pub amp_hours_charged: f32,
    pub watt_hours: f32,
    pub watt_hours_charged: f32,
    pub tachometer: i32,
    pub tachometer_abs: u32,
    pub fault_code: i8,
    pub pos_now: f32,
    pub controller_id: i8,
    pub temp_mos1: f32,
    pub temp_mos2: f32,
    pub temp_mos3: f32,
    pub vd: f32,
    pub vq: f32,
    pub kill_sw: i8,
}

impl Status {
    pub fn new() -> Self {
        Status {
            ext_pkt_id: 0,
            temp_fet: 0.0,
            temp_motor: 0.0,
            motor_current: 0.0,
            current_in: 0.0,
            id: 0.0,
            iq: 0.0,
            duty: 0.0,
            erpm: 0.0,
            input_voltage: 0.0,
            amp_hours: 0.0,
            amp_hours_charged: 0.0,
            watt_hours: 0.0,
            watt_hours_charged: 0.0,
            tachometer: 0,
            tachometer_abs: 0,
            fault_code: 0,
            pos_now: 0.0,
            controller_id: 0,
            temp_mos1: 0.0,
            temp_mos2: 0.0,
            temp_mos3: 0.0,
            vd: 0.0,
            vq: 0.0,
            kill_sw: 0,
        }
    }

    pub fn parse_status(&mut self, buf: &mut BytesMut) -> Result<()> {
        self.ext_pkt_id = buf.get_u8();
        self.temp_fet = buf.get_i16() as f32 / 10.0;
        self.temp_motor = buf.get_i16() as f32 / 10.0;
        self.motor_current = buf.get_i32() as f32 / 100.0;
        self.current_in = buf.get_i32() as f32 / 100.0;
        self.id = buf.get_i32() as f32 / 100.0;
        self.iq = buf.get_i32() as f32 / 100.0;
        self.duty = buf.get_i16() as f32 / 1000.0;
        self.erpm = buf.get_i32() as f32;
        self.input_voltage = buf.get_i16() as f32 / 10.0;
        self.amp_hours = buf.get_i32() as f32 / 10000.0;
        self.amp_hours_charged = buf.get_i32() as f32 / 10000.0;
        self.watt_hours = buf.get_i32() as f32 / 10000.0;
        self.watt_hours_charged = buf.get_i32() as f32 / 10000.0;
        self.tachometer = buf.get_i32();
        self.tachometer_abs = buf.get_u32();
        self.fault_code = buf.get_i8();
        self.pos_now = buf.get_i32() as f32 / 100000.0;
        self.controller_id = buf.get_i8();
        self.temp_mos1 = buf.get_i16() as f32 / 10.0;
        self.temp_mos2 = buf.get_i16() as f32 / 10.0;
        self.temp_mos3 = buf.get_i16() as f32 / 10.0;
        self.vd = buf.get_i32() as f32 / 1000.0;
        self.vq = buf.get_i32() as f32 / 1000.0;
        self.kill_sw = buf.get_i8();

        Ok(())
    }

    pub fn save_status(&mut self, buf: &mut BytesMut, file: &str) -> Result<()> {
        let csv_path = Path::new(file);
        let csv = OpenOptions::new()
            .write(true)
            .append(true)
            .create(true) // 如果文件不存在，则创建
            .open(file)?;

        let is_empty = metadata(csv_path)?.len() == 0;

        let mut wtr = csv::WriterBuilder::new()
            .has_headers(is_empty)
            .delimiter(b';')
            .from_writer(csv);
        let _ = self.parse_status(buf)?;

        wtr.serialize(self)?;
        wtr.flush()?;

        Ok(())
    }
}

pub struct Motor {
    pub port: Option<Arc<Mutex<Box<dyn SocketCan + Send>>>>,
    // pub sender: Option<mpsc::Sender<&'static str>>,
    // pub receiver: Option<mpsc::Receiver<&'static str>>,
    pub tx: watch::Sender<bool>,
    pub rx: watch::Receiver<bool>,
    // pub start_listening: Option<Arc<Mutex<bool>>>,
    // pub recoder_handle: Option<Box<csv::Writer<std::fs::File>>>,
    pub csv: Arc<Mutex<Option<String>>>,
}

pub static MOTOR: Lazy<Mutex<Motor>> = Lazy::new(|| Mutex::new(Motor::new()));

// pub static MOTOR: Lazy<Arc<Motor>> = Lazy::new(|| Arc::new(Motor::new()));

#[allow(dead_code)]
impl Motor {
    pub fn new() -> Self {
        let (tx, rx) = watch::channel(true); // 创建一个 watch 通道，默认值为 true
                                             // let (mpsc_tx, mpsc_rx) = mpsc::channel(32);
        Motor {
            port: None,
            // sender: Some(mpsc_tx),
            // receiver: Some(mpsc_rx),
            tx: tx,
            rx: rx,
            csv: Arc::new(Mutex::new(None)),
        }
    }

    pub fn open(&mut self, port: String, bitrate: slcan::BitRate) -> Result<()> {
        println!("{}, {:?}", port, bitrate);

        let mut port = slcan::CanSocket::new(port, bitrate);
        match port.open() {
            std::result::Result::Ok(_) => {
                self.port = Some(Arc::new(Mutex::new(Box::new(port))));
                Ok(())
            }
            Err(_) => bail!("Failed To connect port".to_string()),
        }
    }

    async fn parse_buf(&self, buf: &mut BytesMut) -> Result<()> {
        let mut csv_lock = self.csv.lock().await;
        if let Some(ref mut csv) = *csv_lock {
            let mut status = Status::new();
            match status.save_status(buf, &csv) {
                std::result::Result::Ok(_) => {}
                Err(err) => {
                    println!("write csv failed: {}", err);
                }
            }
        }

        Ok(())
    }

    pub async fn start_listening(&mut self) {
        // 确保 sender 已经被设置
        // let sender = self.sender.as_ref().expect("Sender not set");

        let mut buf = BytesMut::new();
        let csv = self.csv.clone();

        let rx = self.rx.clone();
        // 使用 tokio::task::spawn 启动异步任务
        let port = self.port.clone();

        // let self_clone = Arc::clone(&self);

        if let Some(port) = port {
            // let mut buffer: Vec<u8> = vec![0; 1024];
            tokio::spawn(async move {
                loop {
                    if !*rx.borrow() {
                        println!("Listening stopped, exiting task.");
                        break; // 如果 rx 为 false，则退出任务
                    }

                    match port.lock().await.read() {
                        std::result::Result::Ok(frame) => {
                            let mut id = 0;
                            match frame.id {
                                slcan::Id::Extended(eid) => {
                                    id = eid.as_raw();
                                }
                                _ => {}
                            }

                            // if id != 0x0908 && id != 0x1008 {
                            //     println!(
                            //         "id: {:#x}, DLC: {}, Data: {:?}",
                            //         id, frame.dlc, frame.data
                            //     )
                            // }

                            // println!("{:?}", csv);

                            if id == 0x0565 {
                                buf.put_slice(&frame.data[1..frame.dlc]);
                            } else if id == 0x0765 {
                                let chk = (frame.data[4] as u16) << 8 | frame.data[5] as u16;
                                let len = (frame.data[2] as u16) << 8 | frame.data[3] as u16;

                                let crc = Crc::<u16>::new(&CRC_16_XMODEM);
                                if crc.checksum(&buf) == chk && buf.len() == len as usize {
                                    // println!("{:?}", buf.to_vec());

                                    if let Some(file) = csv.lock().await.clone() {
                                        let mut status = Status::new();
                                        match status.save_status(&mut buf, &file) {
                                            std::result::Result::Ok(_) => {}
                                            Err(err) => {
                                                println!("write csv failed: {}", err);
                                            }
                                        }
                                    }
                                } else {
                                    println!("Error crc, {:#x}, {:#x}", chk, crc.checksum(&buf));
                                }

                                buf.clear();

                                // break;
                            }
                        }
                        Err(error) => match error.kind() {
                            std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => (),
                            _ => eprintln!("{:?}", error),
                        },
                    }

                    // tokio::time::sleep(tokio::time::Duration::from_micros(10)).await;
                }
            });
        }
    }

    pub async fn write_can_data(&mut self, id: u32, data: &[u8]) -> Result<()> {
        let port = self.port.clone();

        let rx = self.rx.clone();

        if !*rx.borrow() {
            return Ok(());
        }

        port.unwrap()
            .lock()
            .await
            .write(Id::Extended(slcan::ExtendedId::new(id).unwrap()), data)?;

        Ok(())
    }

    pub async fn set_recorder_path(&mut self, csv: Option<String>) -> Result<()> {
        let mut csv_lock = self.csv.lock().await;

        // 如果 csv 不为空，则清空文件内容
        if let Some(ref path) = csv {
            if !path.is_empty() {
                let file_path = Path::new(path);
                // 清空文件内容
                let file = std::fs::File::create(file_path)?;
                file.set_len(0)?; // 清空文件
            }
        }

        *csv_lock = csv;
        Ok(())
    }

    pub async fn get_vesc_status(&mut self, target: u32) -> Result<()> {
        let id = 0x08 << 8 | target;
        let data = [0x65 as u8, 0x00, 0x04];

        let _ = self.write_can_data(id, &data).await;

        Ok(())
    }

    pub async fn auto_get_vesc_status(&mut self, target: u32, interval: u64) {
        let id = 0x08 << 8 | target;
        let data = [0x65 as u8, 0x00, 0x04];

        let rx = self.rx.clone();
        // 使用 tokio::task::spawn 启动异步任务
        let port = self.port.clone();

        if let Some(port) = port {
            // let mut buffer: Vec<u8> = vec![0; 1024];
            tokio::spawn(async move {
                loop {
                    if !*rx.borrow() {
                        println!("Auto task stopped, exiting task.");
                        break; // 如果 rx 为 false，则退出任务
                    }

                    port.lock()
                        .await
                        .write(Id::Extended(slcan::ExtendedId::new(id).unwrap()), &data)
                        .unwrap();

                    tokio::time::sleep(tokio::time::Duration::from_millis(interval)).await;
                }
            });
        }
    }
}
