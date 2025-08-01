use core::arch::asm;

use crate::eth_defs::*;

/*
// for C test
unsafe extern "C" {
    pub fn plat_printf(fmt: *const u8, _: ...) -> i32;
    pub fn sync_dcache();
    pub fn plat_virt_to_phys(va: u64) -> u32;
    pub fn plat_phys_to_virt(pa: u32) -> u64;
    pub fn plat_malloc_align(size: u64, align: u32) -> u64;
    pub fn plat_handle_tx_buffer(p: u64, buffer: u64) -> u32;
    pub fn plat_handle_rx_buffer(buffer: u64, length: u32) -> u64;
    pub fn plat_rx_ready(gmacdev: *mut net_device);
    pub fn platform_update_linkstate(gmacdev: *mut net_device, status: u32);
    pub fn plat_isr_install();
}
*/


// 这里是测试时用于调用C的printf
// 替换成OS实现的printf
pub unsafe extern "C" fn plat_printf(_fmt: *const u8, _: ...) -> i32 {
    0
}

// 同步dcache中所有cached和uncached访存请求
pub fn sync_dcache() {
    unsafe {
        asm!("dbar 0");
    }
}

// cached虚拟地址转换为物理地址
// dma仅接受32位的物理地址
pub fn plat_virt_to_phys(va: u64) -> u32 {
    va as u32
}

// 物理地址转换为cached虚拟地址
pub fn plat_phys_to_virt(pa: u32) -> u64 {
    pa as u64
}

// 物理地址转换为uncached虚拟地址
pub fn plat_phys_to_uncached(pa: u64) -> u64 {
    pa
}

// 分配按align字节对齐的内存
pub fn plat_malloc_align(size: u64, align: u32) -> u64 {
    0
}

// 处理tx buffer
pub fn plat_handle_tx_buffer(p: u64, buffer: u64) -> u32 {
    //（OS可能会有自定义格式的存储单元）
    // p是OS传递给驱动的存储单元
    // buffer是驱动分配的dma内存
    // 将p的数据copy到buffer中
    // 返回数据总长度
    0
}

// 处理rx buffer
pub fn plat_handle_rx_buffer(buffer: u64, length: u32) -> u64 {
    // buffer是接收到的数据，length是字节数
    // OS需要分配内存，memcpy接收到的数据，并将地址返回
    0
}

// 中断isr通知OS可以调用rx函数
pub fn plat_rx_ready(gmacdev: *mut net_device) {}

// 中断isr通知链路状态发生变化，status - 1表示up，0表示down
// 链路目前仅支持1000Mbps duplex
pub fn platform_update_linkstate(gmacdev: *mut net_device, status: u32) {}

// OS注册中断，中断号为12
pub fn plat_isr_install() {}
