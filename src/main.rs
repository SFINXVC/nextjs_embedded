// include_bytes!()

use rust_embed::Embed;
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use windows::Win32::Foundation::BOOL;
use windows::Win32::System::Console::{
    CTRL_CLOSE_EVENT, CTRL_LOGOFF_EVENT, CTRL_SHUTDOWN_EVENT, SetConsoleCtrlHandler,
};

// check if the current os is not windows, since the entire app only supports windows (x86_64)
#[cfg(not(target_os = "windows"))]
compile_error!("Unfortunately my brother, but this app only supports windows machine :3");

const NODE_BIN: &[u8] = include_bytes!("C:\\Program Files\\nodejs\\node.exe");

#[derive(Embed)]
#[folder = "my-app/.next/standalone_resolved"]
struct NextStandaloneApp;

static TEMP_DIR_PATH: std::sync::OnceLock<PathBuf> = std::sync::OnceLock::new();
static CHILD_PID: AtomicU32 = AtomicU32::new(0);

fn cleanup() {
    let pid = CHILD_PID.load(Ordering::SeqCst);
    if pid != 0 {
        #[cfg(target_os = "windows")]
        unsafe {
            use windows::Win32::System::Threading::{OpenProcess, TerminateProcess, PROCESS_TERMINATE};
            use windows::Win32::Foundation::CloseHandle;
            
            if let Ok(handle) = OpenProcess(PROCESS_TERMINATE, false, pid) {
                let _ = TerminateProcess(handle, 1);
                let _ = CloseHandle(handle);
            }
        }
    }
    
    if let Some(temp_dir) = TEMP_DIR_PATH.get() {
        if temp_dir.exists() {
            let _ = std::fs::remove_dir_all(temp_dir);
            println!("Cleaned up temp directory: {:?}", temp_dir);
        }
    }
}

unsafe extern "system" fn console_ctrl_handler(ctrl_type: u32) -> BOOL {
    match ctrl_type {
        CTRL_CLOSE_EVENT | CTRL_LOGOFF_EVENT | CTRL_SHUTDOWN_EVENT => {
            cleanup();
            std::thread::sleep(std::time::Duration::from_millis(500));
            BOOL(1)
        }
        _ => BOOL(0),
    }
}

fn main() -> std::io::Result<()> {
    let temp_dir = std::env::temp_dir().join("mulyono_raja_tipu_tipu");
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

    TEMP_DIR_PATH
        .set(temp_dir.clone())
        .expect("Failed to set temp dir path");

    std::panic::set_hook(Box::new(|panic_info| {
        eprintln!("Program panicked: {:?}", panic_info);
        cleanup();
    }));

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        println!("\nReceived Ctrl+C signal, cleaning up...");
        cleanup();
        r.store(false, Ordering::SeqCst);
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    unsafe {
        SetConsoleCtrlHandler(Some(console_ctrl_handler), BOOL(1))
            .expect("Failed to set console ctrl handler");
    }

    let node_path = temp_dir.join("node.exe");
    std::fs::write(&node_path, NODE_BIN).expect("Failed to extract node runtime");

    for file in NextStandaloneApp::iter() {
        let out_path = temp_dir.join(&*file);
        if let Some(parent) = out_path.parent() {
            std::fs::create_dir_all(parent).expect("Failed to create parent dir");
        }

        let content = NextStandaloneApp::get(&file).unwrap();
        std::fs::write(&out_path, content.data).expect("Failed to extract file");
    }

    let entry = temp_dir.join("server.js");

    let mut child = std::process::Command::new(&node_path)
        .arg(&entry)
        .current_dir(&temp_dir)
        .spawn()?;

    let pid = child.id();
    CHILD_PID.store(pid, Ordering::SeqCst);
    println!("Node server is now started with PID {}", pid);

    let status = child.wait()?;
    println!("Node server exited with status: {}", status);

    cleanup();

    Ok(())
}
