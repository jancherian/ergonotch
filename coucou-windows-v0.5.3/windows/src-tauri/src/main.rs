// Notch runs without a console window: the notch pill is the whole UI.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(windows)]
fn is_another_instance_running() -> bool {
    use windows::core::HSTRING;
    use windows::Win32::Foundation::{GetLastError, SetLastError, CloseHandle, ERROR_ALREADY_EXISTS, WIN32_ERROR};
    use windows::Win32::System::Threading::CreateMutexW;

    unsafe {
        SetLastError(WIN32_ERROR(0));
        let name = HSTRING::from("Local\\NotchSingleInstanceMutex_v1");
        let handle = match CreateMutexW(None, true, &name) {
            Ok(h) => h,
            Err(_) => return false,
        };
        if GetLastError() == ERROR_ALREADY_EXISTS {
            let _ = CloseHandle(handle);
            return true;
        }
        let _ = handle;
        false
    }
}

fn main() {
    #[cfg(windows)]
    if is_another_instance_running() {
        // Another instance is already running; exit cleanly to prevent WebView2 User Data Folder lock collision
        return;
    }

    notch_lib::run()
}
