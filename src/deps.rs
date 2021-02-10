use std::process::Command;
pub struct Dependency {
    pub name: String,
}

impl Dependency {
    pub fn check(&self) {
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
