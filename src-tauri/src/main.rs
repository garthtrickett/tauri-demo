#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State, Emitter};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AppState {
  pub title: String,
  pub count: i32,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum Action {
  DecrementCounter,
  IncrementCounter,
  ResetCounter,
}

pub struct SAMManager {
  pub state: AppState,
  reactor: Option<Box<dyn Fn() + Send + Sync>>,
  app_handle: AppHandle,
}

impl SAMManager {
  pub fn new(initial_state: AppState, app_handle: AppHandle) -> Self {
    Self {
      state: initial_state,
      reactor: None,
      app_handle,
    }
  }

  /// Registers a reactor function that can be triggered later.
  pub fn set_reactor<F>(&mut self, reactor: F)
  where
    F: Fn() + Send + Sync + 'static,
  {
    self.reactor = Some(Box::new(reactor));
  }

  /// Applies the given action to update the state and emits the update.
  /// It then calls the special trigger handler.
  pub fn apply(&mut self, action: Action) {
    // Update state based on the action.
    match action {
      Action::DecrementCounter => {
        if self.state.count > 0 {
          self.state.count -= 1;
        }
      }
      Action::IncrementCounter => {
        self.state.count += 1;
      }
      Action::ResetCounter => {
        self.state.count = 3;
        self.state.title = "Counter reset!".to_string();
      }
    }

    // Emit the updated state.
    let _ = self.app_handle.emit("state_updated", self.state.clone());

    // Handle any special triggers based on the counter value.
    self.handle_api_triggers();
  }

  /// Checks the counter and triggers API calls when necessary.
  fn handle_api_triggers(&mut self) {
    match self.state.count {
      0 => {
        self.state.title = "API call in progress...".to_string();
        let _ = self.app_handle.emit("state_updated", self.state.clone());
        if let Some(ref reactor) = self.reactor {
          reactor();
        }
      }
      5 => {
        self.state.title = "Counter reached 5! second trigger activated.".to_string();
        let _ = self.app_handle.emit("state_updated", self.state.clone());
      }
      _ => {}
    }
  }
}

/// A thin wrapper around SAMManager for shared, thread-safe access.
struct SharedSAM(Arc<Mutex<SAMManager>>);

/// Configures the reactor for when the counter reaches zero.
fn setup_reactor(shared_sam: Arc<Mutex<SAMManager>>) {
  let app_handle = {
    let sam = shared_sam.lock().unwrap();
    sam.app_handle.clone()
  };

  let shared_sam_for_reactor = Arc::clone(&shared_sam);
  let reactor_closure = move || {
    let app_handle_inner = app_handle.clone();
    let shared_sam_inner = Arc::clone(&shared_sam_for_reactor);
    std::thread::spawn(move || {
      std::thread::sleep(std::time::Duration::from_secs(2));
      let mut sam = shared_sam_inner.lock().unwrap();
      sam.state.title = "Reactor finished external API call".to_string();
      let _ = app_handle_inner.emit("state_updated", sam.state.clone());
    });
  };

  let mut sam = shared_sam.lock().unwrap();
  sam.set_reactor(reactor_closure);
}

#[tauri::command]
fn decrement_counter(state: State<SharedSAM>) -> AppState {
  let mut sam = state.0.lock().unwrap();
  sam.apply(Action::DecrementCounter);
  sam.state.clone()
}

#[tauri::command]
fn increment_counter(state: State<SharedSAM>) -> AppState {
  let mut sam = state.0.lock().unwrap();
  sam.apply(Action::IncrementCounter);
  sam.state.clone()
}

#[tauri::command]
fn reset_counter(state: State<SharedSAM>) -> AppState {
  let mut sam = state.0.lock().unwrap();
  sam.apply(Action::ResetCounter);
  sam.state.clone()
}

#[tauri::command]
fn get_state(state: State<SharedSAM>) -> AppState {
  let sam = state.0.lock().unwrap();
  sam.state.clone()
}

fn main() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle().clone();
      let initial_state = AppState {
        title: "Initial Hello From Rust!".to_string(),
        count: 3,
      };
      let sam_manager = SAMManager::new(initial_state, app_handle);
      let shared_sam = Arc::new(Mutex::new(sam_manager));
      setup_reactor(Arc::clone(&shared_sam));
      app.manage(SharedSAM(shared_sam));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      decrement_counter,
      increment_counter,
      reset_counter,
      get_state
    ])
    .run(tauri::generate_context!())
    .expect("Error while running Tauri application");
}
