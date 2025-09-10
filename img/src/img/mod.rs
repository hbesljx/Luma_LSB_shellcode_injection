use image::{GenericImageView, ImageBuffer, Luma, Rgb, RgbImage};
use std::path::{self, Path};
use std::fs;

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

// 整数版 YCbCr→RGB（无精度损失）
fn ycbcr_to_rgb(y: u8, cb: u8, cr: u8) -> (u8, u8, u8) {
    let y = y as i32 - 16;
    let cb = cb as i32 - 128;
    let cr = cr as i32 - 128;

    // 公式来源：Rec.601 标准，整数乘法避免浮点数误差
    let r = (298 * y + 409 * cr + 128) >> 8;
    let g = (298 * y - 100 * cb - 208 * cr + 128) >> 8;
    let b = (298 * y + 516 * cb + 128) >> 8;

    // 确保结果在 0-255 范围内
    (
        r.clamp(0, 255) as u8,
        g.clamp(0, 255) as u8,
        b.clamp(0, 255) as u8,
    )
}
fn lsb_one_bit(buff:u8,msg:u8)->u8{ //先隐写1bit到1byte中
    let bit = msg & 1;  //保证是0或1
    (buff & !1) | bit   //嵌入最低位
}
fn lsb(y_buff:&mut ImageBuffer<Luma<u8>,Vec<u8>>,msg:&[u8]){
    let mut bits=Vec::new();
    for byte in msg.iter(){
        for i in 0..8{
            bits.push((byte>>i)&1);
        }
    }
    let mut i =0;
    let len=bits.len();
    for y in 0..y_buff.height(){
        for x in 0..y_buff.width(){
            if i>=len{
                // println!("隐写完成!");
                return;
            }
            let old=y_buff.get_pixel_mut(x, y);
            old[0]=lsb_one_bit(old[0], bits[i]);
            i+=1;
        }
    }
    // println!("图像空间不足!");
}
pub fn hide(img_path:&str,bin_path:&str){
    let img=image::open(img_path).unwrap();
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
    let msg=std::fs::read(bin_path).expect("读取bin文件失败!");

    let mut data_with_len=(msg.len() as u32).to_le_bytes().to_vec();
    data_with_len.extend_from_slice(&msg);  //在前面加4字节固定头部，记录数据长度

    lsb(&mut y_buff, &data_with_len);

    let mut output_img=RgbImage::new(width, height);
    for y in 0..height{
        for x in 0..width{
            let yval=y_buff.get_pixel(x, y)[0];
            let cbval=cb_buff.get_pixel(x, y)[0];
            let crval=cr_buff.get_pixel(x, y)[0];
            let (r,g,b)=ycbcr_to_rgb(yval, cbval, crval);
            output_img.put_pixel(x, y, Rgb([r,g,b]));
        }
    }
    output_img.save("output.png");
    println!("已保存到output.png!");
}
pub fn read(output_path:&str){
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
    print!("msg:{:?}",msg_byte);
}