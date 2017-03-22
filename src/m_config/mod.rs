
extern crate config;


//Bool
pub fn get_bool(param: &String) -> bool{

    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    let content = c.get_bool(param);

    let mut return_bool: bool = false;
    match content{
        Some(entry) => return_bool = entry,
        None => return_bool = false,
    }
    return_bool

}

pub fn set_bool(name: &str, value: bool){
    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    c.set(name, value);
    c.refresh();
}


//Get String
pub fn get_string(param: &String) -> String{

    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();
    let content = c.get_str(param);

    let mut return_string: String = String::from("NONE FOUND");
    match content{
        Some(entry) => return_string = entry,
        None => return_string = String::from("No Such Entry"),
    }
    return_string

}

pub fn set_string(name: &str, value: &str){
    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    c.set(name, value);
    c.refresh();
}


//Get int
pub fn get_int(param: &String) -> i64{
    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();
    let content = c.get_int(param);

    let mut return_int: i64 = 0;
    match content{
        Some(entry) => return_int = entry,
        None => return_int = 0,
    }
    return_int
}

pub fn set_int(name: &str, value: i64){
    let mut c = config::Config::new();
    c.merge(config::File::new("Settings", config::FileFormat::Toml)).unwrap();

    c.set(name, value);
    c.refresh();
}
