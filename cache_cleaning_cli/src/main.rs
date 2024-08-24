use directories::BaseDirs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    Command::new("pacman")
        .arg("-Scc")
        .output()
        .expect("Failed to execute first command");

    if let Some(base_dirs) = BaseDirs::new() {
        let cache_dir: PathBuf = base_dirs.cache_dir().to_path_buf();

        Command::new("rm")
            .arg("-r")
            .arg(cache_dir)
            .output()
            .expect("Failed to execute second command");
    } else {
        println!("Failed to get cache directory");
    }

    println!("Cache cleaned");
}
