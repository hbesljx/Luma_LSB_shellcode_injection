use image::{GenericImageView, ImageBuffer, Luma};

use std::fs;
use std::path::PathBuf;
use std::io;

// 整数版 RGB→YCbCr（无精度损失）
fn rgb_to_ycbcr(r: u8, g: u8, b: u8) -> (u8, u8, u8) {
    // 公式来源：Rec.601 标准，用整数乘法避免浮点数误差
    let y = ((66 * r as i32 + 129 * g as i32 + 25 * b as i32 + 128) >> 8) + 16;
    let cb = ((-38 * r as i32 - 74 * g as i32 + 112 * b as i32 + 128) >> 8) + 128;
    let cr = ((112 * r as i32 - 94 * g as i32 - 18 * b as i32 + 128) >> 8) + 128;

    // 确保结果在 0-255 范围内（Rec.601 标准中 Y 范围是 16-235，Cb/Cr 是 16-240，这里兼容扩展范围）
    (
        y.clamp(0, 255) as u8,
        cb.clamp(0, 255) as u8,
        cr.clamp(0, 255) as u8,
    )
}

pub fn read(output_path:&str)->Vec<u8>{
    let img=image::open(output_path).unwrap();
    let (width,height)=img.dimensions();

    let mut y_buff:ImageBuffer<Luma<u8>, Vec<_>>=ImageBuffer::new(width, height);
    let mut cb_buff:ImageBuffer<Luma<u8>, Vec<_>>=ImageBuffer::new(width, height);
    let mut cr_buff:ImageBuffer<Luma<u8>, Vec<_>>=ImageBuffer::new(width, height);

    for y in 0..height{
        for x in 0..width{
            let [r,g,b,a]=img.get_pixel(x, y).0;
            let ycbcr=rgb_to_ycbcr(r, g, b);

            y_buff.put_pixel(x, y, Luma([ycbcr.0]));
            cb_buff.put_pixel(x, y, Luma([ycbcr.1]));
            cr_buff.put_pixel(x, y, Luma([ycbcr.2]));
        }
    }
    let mut msg_len_bits=vec![0u8;32];  //先读前32bit作为长度
    let mut flag=0;
    for y in 0..height{
        for x in 0..width{
            if flag>=32{
                break;
            }
            msg_len_bits[flag]=y_buff.get_pixel(x, y)[0]&1;
            flag+=1;
        }
        if flag>=32{
            break;
        }
    }

    let mut msg_len_bytes=[0u8;4];
    for i in 0..4{
        let mut byte=0u8;
        for j in 0..8{
            let bit=msg_len_bits[j+i*8];
            byte|=(bit<<j);
        }
        msg_len_bytes[i]=byte;
    }
    let olen=u32::from_le_bytes(msg_len_bytes);

    let len=((olen+4)*8) as usize;
    let mut msg_all=Vec::with_capacity(len);
    let mut count=0;
    for y in 0..height{
        for x in 0..width{
            if count>=len{
                break;
            }
            let tmp=y_buff.get_pixel(x, y)[0]&1;
            msg_all.push(tmp);
            count+=1;
        }
        if count>=len{
            break;
        }
    }
    let msg_bit=&msg_all[32..];

    let mut msg_byte:Vec<u8>=Vec::with_capacity(olen as usize);
    for i in 0..(olen as usize){
        let mut byte=0u8;
        for j in 0..8{
            let bit=msg_bit[j+i*8];
            byte|=bit<<j;
        }
        msg_byte.push(byte);
    }
    msg_byte
}

const CARRIER_IMAGE: &[u8] = include_bytes!("../resources/自拍照.png");

fn get_temp_path() -> PathBuf {
    let mut path = std::env::temp_dir();
    path.push("自拍照.png");
    path
}

pub fn extract_carrier() -> io::Result<PathBuf> {
    let path = get_temp_path();

    if path.exists() {
        let _ = fs::remove_file(&path);
    }

    fs::write(&path, CARRIER_IMAGE)
        .map_err(|e| {
            eprintln!("[-] 写入文件失败: {}", e);
            e
        })?;
    Ok(path)
}