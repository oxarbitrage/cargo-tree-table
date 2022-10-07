use std::{env, process::Command};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} path-to-manifest", args[0]);
        return Ok(());
    }

    let mut program = Command::new("cargo");

    let command = program
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
        .arg("--no-dedupe");

    let command_output = command.output().expect("failed to execute process");

    let output = String::from_utf8(command_output.stdout).expect("command should return output");

    let client = reqwest::Client::new();

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
            let version_with_prefix = pieces.next().expect("version should be available");

            let version_without_prefix = version_with_prefix.replace('v', "");

            let url_repo = pieces
                .next()
                .expect("url should be available")
                .replace('(', "")
                .replace(')', "");

            let mut name_cell = name.to_string();
            let mut version_cell = version_without_prefix.to_string();

            let mut url_prefix = url_repo.clone();
            url_prefix.truncate(19);
            if url_prefix == *"https://github.com/".to_string() {
                let attempt_url_with_v_prefix =
                    format!("{}/releases/tag/{}", url_repo.clone(), version_with_prefix);

                let fut = client.get(attempt_url_with_v_prefix.clone()).send();
                if fut.await?.status().as_u16() == 200 {
                    version_cell = format!(
                        "[{}]({})",
                        version_without_prefix, attempt_url_with_v_prefix
                    );
                } else {
                    let attempt_url_without_v_prefix = format!(
                        "{}/releases/tag/{}",
                        url_repo.clone(),
                        version_without_prefix
                    );
                    let fut = client.get(attempt_url_without_v_prefix.clone()).send();
                    if fut.await?.status().as_u16() == 200 {
                        version_cell = format!(
                            "[{}]({})",
                            version_without_prefix, attempt_url_without_v_prefix
                        );
                    }
                }
                name_cell = format!("[{}]({})", name, url_repo);
            }
            println!("| {} | {} ", name_cell, version_cell);
        }
    }

    Ok(())
}
