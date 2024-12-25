
use std::process::{Command,exit};

fn update_commint_push(){
    let add_command = Command::new("git")
        .arg("add")
        .arg("-A")
        .output()
        .expect("Failed to execute git and command")

    if !add_command.status.success(){
        exprintln!("Error: failed to add files to the git");
        exit(1);
    }
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generater())
        .output()
        .expect("failed to execute git command")
    if !commit_command.status.success(){
        exprintln!("Error;faild to commit");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("failed to execute git command");
    if !push_command.status.success(){
        exprintln!("Error: failed to commit");
        exit(1);
    }
    println!("Sucessfuly added,commited")
}
fn name_generater()->String {
    let mut generater = Generator::default();
    generater.next().unwrap();
}

fn main(){
    update_commit_push();
}