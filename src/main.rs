use std::process::Command;

fn check_command(command: &str) {
  let process = Command::new(command).arg("--version").output();

  println!("{:=<120}", "=");
  println!();

  let process = match process {
    Ok(output) => {
      println!("{:?} found", command);
      println!("{}", String::from_utf8_lossy(&output.stdout));
    }
    Err(output) => println!("{}", output),
  };
}

fn main() {
  let commands = ["curl", "node", "docker", "plm"];

  for cmd in commands.iter() {
    check_command(cmd);
  }
}
