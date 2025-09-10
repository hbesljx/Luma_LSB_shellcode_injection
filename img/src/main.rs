// src/main.rs

use std::env;
use std::process;

mod img;

fn print_usage() {
    eprintln!(
        "用法:\n\
         img.exe hide <图片.jpg> <载荷.bin>\n\
         img.exe read <隐写图.png>"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "hide" => {
            if args.len() != 4 {
                print_usage();
                process::exit(1);
            }
            let img_path = &args[2];
            let bin_path = &args[3];

            // 调用你的 hide 函数
            img::hide(img_path, bin_path);
            // 如果你想静默，注释掉 `println!` 在 hide 内部
        }
        "read" => {
            if args.len() != 3 {
                print_usage();
                process::exit(1);
            }
            let output_path = &args[2];

            img::read(output_path); // 它会打印 msg
        }
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}