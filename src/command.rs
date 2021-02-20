use process::{Command, Stdio};
use std::{fs, path, process};

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
}

pub fn run_local(target: &str, name: &str) {
    cleanup(name);

    let target_path = path::PathBuf::from(target);
    let absolute_target_path = fs::canonicalize(&target_path).unwrap();

    let cmd = format!(
        r#"run -d --name {} -v {:#?}:/app node:alpine"#,
        name, absolute_target_path
    )
    .replace("\"", "");

    let splitted: Vec<&str> = cmd.split(' ').collect();

    run("docker", splitted);
}

pub fn cleanup(name: &str) {
    run("docker", vec!["rm", name]);
}
