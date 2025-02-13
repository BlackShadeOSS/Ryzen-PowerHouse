// src-tauri/src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    tray::TrayIconBuilder,
    menu::{Menu, MenuItem},
};

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the app handle.
            let app_handle = app.handle();

            // Create tray menu items.
            let quit_item = MenuItem::with_id(
                &app_handle.clone(),
                "quit",
                "Quit",
                true,
                None::<&str>,
            )?;
            let configure_item = MenuItem::with_id(
                &app_handle.clone(),
                "open",
                "Configure",
                true,
                None::<&str>,
            )?;

            // Create a tray menu with the items.
            let tray_menu = Menu::with_items(&app_handle.clone(), &[&configure_item, &quit_item])?;

            // Build the tray icon.
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(true)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        println!("Quit clicked");
                        app.exit(0);
                    }
                    "open" => {
                        println!("Open clicked");
                        // Use get_webview_window; in Tauri 2 this method may be used instead of get_window.
                        if let Some(window) = app.get_webview_window("main") {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }
                    other => {
                        println!("Unrecognized tray event: {:?}", other);
                    }
                })
                .build(&app_handle.clone())?;

            // Hide the main window on startup.
            if let Some(window) = app.get_webview_window("main") {
                window.hide().unwrap();
            }

            Ok(())
        })
        .on_window_event(|app, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if let Some(window) = app.get_webview_window("main") {
                    window.hide().unwrap();
                }
                api.prevent_close();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
