use std::env;
use std::net::{Ipv4Addr, UdpSocket};
use std::{io, str};
fn main() -> std::io::Result<()> {
    println!("udp client starting ... ");
    let args: Vec<String> = env::args().collect();
    let mut speed_test = 0;
    if args.len() < 3 {
        println!("useage: updClient 10.97.24.182 19000");
        println!("useage: updClient 10.97.24.182 19000 package_len send_delay package_number");
        return Ok(());
    } else if args.len() > 3 {
        speed_test = 1;
    }
    println!("{:?}", args);
    let rport = args[2].as_str();
    let port: i32 = rport.parse::<i32>().unwrap();
    let mut rudpip = args[1].clone();
    rudpip = rudpip + ":";
    // let mut bondrip = String::new()+"127.0.0.1:";
    let bport: i32 = port + 1;
    // bondrip = bondrip + bport.to_string().as_str();
    rudpip = rudpip + rport;
    println!("connect ip {}", rudpip);
    // let socket = UdpSocket::bind(bondrip)?;
    let bbport: u16 = bport as u16;
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, bbport)).unwrap();
    socket.connect(rudpip)?;
    if speed_test == 0 {
        loop {
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            socket.send(input.as_bytes())?;
            let mut buffer = [0u8; 1500];
            socket.recv_from(&mut buffer)?;
            println!(
                "recv: {}",
                str::from_utf8(&buffer).expect("Could not write buffer as string")
            );
        }
    } else if speed_test == 1 {
        let package_len = args[3].clone();
        println!("plen {}", package_len);
        let plen: usize = package_len.as_str().parse::<usize>().unwrap();
        let pdelay = args[4].clone();
        let delay: u64 = pdelay.as_str().parse::<u64>().unwrap();
        let pnumber = args[5].clone();
        let mut count: u16 = pnumber.as_str().parse::<u16>().unwrap();
        println!(
            "begin udp speed test package len: {}, delay : {}, count : {} !",
            plen, delay, count
        );
        let mut seq: char = '0'; // 48 0x30 字符0
                                 // 循环发送固定长度的字符串，字符串内容为 字符0 到字符~之间的可视化字符，超过循环组织
        loop {
            // let buf = [0u8,plen];
            let mut buffer = vec![0u8];
            buffer = buffer.repeat(plen as usize);
            buffer.fill(seq as u8);
            let bufs = String::from_utf8(buffer).unwrap();
            let buf = bufs.as_bytes();
            socket.send(&buf)?;
            let dur = std::time::Duration::from_millis(1 * delay);
            std::thread::sleep(dur);
            count = count - 1;
            seq = (seq as u8 + 1u8) as char;
            if seq == '~' {
                // 126 0x7E 避免溢出
                seq = '0';
            }
            println!("count {} , buf {:?} ", count, seq);
            if count == 0 {
                break;
            }
        }
        println!("Done !");
        return Ok(());
    } else {
        let dur = std::time::Duration::from_millis(10);
        std::thread::sleep(dur);
        return Ok(());
    }
}
