use std::env; 
use std::{io, str};
use std::net::{Ipv4Addr, UdpSocket};
    
fn main() -> std::io::Result<()> {
    println!("udp client starting ... ");
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("useage: updClient 10.97.24.182 19000");
        return Ok(());
    }
    println!("{:?}", args);
    let rport = args[2].as_str();
    let port: i32 = rport.parse::<i32>().unwrap();
    let mut rudpip = args[1].clone();
    rudpip = rudpip + ":";
    let mut bondrip = String::new()+"127.0.0.1:";
    let bport: i32 = port + 1;
    bondrip = bondrip + bport.to_string().as_str();
    rudpip = rudpip + rport;
    println!("bond ip {} , connect ip {}", bondrip, rudpip);
    // let socket = UdpSocket::bind(bondrip)?;
    let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 12345)).unwrap();
    socket.connect(rudpip)?;

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
 
}
