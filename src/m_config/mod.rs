


pub struct config_values {
    pub last_folder: String,
}

pub fn create_config() -> config_values{
    let mut conf: config_values = config_values{last_folder: String::from("/home/")};
    conf
}
