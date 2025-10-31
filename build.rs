use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() {
    let standalone_dir = Path::new("my-app/.next/standalone");
    let output_dir = Path::new("my-app/.next/standalone_resolved");

    println!("cargo:rerun-if-changed=my-app/.next/standalone");

    if !standalone_dir.exists() {
        println!(
            "cargo:warning=Standalone directory does not exist yet: {:?}",
            standalone_dir
        );
        return;
    }

    if output_dir.exists() {
        fs::remove_dir_all(output_dir).expect("Failed to remove old output directory");
    }

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    copy_dir_resolve_symlinks(standalone_dir, output_dir)
        .expect("Failed to copy and resolve symlinks");

    println!(
        "cargo:warning=Symlinks resolved and copied to: {:?}",
        output_dir
    );
}

fn copy_dir_resolve_symlinks(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let file_name = entry.file_name();
        let dst_path = dst.join(&file_name);

        let metadata = fs::metadata(&src_path)?;

        if metadata.is_dir() {
            copy_dir_resolve_symlinks(&src_path, &dst_path)?;
        } else {
            let symlink_metadata = fs::symlink_metadata(&src_path)?;

            if symlink_metadata.file_type().is_symlink() {
                let target = fs::read_link(&src_path)?;
                let resolved_path = if target.is_absolute() {
                    target
                } else {
                    src_path.parent().unwrap().join(&target).canonicalize()?
                };

                println!(
                    "cargo:warning=Resolving symlink: {:?} -> {:?}",
                    src_path, resolved_path
                );

                if resolved_path.is_dir() {
                    copy_dir_resolve_symlinks(&resolved_path, &dst_path)?;
                } else {
                    fs::copy(&resolved_path, &dst_path)?;
                }
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
    }

    Ok(())
}
