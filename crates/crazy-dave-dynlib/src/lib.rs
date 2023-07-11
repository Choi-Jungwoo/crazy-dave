use std::thread;

use crazy_dave_core;
use winapi::shared::minwindef::*;

const DLL_PROCESS_ATTACH: DWORD = 1;
const DLL_PROCESS_DETACH: DWORD = 0;

#[no_mangle]
extern "system" fn DllMain(_dll_module: HINSTANCE, call_reason: DWORD, _reserved: LPVOID) -> BOOL {
    match call_reason {
        DLL_PROCESS_ATTACH => {
            thread::spawn(|| {
                crazy_dave_core::start_window();
            });
        }
        DLL_PROCESS_DETACH => (),
        _ => (),
    }

    TRUE
}
