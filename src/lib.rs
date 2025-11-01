#![no_std]

use core::alloc::{GlobalAlloc, Layout};

const SYS_MMAP: u64 = 9;
const SYS_MUNMAP: u64 = 10;

#[inline(always)]
pub fn syscall3(num: u64, a0: u64, a1: u64, a2: u64) -> u64 {
    let ret: u64;
    unsafe {
        core::arch::asm!(
            "int 0x80",
            in("rax") num,
            in("rdi") a0,
            in("rsi") a1,
            in("rdx") a2,
            lateout("rax") ret,
            options(nostack)
        );
    }
    ret
}

pub struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = (layout.size() + 4095) & !4095;
        let ptr = syscall3(SYS_MMAP, 0, size as u64, 0);

        if ptr == u64::MAX {
            core::ptr::null_mut()
        } else {
            ptr as *mut u8
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = (layout.size() + 4095) & !4095;
        syscall3(SYS_MUNMAP, ptr as u64, size as u64, 0);
    }
}

#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;
