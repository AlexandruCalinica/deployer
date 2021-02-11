use std::process::Command;
use std::vec;
pub struct Dependency {
    pub name: String,
}

impl Dependency {
    pub fn check(&self) {
        let output = Command::new(&self.name).arg("--version").output();

        println!("{:=<120}", "=");

        match output {
            Ok(_) => {
                println!("{:?} found", self.name);
            }
            Err(_) => {
                println!("{:?} is not installed", self.name);
            }
        };
    }
}

pub fn check_dependencies() {
    let mut commands = vec![];
    commands.insert(0, "curl".to_string());
    commands.insert(0, "node".to_string());
    commands.insert(0, "docker".to_string());
    commands.insert(0, "plm".to_string());

    for name in commands.iter() {
        Dependency {
            name: name.to_string(),
        }
        .check();
    }
}
