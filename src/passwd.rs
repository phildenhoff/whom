use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
    process::Command,
    str
};
use libc;

#[cfg(target_os = "macos")]
fn get_platform() -> &'static str {
    return "macos"
}

#[cfg(target_os = "linux")]
fn get_platform() -> &'static str {
    return "linux"
}

#[cfg(all(not(target_os = "macos"), not(target_os = "linux")))]
fn get_platform() -> &'static str {
    return "unsupported"
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn read_user_from_line(line: String, username_index: usize) -> String {
    // Darwin passwd(5)
    // 0 name   User's login name
    // 1 password   User's encrypted password.
    // 2 uid        User's id.
    // 3 gid        User's login group id.
    // 4 class      Unused.
    // 5 change     Password change time.
    // 6 expiry     Account expiration time.
    // 7 gecos      User's full name.
    // 8 home_dir   User's home directory.
    // 9 shell      User's login shell.

	// Linux passwd(5):
	// 0 login name
	// 1 optional encrypted password
	// 2 numerical user ID
	// 3 numerical group ID
	// 4 user name or comment field
	// 5 user home directory
	// 6 optional user command interpreter
    return String::from(line.split(":").nth(username_index).unwrap());
}

fn extract_linux(filename: &str, uid: i32) -> String {
    let lines = lines_from_file(filename);
    let mut username_line: String = String::from("None");
    for line in lines{
        if line.contains(&*uid.to_string()) {
            username_line = line;
        }
    }
    return username_line;
}

fn extract_macos(uid: i32) -> String {
    let output = Command::new("/usr/bin/id")
        .arg("-P")
        .arg(uid.to_string())
        .output()
        .expect("failed to execute");

    let _ = match str::from_utf8(&output.stdout) {
        Ok(v) => return v.to_string(),
        Err(_) => return String::new()
    };
}

fn get_current_uid() -> i32 {
    unsafe { libc::getuid() as i32 }
}

/* Returns the current user, using passwd.
 */
pub fn get_user() -> Option<String> {
    let uid: i32 =  get_current_uid(); // TODO: this
    let platform = get_platform();

    let username = match platform {
        "macos" => Some(read_user_from_line(extract_macos(uid), 7)),
        "linux" => Some(read_user_from_line(
            extract_linux("/etc/passwd", uid), 0)),
        &_ => None
    };

    return username
}
