// use std::{io, str};
use std::vec::{Vec};
    
// &str    -> String--| String::from(s) or s.to_string() or s.to_owned()
// &str    -> &[u8]---| s.as_bytes()
// &str    -> Vec<u8>-| s.as_bytes().to_vec() or s.as_bytes().to_owned()
// String  -> &str----| &s if possible* else s.as_str()
// String  -> &[u8]---| s.as_bytes()
// String  -> Vec<u8>-| s.into_bytes()
// &[u8]   -> &str----| s.to_vec() or s.to_owned()
// &[u8]   -> String--| std::str::from_utf8(s).unwrap(), but don't**
// &[u8]   -> Vec<u8>-| String::from_utf8(s).unwrap(), but don't**
// Vec<u8> -> &str----| &s if possible* else s.as_slice()
// Vec<u8> -> String--| std::str::from_utf8(&s).unwrap(), but don't**
// Vec<u8> -> &[u8]---| String::from_utf8(s).unwrap(), but don't**


fn main()   {
    println!("udp client starting ... ");
    let mut buffer:Vec<u8> = vec![0u8; 120 as usize];
    buffer.fill('a' as u8);
    // "sssssss".as_bytes()
    // String::from_utf8(buffer).unwrap()
    // /String::from("Hello boys")
    let buf = "Hello boys ! come come";
    let bytes:&[u8] = buf.as_bytes();
    println!("{:?}",bytes);
    let by = String::from_utf8(buffer).unwrap();
    let buf:&[u8] = by.as_bytes();
    println!("{:?}", buf);
}
