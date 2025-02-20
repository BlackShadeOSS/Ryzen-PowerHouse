use tauri::{
    Manager,
    tray::TrayIconBuilder,
    menu::{MenuBuilder, MenuItem, IsMenuItem},
    Emitter,
};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;
use std::process::Command;

static PRESETS: Mutex<Vec<Preset>> = Mutex::new(vec![]);

#[derive(Clone, Serialize, Deserialize)]
struct Preset {
    name: String,
    stapm_limit: Option<u32>,
    fast_limit: Option<u32>,
    slow_limit: Option<u32>,
    slow_time: Option<u32>,
    tctl_temp: Option<u32>,
    apu_skin_temp: Option<u32>,
    dgpu_skin_temp: Option<u32>,
    power_saving: bool,
    max_performance: bool,
}

#[tauri::command]
fn save_preset(preset: Preset) -> Result<(), String> {
    let mut presets = PRESETS.lock().unwrap();
    presets.push(preset);
    Ok(())
}

#[tauri::command]
fn get_presets() -> Result<Vec<Preset>, String> {
    let presets = PRESETS.lock().unwrap();
    Ok(presets.clone())
}

#[tauri::command]
async fn apply_preset(name: String) -> Result<(), String> {
    let presets = PRESETS.lock().unwrap();
    let preset = presets.iter().find(|p| p.name == name);

    if let Some(preset) = preset {
        // Build ryzenadj command
        let mut command: Vec<String> = vec!["ryzenadj".to_string()];  // Changed to Vec<String>

        // Helper function to add option with value
        let add_option = |option: &str, value: Option<u32>| {
            if let Some(v) = value {
                let value_str = v.to_string();
                vec![option.to_string(), value_str]  // Convert both to owned strings
            } else {
                vec![]
            }
        };

        // Add all options
        if let Some(limit) = preset.stapm_limit {
            command.extend(add_option("--stapm-limit", Some(limit)));
        }
        if let Some(limit) = preset.fast_limit {
            command.extend(add_option("--fast-limit", Some(limit)));
        }
        if let Some(limit) = preset.slow_limit {
            command.extend(add_option("--slow-limit", Some(limit)));
        }
        if let Some(time) = preset.slow_time {
            command.extend(add_option("--slow-time", Some(time)));
        }
        if let Some(temp) = preset.tctl_temp {
            command.extend(add_option("--tctl-temp", Some(temp)));
        }
        if let Some(temp) = preset.apu_skin_temp {
            command.extend(add_option("--apu-skin-temp", Some(temp)));
        }
        if let Some(temp) = preset.dgpu_skin_temp {
            command.extend(add_option("--dgpu-skin-temp", Some(temp)));
        }

        println!("Executing command: {:?}", command.join(" "));

        // Execute the command
        let status = Command::new(&command[0])
            .args(&command[1..])
            .status()
            .map_err(|e| format!("Failed to execute command: {}", e))?;

        if !status.success() {
            return Err(format!("Command failed with exit code: {}", status.code().unwrap_or(1)));
        }

        Ok(())
    } else {
        Err(format!("Preset not found: {}", name))
    }
}

fn enable_tray(app: &mut tauri::App) {
    use tauri::menu::{MenuBuilder, MenuItem};

    // Create default menu items
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>).unwrap();
    let configure_i = MenuItem::with_id(app, "open", "Configure", true, None::<&str>).unwrap();

    // Get presets and create menu items
    let presets = PRESETS.lock().unwrap();
    let mut custom_items = Vec::new();

    if presets.is_empty() {
        // Add disabled "No Presets" item
        let no_presets_i = MenuItem::with_id(app, "no_presets", "No Presets", false, None::<&str>).unwrap();
        custom_items.push(no_presets_i);
    } else {
        // Add preset items
        for preset in &*presets {
            let item = MenuItem::with_id(app, &preset.name, &preset.name, true, None::<&str>).unwrap();
            custom_items.push(item);
        }
    }

    // Convert Vec<MenuItem> to &[&dyn IsMenuItem]
    let items: Vec<&dyn IsMenuItem<_>> = custom_items
        .iter()
        .map(|item| item as &dyn IsMenuItem<_>)
        .collect();

    // Build menu with separators
    let menu = MenuBuilder::new(app)
        .items(&items)
        .separator()
        .item(&configure_i)
        .item(&quit_i)
        .build()
        .unwrap();

    // Create tray icon
    let _tray = TrayIconBuilder::with_id("tray")
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "quit" => {
                println!("Quit clicked");
                app.exit(0);
            }
            "open" => {
                println!("Open clicked");
                if let Some(window) = app.get_webview_window("main") {
                    window.show().unwrap();
                    window.set_focus().unwrap();
                }
            }
            "no_presets" => {
                println!("No presets available");
            }
            preset_id => {
                println!("Preset clicked: {}", preset_id);
                let _ = app.emit("apply_preset", preset_id).unwrap();  // Changed to emit
            }
            _ => println!("Unhandled menu item: {:?}", event.id),
        })
        .build(app)
        .unwrap();
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            enable_tray(app);
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
        .invoke_handler(tauri::generate_handler![save_preset, get_presets, apply_preset])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}