use log::info;
use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Stdio,
};
use tokio::time::{sleep, Duration};
use tokio::{
    net::TcpStream,
    process::{Child, Command},
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

pub async fn cleanup(kill_db: bool, proc: Option<&mut Child>) {
    if kill_db {
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
                .await
                .expect("could not execute cleanup step");
        }
    }
    match proc {
        Some(proc) => {
            proc.kill().await.expect("could not kill server process");
        }
        None => (),
    }
}

pub async fn server<S>(spec_path: &S) -> Child
where
    S: AsRef<OsStr> + ?Sized,
{
    let path = get_root_dir();
    let proc = Command::new(env!("CARGO_BIN_EXE_corolla"))
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
        .arg("-r")
        .arg("/test")
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .expect(&format!(
            "failed to run corolla with {}",
            path.to_string_lossy()
        ));
    // don't return until the server is fully started and ready to use
    while TcpStream::connect("localhost:50000").await.is_err() {
        info!("waiting to connect to corolla server");
        sleep(Duration::from_secs(1)).await;
    }
    proc
}
