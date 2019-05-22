use std::process::Command;
use std::str;
use std::env;

mod passwd;

fn check_env() -> Option<String> {
    let vec = vec!["GIT_AUTHOR_NAME", "GIT_COMMITTER_NAME", "HGUSER", "C9_USER"];
    for x in &vec {
        if let Ok(name) = env::var(&x) {
            return Some(name);
        }
    }
    return None
}

fn check_git() -> Option<String> {
    let output = Command::new("git")
            .arg("config")
            .arg("--global")
            .arg("user.name")
            .output()
            .expect("failed to execute");

    let _ = match str::from_utf8(&output.stdout) {
        Ok(v) => return Some(v.to_string()),
        Err(_) => return None
    };
}

fn main() {
    if let Some(name) = check_env() {
        println!("{}", name);
    } else if let Some(name) = check_git() {
        println!("{}", name);
    } else if let Some(name) = passwd::get_user(){
        println!("{}", name);
    }
}
