use std::time::Duration;

use ch341::{CH341CloseDevice, CH341OpenDevice, CH341SetOutput};
use ch341_sys as ch341;

fn main() {
    unsafe{
        //https://github.com/tomek-o/CH341A-tool
        blink();
        st7789();
    }
    
}

#[allow(unused)]
unsafe fn blink(){
    let dev = CH341OpenDevice(0);
    
    if dev as u64 == u64::MAX{
        println!("设备打开失败!");
        return;
    }
    println!("设备打开成功");

    // 位2为1说明iSetDataOut的7-位0有效,否则忽略
    // 位3为1说明iSetDirOut的位7-位0有效,否则忽略
    let enable = 0b1100;
    
    //iSetDirOut 低8位对应CH341的D7-D0引脚。
	//iSetDataOut 低8位对应CH341的D7-D0引脚的电平输出。
    for _ in 0..5{
        let set_dir_out = 0b111;
        let set_data_out = 0b101;
        let _ = CH341SetOutput(0, enable, set_dir_out, set_data_out);
        std::thread::sleep(Duration::from_millis(300));

        let set_dir_out = 0b111;
        let set_data_out = 0b010;
        let _ = CH341SetOutput(0, enable, set_dir_out, set_data_out);
        std::thread::sleep(Duration::from_millis(300));
    }

    CH341CloseDevice(0);
    println!("设备已关闭");
}

/*
ST7789跟CH341A SPI连接方式
VCC <=> VCC
GND <=> GND
SCL <=> SCK (时钟线)
SDA <=> MISO (数据线)
RES <=> 复位
DC  <=> ？数据/命令选择
BLK <=> VCC
 */
unsafe fn st7789(){

}