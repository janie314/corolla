use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
};

pub fn get_root_dir() -> PathBuf {
    Path::new(env!("CARGO_BIN_EXE_corolla"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf()
}

pub fn cleanup(pid: Option<u32>) {
    let path = get_root_dir();
    for file in [
        "corolla-test.sqlite3",
        "corolla-test.sqlite3-shm",
        "corolla-test.sqlite3-wal",
    ]
    .iter()
    {
        Command::new("rm")
            .arg(Path::new(&path).join("tmp").join(file).to_str().unwrap())
            .output()
            .expect("could not execute cleanup step");
    }
    match pid {
        Some(pid) => {
            Command::new("kill")
                .arg(pid.to_string())
                .output()
                .expect("could not kill corolla; this will require manual cleanup");
        }
        None => (),
    }
}

fn server<S>(spec_path: &S) -> Child
where
    S: AsRef<OsStr> + ?Sized,
{
    let path = get_root_dir();
    Command::new(env!("CARGO_BIN_EXE_corolla"))
        .arg("-s")
        .arg(spec_path)
        .arg("-d")
        .arg(
            Path::new(&path)
                .join("tmp")
                .join("corolla-test.sqlite3")
                .to_str()
                .unwrap(),
        )
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!(
            "failed to run corolla with {}",
            path.to_string_lossy()
        ))
}
