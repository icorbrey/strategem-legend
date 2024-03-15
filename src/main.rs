use config::Config;

pub mod code;
pub mod config;
pub mod direction;
pub mod strategems;

fn main() {
    let config = Config::load_from_yaml("./config.yaml")
        .expect("Should have been able to read `config.yaml`");

    for s in config.strategems.iter() {
        println!("Name: {}\nCode: {}\n", s.name, s.code);
    }
}
