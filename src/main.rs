/*use std::fs::File;
use std::io::{BufReader, Read};
//use std::io::Write;
use std::path::Path;

fn main() {
    //Path to new/current file.
    let path = Path::new("C:/Users/Chi/Desktop/loremipsum.txt");
    open_file(&path);

    let utf_8 = [239, 187, 191, 49, 13, 10, 13, 10, 50];
    let ansi = [49, 13, 10, 13, 10, 50];
}

/*//Write-only: Writes user input into the file.
fn save_file(path: &Path, input: &String) {
    let mut f = File::create(&path).expect("Unable to create file");
    f.write_all(&input.as_bytes())
        .expect("Unable to write data");
}*/

//Read-only: Prints the content of the file to the terminal.
fn open_file(path: &Path) {
    let f = File::open(&path).expect("Unable to open file");
    let mut br = BufReader::new(f);
    let mut text = String::new();
    br.read_to_string(&mut text).expect("Unable to read string");
    println!("{:?}", text.as_bytes());
}*/

#![windows_subsystem = "windows"]
extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ffi::OsStr;
use std::iter::once;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use kernel32::GetModuleHandleW;
use user32::{CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, MessageBoxW,
             PostQuitMessage, RegisterClassW, TranslateMessage};
use winapi::minwindef::{LPARAM, LRESULT, WPARAM};
use winapi::windef::{HWND, POINT};
use winapi::winuser::{CS_HREDRAW, CS_OWNDC, CS_VREDRAW, CW_USEDEFAULT, MB_OK, MSG, WM_DESTROY,
                      WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};

//This can be used for a Trait
fn to_wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(once(0)).collect()
}

fn create_window(name: &str, title: &str) {
    let class = to_wide(name);
    let title = to_wide(title);

    unsafe {
        let wnd_class = WNDCLASSW {
            style: CS_OWNDC | CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(wnd_proc),
            hInstance: GetModuleHandleW(null_mut()), //HINSTANCE of the .exe
            lpszClassName: class.as_ptr(),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hIcon: null_mut(),
            hCursor: null_mut(),
            hbrBackground: null_mut(),
            lpszMenuName: null_mut(),
        };

        if RegisterClassW(&wnd_class) == 0 {
            MessageBoxW(
                null_mut(),
                to_wide("Failed to register class.").as_ptr() as *const u16,
                to_wide("ERROR").as_ptr() as *const u16,
                MB_OK,
            );
        } else {
            let handle = CreateWindowExW(
                0, //WS_EX_COMPOSITED if some things do not appear
                class.as_ptr(),
                title.as_ptr(),
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                null_mut(),
                null_mut(),
                GetModuleHandleW(null_mut()),
                null_mut(),
            );

            if handle.is_null() {
                MessageBoxW(
                    null_mut(),
                    to_wide("Failed to create window.").as_ptr() as *const u16,
                    to_wide("ERROR").as_ptr() as *const u16,
                    MB_OK,
                );
            }
        }
    }
}

unsafe extern "system" fn wnd_proc(
    hwnd: HWND,     //Handle to main/parent window
    message: u32,   //System-defined messages (e.g. WM_SIZE)
    wparam: WPARAM, //More message specified information (e.g. User clicked menu item X)
    lparam: LPARAM, //More message specified information
) -> LRESULT {
    match message {
        WM_DESTROY => {
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(hwnd, message, wparam, lparam),
    }
}

fn main() {
    create_window("my_window", "Hello Windows!");

    let mut msg = MSG {
        hwnd: null_mut(),
        message: 0,
        wParam: 0,
        lParam: 0,
        time: 0,
        pt: POINT { x: 0, y: 0 },
    };
    //unsafe alternative -> let mut msg: MSG = mem::uninitialized();

    unsafe {
        while GetMessageW(&mut msg as *mut MSG, 0 as HWND, 0, 0) != 0 {
            TranslateMessage(&msg as *const MSG);
            DispatchMessageW(&msg as *const MSG);
        }
    }
}
