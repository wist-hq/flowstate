use std::sync::{Arc, Mutex};

pub struct State {
    proc: Option<std::process::Child>,
}

type SharedState = Arc<Mutex<State>>;

#[tauri::command]
pub async fn start_preview(
    path: String,
    state: tauri::State<'_, SharedState>,
) -> Result<(), String> {
    stop_preview(state.clone())?;

    let proc = std::process::Command::new("tinymist")
        .arg("preview")
        .arg(&path)
        .arg("--no-open")
        .arg("--data-plane-host=localhost:8080")
        .spawn()
        .map_err(|e| e.to_string())?;

    let mut state = state.lock().unwrap();
    state.proc = Some(proc);

    Ok(())
}

#[tauri::command]
pub fn stop_preview(state: tauri::State<'_, SharedState>) -> Result<(), String> {
    let mut state = state.lock().unwrap();

    if let Some(mut proc) = state.proc.take() {
        let _ = proc.kill();
    }

    Ok(())
}

pub fn init() -> SharedState {
    Arc::new(Mutex::new(State { proc: None }))
}
