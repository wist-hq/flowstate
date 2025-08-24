use std::{
    fs,
    process::{Child, Command, Stdio},
    sync::{mpsc::Sender, Arc, Mutex},
};

use notify::{Event, RecursiveMode, Watcher};
use tauri::Emitter;
use tempfile::TempDir;

pub struct State {
    child: Option<Child>,
    thread: Option<std::thread::JoinHandle<()>>,
    shutdown_tx: Option<Sender<()>>,
}

type SharedState = Arc<Mutex<State>>;

#[tauri::command]
pub async fn watch(
    app_handle: tauri::AppHandle,
    path: String,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    stop_watch(state.clone())?;

    let tmp_dir = TempDir::new().map_err(|e| e.to_string())?;
    let out_path = tmp_dir.path().join("out.html");

    let child = Command::new("typst")
        .arg("watch")
        .arg(&path)
        .arg(&out_path)
        .arg("--format")
        .arg("html")
        .arg("--no-serve")
        .arg("--no-reload")
        .env("TYPST_FEATURES", "html")
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
        .map_err(|e| e.to_string())?;

    let app_clone = app_handle.clone();
    let out_path_clone = out_path.clone();

    let (tx, rx) = std::sync::mpsc::channel::<()>();

    let thread = std::thread::spawn(move || {
        let mut last_html = String::new();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                if event.paths.contains(&out_path_clone) {
                    if let Ok(html) = fs::read_to_string(&out_path_clone) {
                        if html != last_html {
                            last_html = html.clone();
                            let _ = app_clone.emit("typst-update", html);
                        }
                    }
                }
            }
        })
        .unwrap();

        println!("{:?}", tmp_dir.path());
        watcher
            .watch(tmp_dir.path(), RecursiveMode::NonRecursive)
            .unwrap();

        let _ = rx.recv();
    });

    let mut state = state.lock().unwrap();
    state.child = Some(child);
    state.thread = Some(thread);
    state.shutdown_tx = Some(tx);

    Ok(())
}

#[tauri::command]
pub fn stop_watch(state: tauri::State<'_, SharedState>) -> Result<(), String> {
    let mut state = state.lock().unwrap();

    if let Some(mut child) = state.child.take() {
        let _ = child.kill();
    }

    if let Some(tx) = state.shutdown_tx.take() {
        let _ = tx.send(());
    }

    if let Some(thread) = state.thread.take() {
        let _ = thread.join();
    }

    Ok(())
}

pub fn init() -> SharedState {
    Arc::new(Mutex::new(State {
        child: None,
        thread: None,
        shutdown_tx: None,
    }))
}
