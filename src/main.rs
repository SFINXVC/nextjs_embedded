// include_bytes!()

use rust_embed::Embed;

// check if the current os is not windows, since the entire app only supports windows (x86_64)
#[cfg(not(target_os = "windows"))]
compile_error!("Unfortunately my brother, but this app only supports windows machine :3");

const NODE_BIN: &[u8] = include_bytes!("C:\\Program Files\\nodejs\\node.exe");

#[derive(Embed)]
#[folder = "my-app/.next/standalone"]
struct NextStandaloneApp;

fn main() -> std::io::Result<()> {
    let temp_dir = std::env::temp_dir().join("mulyono_raja_tipu_tipu");
    std::fs::create_dir_all(&temp_dir).expect("Failed to create temp dir");

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

    println!("Node server is now started with PID {}", child.id());

    let status = child.wait()?;
    println!("Node server exited with status: {}", status);

    Ok(())
}
