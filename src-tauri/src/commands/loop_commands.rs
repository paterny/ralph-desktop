use super::*;
use crate::engine::LoopEngine;
use std::path::PathBuf;

/// Start Ralph Loop for a project
#[tauri::command]
pub async fn start_loop(
    app_handle: AppHandle,
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let mut project_state = storage::load_project_state(&uuid).map_err(|e| e.to_string())?;

    let task = project_state
        .task
        .as_ref()
        .ok_or("No task configured for this project")?;

    let config = storage::load_config().map_err(|e| e.to_string())?;

    // Create loop engine
    let engine = LoopEngine::new(
        project_id.clone(),
        PathBuf::from(&project_state.path),
        task.cli,
        task.prompt.clone(),
        task.max_iterations,
        task.completion_signal.clone(),
        config.iteration_timeout_ms,
        config.idle_timeout_ms,
        app_handle.clone(),
    );

    // Store engine handle
    let handle = Arc::new(LoopEngineHandle {
        pause_flag: engine.get_pause_flag(),
        stop_flag: engine.get_stop_flag(),
        resume_notify: engine.get_resume_notify(),
    });

    {
        let mut loops = state.running_loops.write().await;
        loops.insert(uuid, handle);
    }

    // Update project status
    project_state.status = ProjectStatus::Running;
    project_state.execution = Some(ExecutionState {
        started_at: Utc::now(),
        paused_at: None,
        completed_at: None,
        current_iteration: 0,
        last_output: String::new(),
        last_error: None,
        last_exit_code: None,
    });
    project_state.updated_at = Utc::now();
    storage::save_project_state(&project_state).map_err(|e| e.to_string())?;

    // Spawn loop in background
    let state_clone = state.inner().clone();
    tokio::spawn(async move {
        let result = engine.start().await;

        // Update project state based on result
        if let Ok(mut project_state) = storage::load_project_state(&uuid) {
            match result {
                Ok(LoopState::Completed { iteration }) => {
                    project_state.status = ProjectStatus::Done;
                    if let Some(ref mut exec) = project_state.execution {
                        exec.completed_at = Some(Utc::now());
                        exec.current_iteration = iteration;
                    }
                }
                Ok(LoopState::Failed { iteration }) => {
                    project_state.status = ProjectStatus::Failed;
                    if let Some(ref mut exec) = project_state.execution {
                        exec.current_iteration = iteration;
                    }
                }
                Ok(LoopState::Idle) => {
                    project_state.status = ProjectStatus::Cancelled;
                }
                _ => {}
            }
            project_state.updated_at = Utc::now();
            let _ = storage::save_project_state(&project_state);
        }

        // Remove from running loops
        let mut loops = state_clone.running_loops.write().await;
        loops.remove(&uuid);
    });

    Ok(())
}

/// Pause Ralph Loop
#[tauri::command]
pub async fn pause_loop(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;

    let loops = state.running_loops.read().await;
    if let Some(handle) = loops.get(&uuid) {
        handle.pause_flag.store(true, std::sync::atomic::Ordering::SeqCst);

        // Update project status
        let mut project_state = storage::load_project_state(&uuid).map_err(|e| e.to_string())?;
        project_state.status = ProjectStatus::Pausing;
        project_state.updated_at = Utc::now();
        storage::save_project_state(&project_state).map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Loop not running for this project".to_string())
    }
}

/// Resume Ralph Loop
#[tauri::command]
pub async fn resume_loop(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;

    let loops = state.running_loops.read().await;
    if let Some(handle) = loops.get(&uuid) {
        handle.resume_notify.notify_one();

        // Update project status
        let mut project_state = storage::load_project_state(&uuid).map_err(|e| e.to_string())?;
        project_state.status = ProjectStatus::Running;
        if let Some(ref mut exec) = project_state.execution {
            exec.paused_at = None;
        }
        project_state.updated_at = Utc::now();
        storage::save_project_state(&project_state).map_err(|e| e.to_string())?;

        Ok(())
    } else {
        Err("Loop not running for this project".to_string())
    }
}

/// Stop Ralph Loop
#[tauri::command]
pub async fn stop_loop(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<(), String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;

    let loops = state.running_loops.read().await;
    if let Some(handle) = loops.get(&uuid) {
        handle.stop_flag.store(true, std::sync::atomic::Ordering::SeqCst);
        handle.resume_notify.notify_one(); // In case it's paused

        Ok(())
    } else {
        Err("Loop not running for this project".to_string())
    }
}

/// Get loop status for a project
#[tauri::command]
pub async fn get_loop_status(
    state: State<'_, AppState>,
    project_id: String,
) -> Result<bool, String> {
    let uuid = Uuid::parse_str(&project_id).map_err(|e| e.to_string())?;
    let loops = state.running_loops.read().await;
    Ok(loops.contains_key(&uuid))
}
