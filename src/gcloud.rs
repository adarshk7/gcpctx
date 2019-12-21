use std::process::Command;

pub fn get_project_list() -> Vec<String> {
    let command_output = Command::new("gcloud")
        .arg("projects")
        .arg("list")
        .arg("--verbosity=none")
        .output()
        .expect("Failed to run gcloud command.");

    String::from_utf8_lossy(&command_output.stdout)
        .lines()
        .skip(1)
        .map(|x| x.to_string())
        .collect()
}
