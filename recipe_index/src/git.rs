use chrono::prelude::*;
use std::{path::Path, process::Command, str};

pub fn created_on(path: &Path) -> DateTime<Utc> {
    let output = Command::new("git")
        .args(&[
            "log",
            "--diff-filter=A",
            "--pretty=\"%aI\"",
            path.to_str().expect("non-utf8 path"),
        ])
        .output()
        .expect("git log to get created_on");

    str::from_utf8(&output.stdout)
        .expect("stderr to utf8")
        .trim()
        .trim_matches('"')
        .parse::<DateTime<Utc>>()
        .expect("parsing created on date")
}

pub fn last_modified(path: &Path) -> DateTime<Utc> {
    let output = Command::new("git")
        .args(&[
            "log",
            "-1",
            "--pretty=\"%aI\"",
            path.to_str().expect("non-utf8 path"),
        ])
        .output()
        .expect("git log to get last modified");

    str::from_utf8(&output.stdout)
        .expect("stderr to utf8")
        .trim()
        .trim_matches('"')
        .parse::<DateTime<Utc>>()
        .expect("parsing last modified date")
}
