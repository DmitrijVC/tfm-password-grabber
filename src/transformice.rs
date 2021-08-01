use super::*;

pub const OFFSETS: [usize; 5] = [0x5C, 0x7BC, 0x144, 0x8, 0x20];


pub unsafe fn get_adobe_air_address() -> *mut HINSTANCE__ {
    let module_name: Vec<u16> = MODULE_NAME.encode_utf16().collect();
    let module_path: Vec<u16> = MODULE_PATH.encode_utf16().collect();

    let mut failures = 0_u16;
    let mut module_handle = std::ptr::null_mut() as *mut HINSTANCE__;

    while module_handle.is_null() {
        if failures < 5 {
            println!("[Adobe AIR.dll] Loading from GetModuleHandle...");
            module_handle = GetModuleHandleW(module_name.as_ptr());

        } else if failures < 8 {
            println!("[Adobe AIR.dll] Loading from LoadLibrary...");
            module_handle = LoadLibraryW(module_path.as_ptr());

        } else {
            println!("[Adobe AIR.dll] Loading failed, restarting the thread!\n");

            return NULL_MUT as *mut _;
        }

        failures += 1;
        thread::sleep(Duration::from_millis(500));
    }

    println!("[Adobe AIR.dll] Loaded at: {:?}", module_handle);

    module_handle
}


pub unsafe fn read_password(ptr: *mut u8, size: usize) -> String {
    let raw_bytes= Vec::from_raw_parts(
        ptr,
        size,
        size,
    );

    let mut password = String::with_capacity(size);
    for byte in raw_bytes {
        if byte < 32 || byte > 126 {
            break;
        }

        password.push(byte as char);
    }

    password
}
