#![allow(non_camel_case_types, non_snake_case, unused)]

type UCHAR = u8;
type USHORT = u16;
type ULONG = u32;
type NTSTATUS = u32;
type BOOL = bool;
type PVOID = *mut c_void;
type PULONG = *mut ULONG;
type PUCHAR = *mut u8;
type PCHAR = *mut c_char;
type LONG = i32;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct _USB_SETUP_PKT {
    pub mUspReqType: UCHAR,
    pub mUspRequest: UCHAR,
    pub __bindgen_anon_1: _USB_SETUP_PKT__bindgen_ty_1,
    pub __bindgen_anon_2: _USB_SETUP_PKT__bindgen_ty_2,
    pub mLength: USHORT,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _USB_SETUP_PKT__bindgen_ty_1 {
    pub __bindgen_anon_1: _USB_SETUP_PKT__bindgen_ty_1__bindgen_ty_1,
    pub mUspValue: USHORT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _USB_SETUP_PKT__bindgen_ty_1__bindgen_ty_1 {
    pub mUspValueLow: UCHAR,
    pub mUspValueHigh: UCHAR,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _USB_SETUP_PKT__bindgen_ty_2 {
    pub __bindgen_anon_1: _USB_SETUP_PKT__bindgen_ty_2__bindgen_ty_1,
    pub mUspIndex: USHORT,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _USB_SETUP_PKT__bindgen_ty_2__bindgen_ty_1 {
    pub mUspIndexLow: UCHAR,
    pub mUspIndexHigh: UCHAR,
}
pub type mUSB_SETUP_PKT = _USB_SETUP_PKT;
pub type mPUSB_SETUP_PKT = *mut _USB_SETUP_PKT;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct _WIN32_COMMAND {
    pub __bindgen_anon_1: _WIN32_COMMAND__bindgen_ty_1,
    pub mLength: ULONG,
    pub __bindgen_anon_2: _WIN32_COMMAND__bindgen_ty_2,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _WIN32_COMMAND__bindgen_ty_1 {
    pub mFunction: ULONG,
    pub mStatus: NTSTATUS,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union _WIN32_COMMAND__bindgen_ty_2 {
    pub mSetupPkt: mUSB_SETUP_PKT,
    pub mBuffer: [UCHAR; 512usize],
}
pub type mWIN32_COMMAND = _WIN32_COMMAND;
pub type mPWIN32_COMMAND = *mut _WIN32_COMMAND;
pub type mPCH341_INT_ROUTINE = ::std::option::Option<unsafe extern "C" fn(iStatus: ULONG)>;
extern "C" {
    pub fn CH341OpenDevice(iIndex: ULONG) -> HANDLE;
}
extern "C" {
    pub fn CH341CloseDevice(iIndex: ULONG);
}
extern "C" {
    pub fn CH341GetVersion() -> ULONG;
}
extern "C" {
    pub fn CH341DriverCommand(iIndex: ULONG, ioCommand: mPWIN32_COMMAND) -> ULONG;
}
extern "C" {
    pub fn CH341GetDrvVersion() -> ULONG;
}
extern "C" {
    pub fn CH341ResetDevice(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341GetDeviceDescr(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341GetConfigDescr(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetIntRoutine(iIndex: ULONG, iIntRoutine: mPCH341_INT_ROUTINE) -> BOOL;
}
extern "C" {
    pub fn CH341ReadInter(iIndex: ULONG, iStatus: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341AbortInter(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetParaMode(iIndex: ULONG, iMode: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341InitParallel(iIndex: ULONG, iMode: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ReadData0(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ReadData1(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341AbortRead(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341WriteData0(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341WriteData1(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341AbortWrite(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341GetStatus(iIndex: ULONG, iStatus: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ReadI2C(iIndex: ULONG, iDevice: UCHAR, iAddr: UCHAR, oByte: PUCHAR) -> BOOL;
}
extern "C" {
    pub fn CH341WriteI2C(iIndex: ULONG, iDevice: UCHAR, iAddr: UCHAR, iByte: UCHAR) -> BOOL;
}
extern "C" {
    pub fn CH341EppReadData(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341EppReadAddr(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341EppWriteData(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341EppWriteAddr(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341EppSetAddr(iIndex: ULONG, iAddr: UCHAR) -> BOOL;
}
extern "C" {
    pub fn CH341MemReadAddr0(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341MemReadAddr1(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341MemWriteAddr0(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341MemWriteAddr1(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetExclusive(iIndex: ULONG, iExclusive: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetTimeout(iIndex: ULONG, iWriteTimeout: ULONG, iReadTimeout: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ReadData(iIndex: ULONG, oBuffer: PVOID, ioLength: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341WriteData(iIndex: ULONG, iBuffer: PVOID, ioLength: PULONG) -> BOOL;
}

extern "C" {
    pub fn CH341GetDeviceName(iIndex: ULONG) -> PVOID;
}
extern "C" {
    pub fn CH341GetVerIC(iIndex: ULONG) -> ULONG;
}
extern "C" {
    pub fn CH341FlushBuffer(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341WriteRead(
        iIndex: ULONG,
        iWriteLength: ULONG,
        iWriteBuffer: PVOID,
        iReadStep: ULONG,
        iReadTimes: ULONG,
        oReadLength: PULONG,
        oReadBuffer: PVOID,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341SetStream(iIndex: ULONG, iMode: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetDelaymS(iIndex: ULONG, iDelay: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341StreamI2C(
        iIndex: ULONG,
        iWriteLength: ULONG,
        iWriteBuffer: PVOID,
        iReadLength: ULONG,
        oReadBuffer: PVOID,
    ) -> BOOL;
}
pub const _EEPROM_TYPE_ID_24C01: _EEPROM_TYPE = 0;
pub const _EEPROM_TYPE_ID_24C02: _EEPROM_TYPE = 1;
pub const _EEPROM_TYPE_ID_24C04: _EEPROM_TYPE = 2;
pub const _EEPROM_TYPE_ID_24C08: _EEPROM_TYPE = 3;
pub const _EEPROM_TYPE_ID_24C16: _EEPROM_TYPE = 4;
pub const _EEPROM_TYPE_ID_24C32: _EEPROM_TYPE = 5;
pub const _EEPROM_TYPE_ID_24C64: _EEPROM_TYPE = 6;
pub const _EEPROM_TYPE_ID_24C128: _EEPROM_TYPE = 7;
pub const _EEPROM_TYPE_ID_24C256: _EEPROM_TYPE = 8;
pub const _EEPROM_TYPE_ID_24C512: _EEPROM_TYPE = 9;
pub const _EEPROM_TYPE_ID_24C1024: _EEPROM_TYPE = 10;
pub const _EEPROM_TYPE_ID_24C2048: _EEPROM_TYPE = 11;
pub const _EEPROM_TYPE_ID_24C4096: _EEPROM_TYPE = 12;
pub type _EEPROM_TYPE = ::std::os::raw::c_int;
use std::{ffi::{c_char, c_void}, os::windows::raw::HANDLE};

pub use self::_EEPROM_TYPE as EEPROM_TYPE;
extern "C" {
    pub fn CH341ReadEEPROM(
        iIndex: ULONG,
        iEepromID: EEPROM_TYPE,
        iAddr: ULONG,
        iLength: ULONG,
        oBuffer: PUCHAR,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341WriteEEPROM(
        iIndex: ULONG,
        iEepromID: EEPROM_TYPE,
        iAddr: ULONG,
        iLength: ULONG,
        iBuffer: PUCHAR,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341GetInput(iIndex: ULONG, iStatus: PULONG) -> BOOL;
}
extern "C" {
    pub fn CH341SetOutput(
        iIndex: ULONG,
        iEnable: ULONG,
        iSetDirOut: ULONG,
        iSetDataOut: ULONG,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341Set_D5_D0(iIndex: ULONG, iSetDirOut: ULONG, iSetDataOut: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341StreamSPI3(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        ioBuffer: PVOID,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341StreamSPI4(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        ioBuffer: PVOID,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341StreamSPI5(
        iIndex: ULONG,
        iChipSelect: ULONG,
        iLength: ULONG,
        ioBuffer: PVOID,
        ioBuffer2: PVOID,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341BitStreamSPI(iIndex: ULONG, iLength: ULONG, ioBuffer: PVOID) -> BOOL;
}
extern "C" {
    pub fn CH341SetBufUpload(iIndex: ULONG, iEnableOrClear: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341QueryBufUpload(iIndex: ULONG) -> LONG;
}
extern "C" {
    pub fn CH341SetBufDownload(iIndex: ULONG, iEnableOrClear: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341QueryBufDownload(iIndex: ULONG) -> LONG;
}
extern "C" {
    pub fn CH341ResetInter(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ResetRead(iIndex: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341ResetWrite(iIndex: ULONG) -> BOOL;
}
pub type mPCH341_NOTIFY_ROUTINE = ::std::option::Option<unsafe extern "C" fn(iEventStatus: ULONG)>;
extern "C" {
    pub fn CH341SetDeviceNotify(
        iIndex: ULONG,
        iDeviceID: PCHAR,
        iNotifyRoutine: mPCH341_NOTIFY_ROUTINE,
    ) -> BOOL;
}
extern "C" {
    pub fn CH341SetupSerial(iIndex: ULONG, iParityMode: ULONG, iBaudRate: ULONG) -> BOOL;
}
extern "C" {
    pub fn CH341OpenDeviceEx(iIndex: ULONG) -> HANDLE;
}
extern "C" {
    pub fn CH341CloseDeviceEx(iIndex: ULONG);
}
extern "C" {
    pub fn CH341GetDeviceNameEx(iIndex: ULONG) -> PCHAR;
}
extern "C" {
    pub fn CH341SetDeviceNotifyEx(
        iIndex: ULONG,
        iDeviceID: PCHAR,
        iNotifyRoutine: mPCH341_NOTIFY_ROUTINE,
    ) -> BOOL;
}