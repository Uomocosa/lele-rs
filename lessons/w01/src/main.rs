fn main() {
    println!("Hello, world!");
}


#[cfg(test)]
mod tests {
    // use super::*;
    use std::process::Command;

    #[test]
    fn run_from_cli() {
        let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "cargo run lesson::w01"])
            .output()
            .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("cargo run lesson::w01")
                .output()
                .expect("failed to execute process")
        };

        let output_msg = String::from_utf8(output.stdout).unwrap();
        print!("{output_msg:?}");
        // assert_eq!(output_msg, "hello\r\n");
    }

    #[ignore = "not a test"]
    #[test]
    fn run_from_cli_example() {
        let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "echo hello"])
            .output()
            .expect("failed to execute process")
        } else {
            Command::new("sh")
                .arg("-c")
                .arg("echo hello")
                .output()
                .expect("failed to execute process")
        };

        let output_msg = String::from_utf8(output.stdout).unwrap();
        // print!("{output_msg:?}");
        assert_eq!(output_msg, "hello\r\n");
    }

    
}
