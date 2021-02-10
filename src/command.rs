use std::process::{Command, Stdio};

pub fn run(cmd: &str, arg: &str) {
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
