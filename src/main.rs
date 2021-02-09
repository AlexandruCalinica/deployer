use std::process::{Command, Stdio};
use std::vec;

struct Dependency {
    name: String,
}

impl Dependency {
    fn check(&self) {
        let output = Command::new(&self.name).arg("--version").output();

        println!("{:=<120}", "=");
        println!();

        match output {
            Ok(res) => {
                println!("{:?} found", self.name);
                println!("{}", String::from_utf8_lossy(&res.stdout));
            }
            Err(err) => {
                println!("{:?} is not installed", self.name);
                println!("{}", err);
            }
        };
    }
}

fn run(cmd: &str, arg: &str) {
    let mut output = Command::new(cmd)
        .arg(arg)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    println!("{:=<120}", "=");
    println!();

    let std = output.wait();
    println!("{:?}", std);
}

fn main() {
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

    run("docker-compose", "up");
}
