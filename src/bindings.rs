pub use std::ffi::c_void;
pub use std::time::Duration;
pub use std::{
    ptr,
    thread
};

pub use winapi::um::winbase::IsBadReadPtr;
pub use winapi::um::processthreadsapi::CreateThread;
pub use winapi::um::consoleapi::AllocConsole;
pub use winapi::um::errhandlingapi::GetLastError;
pub use winapi::um::handleapi::CloseHandle;
pub use winapi::vc::vadefs::uintptr_t;
pub use winapi::um::libloaderapi::{
    DisableThreadLibraryCalls,
    FreeLibraryAndExitThread,
    GetModuleHandleW,
    LoadLibraryW
};
pub use winapi::shared::minwindef::{
    HINSTANCE,
    DWORD,
    LPVOID,
    BOOL,
    TRUE,
    HINSTANCE__
};
pub use winapi::um::winnt::{
    DLL_PROCESS_ATTACH,
    DLL_PROCESS_DETACH,
};
