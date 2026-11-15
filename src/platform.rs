//! Small OS differences in one place: data folders, finding tools, virtualenv layout, hidden child windows.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Folder for Bolo's own files (progress, recordings, voices, Python environments).
pub fn data_dir() -> PathBuf {
    #[cfg(windows)]
    let base = std::env::var_os("LOCALAPPDATA").map(PathBuf::from).or_else(|| std::env::var_os("USERPROFILE").map(|h| PathBuf::from(h).join("AppData").join("Local")));
    #[cfg(not(windows))]
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")));
    base.unwrap_or_else(|| PathBuf::from(".")).join("bolo")
}

/// The user's Documents folder (backups go here).
pub fn documents_dir() -> PathBuf {
    #[cfg(windows)]
    let home = std::env::var_os("USERPROFILE");
    #[cfg(not(windows))]
    let home = std::env::var_os("HOME");
    home.map(PathBuf::from).unwrap_or_default().join("Documents")
}

/// `bin/<name>` on Linux, `Scripts\<name>.exe` on Windows, inside a Python virtualenv.
pub fn venv_bin(venv: &Path, name: &str) -> PathBuf {
    #[cfg(windows)]
    return venv.join("Scripts").join(format!("{name}.exe"));
    #[cfg(not(windows))]
    return venv.join("bin").join(name);
}

/// Is `name` an executable on PATH? (Honours PATHEXT on Windows.)
pub fn which(name: &str) -> Option<PathBuf> {
    let paths = std::env::var_os("PATH")?;
    #[cfg(windows)]
    let exts: Vec<String> = std::env::var("PATHEXT").unwrap_or(".EXE;.CMD;.BAT".into()).split(';').map(|e| e.to_lowercase()).collect();
    for dir in std::env::split_paths(&paths) {
        let plain = dir.join(name);
        if plain.is_file() {
            return Some(plain);
        }
        #[cfg(windows)]
        for e in &exts {
            let p = dir.join(format!("{name}{e}"));
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

/// Stop Windows from flashing a console window for every child process.
pub fn quiet(cmd: &mut Command) -> &mut Command {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    cmd
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn venv_layout() {
        let p = venv_bin(Path::new("v"), "python");
        #[cfg(windows)]
        assert!(p.ends_with("Scripts/python.exe") || p.ends_with("Scripts\\python.exe"));
        #[cfg(not(windows))]
        assert!(p.ends_with("bin/python"));
    }
    #[test]
    fn data_dir_ends_with_bolo() {
        assert!(data_dir().ends_with("bolo"));
    }
}
