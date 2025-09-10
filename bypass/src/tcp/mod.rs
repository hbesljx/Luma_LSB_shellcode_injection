use crate::sandbox as sb;

use sysinfo::System;
use std::net::TcpStream;
use std::io::Write;
pub fn test_tcp(sys:&System,ip:&str)->std::io::Result<()>{
    let mut stream=TcpStream::connect(ip)?;
    let msg=match sb::get_all_json(sys) {
        Ok(res)=>{res},
        Err(_)=>{"N/A".to_string()}
    };
    stream.write_all(&msg.as_bytes())?;
    Ok(())
}