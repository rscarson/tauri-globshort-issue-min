use std::{thread};
use tauri::{ Manager, GlobalShortcutManager };

fn spawn_handler() {
    thread::spawn(move || {
        println!("Received keyboard event!");
    });
}

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let mut manager = app.app_handle().global_shortcut_manager();
        if manager.unregister_all().is_ok() {
            manager.register("CmdOrCtrl+Space", move || spawn_handler()).ok();
        }
        
        Ok(())
      })
    .run(tauri::generate_context!())
    .expect("unable to run Tauri application");
}