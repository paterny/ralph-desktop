use crate::auto_update::{self, AutoUpdateService, UpdateState};
use crate::commands::AppState;
use tauri::State;

#[tauri::command]
pub async fn get_update_state(state: State<'_, AppState>) -> Result<UpdateState, String> {
    let update_state = state.update_state.read().await;
    Ok(update_state.clone())
}

#[tauri::command]
pub async fn check_for_updates(
    state: State<'_, AppState>,
    idle_ok: bool,
) -> Result<UpdateState, String> {
    let service = AutoUpdateService::new();
    let current_version = env!("CARGO_PKG_VERSION");
    let next = service
        .check_and_download(current_version, idle_ok)
        .await
        .map_err(|e| e.to_string())?;

    let mut update_state = state.update_state.write().await;
    *update_state = next.clone();
    Ok(next)
}

#[tauri::command]
pub async fn load_update_state_cmd(state: State<'_, AppState>) -> Result<UpdateState, String> {
    let mut loaded = auto_update::load_update_state().map_err(|e| e.to_string())?;
    loaded.current_version = env!("CARGO_PKG_VERSION").to_string();
    let mut update_state = state.update_state.write().await;
    *update_state = loaded.clone();
    Ok(loaded)
}
