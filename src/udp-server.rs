use std::net::UdpSocket;
use std::{io, str};
use std::env;

fn main() -> std::io::Result<()> {
    println!(
        "udp server starting ... "      
    );
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("useage：udpServer 10.97.24.182 19000");
        return Ok(());
    }
    println!("{:?}", args);
    let rport = args[2].as_str();
    let port: i32 = rport.parse::<i32>().unwrap();
    let mut rudpip = args[1].clone();
    rudpip = rudpip + ":";
    let mut bondrip = rudpip.clone();
    let bport: i32 = port;
    bondrip = bondrip + bport.to_string().as_str(); 
    println!("bind ip {} ", bondrip);
    let socket = UdpSocket::bind(bondrip)?;

    loop {
        let mut buf = [0u8; 1500];
        let (amt, src) = socket.recv_from(&mut buf)?;

        println!(
            "recv: {}",
            str::from_utf8(&buf).expect("Could not write buffer as string")
        );
        let buf = &mut buf[..amt];
        buf.reverse();
        socket.send_to(buf, &src)?;
        println!(
            "sended: {}",
            str::from_utf8(&buf).expect("Could not write buffer as string")
        );
    }
}