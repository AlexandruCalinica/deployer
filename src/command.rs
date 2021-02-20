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

pub fn run_local(target: &str, name: &str, command: &str, port_map: &str) {
    cleanup(name);

    let target_path = path::PathBuf::from(target);
    let absolute_target_path = fs::canonicalize(&target_path).unwrap();
    let path = &absolute_target_path.to_str().unwrap();
    let cmd_str = format!(
        r#"run -dp {} --name {} -w /app -v {}:/app node:alpine sh -c"#,
        port_map, name, path
    );

    let mut splitted: Vec<&str> = cmd_str.split(' ').collect();
    splitted.push(command);

    run("docker", splitted);
}

pub fn cleanup(name: &str) {
    run("docker", vec!["rm", name]);
}
