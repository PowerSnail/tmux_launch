use std::process::{Command, Stdio, exit};
use std::io::Result;

fn main() -> Result<()> {
    let output = Command::new("tmux")
        .arg("list-sessions")
        .output()
        .expect("failed to execute process");
    let args: Vec<&str> = output
        .stdout
        .split(|&char| char == '\n' as u8)
        .flat_map(|line| line.splitn(2, |&c| c == ':' as u8))
        .map(|bytes| std::str::from_utf8(bytes).unwrap())
        .collect();

    let arg_count = args.len() / 2;
    match arg_count {
        0 => Command::new("tmux")
            .spawn()?,
        1 => Command::new("tmux")
            .arg("a")
            .spawn()?,
        n => {
            let child = Command::new("dialog")
                .arg("--menu")
                .arg("Select a session") // Title
                .arg("0") // Height
                .arg("0") // Width
                .arg(format!("{}", n)) // menu height
                .args(&args[0..n * 2]) // arg list for options, drop the last element ("")
                .stderr(Stdio::piped()) // collect only stderr
                .spawn()?;

            let output = child
                .wait_with_output()?;
            
            if !output.status.success() {
                exit(output.status.code().unwrap());
            }

            let answer = output.stderr;
            let choice = std::str::from_utf8(&answer).unwrap();
            Command::new("tmux")
                .arg("a")
                .arg("-t")
                .arg(&choice)
                .spawn()?
        }
    }
    .wait()
    .expect("Failed to run tmux");
    Ok(())
}
