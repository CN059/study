// 移除对标准库的依赖
#![no_std]
#![no_main]
use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello, no_std!\n";
    unsafe{
        core::arch::asm!(
            "syscall",
            in("rax") 1usize, // syscall number
            in("rdi") 1usize, // fd 0 stdin 1 stdout 2 stderr 3
            in("rsi") msg.as_ptr(),
            in("rdx") msg.len(),
        )   
    }
    unsafe{
        core::arch::asm!(
            "syscall",
            in("rax") 60usize, // syscall number
            in("rdi") 0usize, // exit code
        )
    }
    loop{}
}