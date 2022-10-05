use std::{env, process::Command};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} path-to-manifest", args[0]);
        return;
    }

    let command_output = Command::new("cargo")
        .arg("tree")
        .arg("--manifest-path")
        .arg(args[1].clone())
        .arg("--depth")
        .arg("1")
        .arg("--prefix")
        .arg("none")
        .arg("--locked")
        .arg("--edges")
        .arg("normal")
        .arg("--workspace")
        .arg("--exclude")
        .arg("zebra-client")
        .arg("--exclude")
        .arg("zebra-test")
        .arg("--format")
        .arg("{p} {r}")
        .arg("--features")
        .arg("sentry,journald,filter-reload,prometheus")
        .output()
        .expect("failed to execute process");

    let output = String::from_utf8(command_output.stdout).expect("command should return output");

    let mut next_line_is_header = false;
    for (n, line) in output.lines().enumerate() {
        if line.is_empty() {
            next_line_is_header = true;
            continue;
        }

        let mut pieces = line.split(' ');

        if n == 0 || next_line_is_header {
            if n > 0 {
                println!();
                println!();
            }

            let name = pieces.next().expect("name should be available");
            let version = pieces.next().expect("version should be available");

            println!("## {} {}", name, version);
            println!();
            println!();
            println!("| Name | Version");
            println!("|------| -------");

            next_line_is_header = false;
        } else {
            let name = pieces.next().expect("name should be available");
            let version = pieces
                .next()
                .expect("version should be available")
                .replace('v', "");
            let url = pieces
                .next()
                .expect("url should be available")
                .replace('(', "")
                .replace(')', "");
            println!("| [{}]({}) | {} ", name, url, version);
        }
    }
}
