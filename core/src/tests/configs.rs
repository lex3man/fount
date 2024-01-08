use crate::{
    configs::{config_dir, config_file, init, save_config},
    models::Config,
};

#[test]
fn new_config() {
    if config_file().exists() {
        std::fs::create_dir_all(config_dir().join("tmp").to_str().unwrap()).unwrap();
        std::fs::copy(
            config_file().to_str().unwrap(),
            config_dir().join("tmp").join("main.cfg").to_str().unwrap(),
        )
        .expect("can't copy config");
        std::fs::remove_file(config_file()).unwrap();
    }
    let config = Config::by_default();
    save_config(&config).expect("oooops");

    if config_dir().join("tmp").join("main.cfg").exists() {
        std::fs::remove_file(config_file()).unwrap();
        std::fs::copy(
            config_dir().join("tmp").join("main.cfg").to_str().unwrap(),
            config_file().to_str().unwrap(),
        )
        .expect("can't copy config");
        std::fs::remove_file(config_dir().join("tmp").join("main.cfg")).unwrap();
        std::fs::remove_dir(config_dir().join("tmp")).unwrap();
    }
}

#[test]
fn edit_config() {
    let port: u16 = 8088;
    let mut config = init();

    config.settings.port = port.to_string();
    save_config(&config).unwrap();

    let config_port = config.get_port();

    assert_eq!(config_port, port);
}
