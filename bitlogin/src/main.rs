#![feature(try_trait)]

use bitlogin::{MyError, User};
use clap::{load_yaml, App};
use maplit::hashmap;

fn run() -> Result<(), MyError> {
    let yaml = load_yaml!("../cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let map = hashmap!{
        "web" => "8",
        "mobile" => "1",
        "library" => "1",
    };

    let action = matches.value_of("action")?;
    let username = matches.value_of("username")?;
    let password = matches.value_of("password")?;

    let user = User::new(username, password)?;

    if action == "login" {
        println!("{}", user.login()?.pretty(2));
    } else if action == "logout" {
        let wlan_type = matches.value_of("type").unwrap_or("web");
        let acid = map.get(wlan_type)?;
        println!("{}", user.logout(acid)?.pretty(2));
    }

    Ok(())
}

fn main() {
    match run() {
        Ok(_t) => (),
        Err(e) => eprintln!("{}", e),
    }
}
