use std::process::{Command, Stdio};

pub fn run(cmd: &str, args: Vec<&str>) {
    let mut output = Command::new(cmd)
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();

    match output.wait() {
        Ok(status) => println!("{}", status),
        Err(err) => println!("{}", err),
    }
    println!();
}
