use std::{ffi::{c_char, CStr}, time::Duration};

use ch341::{CH341CloseDevice, CH341OpenDevice, CH341SetOutput};
use ch341_sys as ch341;

fn main() {
    unsafe{
        //https://github.com/tomek-o/CH341A-tool
        blink();
    }
    
}

unsafe fn blink(){
    let dev = CH341OpenDevice(0);
    
    if dev as u64 == u64::MAX{
        println!("设备打开失败!");
        return;
    }
    println!("设备打开成功");

    // 位7-位0对应CH341的D7-D0引脚

    // 位2为1说明iSetDataOut的7-位0有效,否则忽略
    // 位3为1说明iSetDirOut的位7-位0有效,否则忽略
    let enable = 1<<2;
    
    //iSetDirOut  设置I/O方向,某位清0则对应引脚为输入,某位置1则对应引脚为输出,并口方式下默认值为0x000FC000,参考下面的位说明
	//iSetDataOut  输出数据,如果I/O方向为输出,那么某位清0时对应引脚输出低电平,某位置1时对应引脚输出高电平,参考下面的位说明

    for _ in 0..20{
        let set_dir_out = 1; // 第0位为1，代表D0引脚为输出
        let set_data_out = 1; // 第0位为1，代表D0引脚输出高电平
        let success = CH341SetOutput(0, enable, set_dir_out, set_data_out);
        println!("GPIO写入{success}");
        std::thread::sleep(Duration::from_millis(1000));

        let set_dir_out = 1; // 第0位为1，代表D0引脚为输出
        let set_data_out = 0; // 第0位为1，代表D0引脚输出低电平
        let success = CH341SetOutput(0, enable, set_dir_out, set_data_out);
        println!("GPIO写入{success}");
        std::thread::sleep(Duration::from_millis(1000));
    }

    CH341CloseDevice(0);
    println!("设备已关闭");
}
