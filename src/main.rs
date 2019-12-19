use std::process::Command;

use cursive::align::HAlign;
use cursive::event::EventResult;
use cursive::traits::*;
use cursive::views::{Dialog, OnEventView, SelectView, TextView};
use cursive::Cursive;

fn main() {
    let mut select = SelectView::new()
        .h_align(HAlign::Center)
        .autojump();

    let project_list = Command::new("gcloud")
                               .arg("projects").arg("list").arg("--verbosity=none")
                               .output()
                               .expect("failed to run gcloud command.");
    let result = String::from_utf8_lossy(&project_list.stdout);
    select.add_all_str(result.lines());

    select.set_on_submit(select_gcloud_project);

    // Vim-style navigation
    let select = OnEventView::new(select)
        .on_pre_event_inner('k', |s, _| {
            s.select_up(1);
            Some(EventResult::Consumed(None))
        })
        .on_pre_event_inner('j', |s, _| {
            s.select_down(1);
            Some(EventResult::Consumed(None))
        });

    let mut siv = Cursive::default();

    siv.add_fullscreen_layer(
        Dialog::around(select.scrollable()).title("Select a project"));

    siv.run();
}

fn select_gcloud_project(siv: &mut Cursive, project: &str) {
    let project_id = project.split_whitespace().collect::<Vec<&str>>()[0];
    Command::new("gcloud").arg("config").arg("set").arg("project")
            .arg(project_id)
            .output()
            .expect("failed to run gcloud command.");

    siv.pop_layer();
    let text = format!("Switched to {}", project_id);
    siv.add_layer(
        Dialog::around(TextView::new(text)).button("Quit", |s| s.quit()));
}
