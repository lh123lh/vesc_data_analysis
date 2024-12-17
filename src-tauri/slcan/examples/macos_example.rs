// /*
//  * @Author: Liuheng
//  * @Date: 2024-12-10 18:50:31
//  * @LastEditors: Liuheng
//  * @LastEditTime: 2024-12-10 19:02:21
//  * @Description: file content
//  */

use slcan::SocketCan;

fn main() {
    let mut can = slcan::CanSocket::new("COM7".to_string(), slcan::BitRate::Setup500Kbit);

    // can.close().unwrap();
    can.open().unwrap();

    loop {
        match can.read() {
            Ok(frame) => println!("{}", frame),
            Err(error) => match error.kind() {
                std::io::ErrorKind::TimedOut | std::io::ErrorKind::WouldBlock => (),
                _ => eprintln!("{:?}", error),
            },
        }
    }
}