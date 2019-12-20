use std::process::Command;

static error_message: &str = "Failed to run gcloud command.";

pub fn get_project_list<'a>() -> &'a Vec<&'a str> {
    let command_output =
        Command::new("gcloud")
                .arg("projects").arg("list").arg("--verbosity=none")
                .output().expect("Failed to run gcloud command.");

    // Ignore first row because those are just titles.
    let parsed_rows = String::from_utf8_lossy(&command_output.stdout).lines().collect::<Vec<&str>>();
    return &parsed_rows[1..].to_vec();
}