use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    let path = home::home_dir()
        .unwrap()
        .join(".config")
        .join("fount")
        .join("config");

    if !path.exists() {
        std::fs::create_dir_all(&path).unwrap();
    }

    path
}

pub fn config_file() -> PathBuf {
    config_dir().join("main.cfg")
}
