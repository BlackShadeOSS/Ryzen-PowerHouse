use tauri::{
    Manager,
    tray::TrayIconBuilder,
    menu::{MenuBuilder, MenuItem, IsMenuItem},
};
use serde::{Serialize, Deserialize};
use std::sync::Mutex;

// Global presets vector
static PRESETS: Mutex<Vec<Preset>> = Mutex::new(vec![]);

#[derive(Clone, Serialize, Deserialize)]
struct Preset {
    name: String,
    stapm_limit: Option<u32>,        // Sustained Power Limit (mW)
    fast_limit: Option<u32>,         // Actual Power Limit (mW)
    slow_limit: Option<u32>,         // Average Power Limit (mW)
    slow_time: Option<u32>,          // Slow PPT Constant Time (s)
    tctl_temp: Option<u32>,          // Tctl Temperature Limit (°C)
    apu_skin_temp: Option<u32>,      // APU Skin Temperature Limit (°C)
    dgpu_skin_temp: Option<u32>,     // dGPU Skin Temperature Limit (°C)
    power_saving: bool,              // Power saving mode
    max_performance: bool,           // Max performance mode
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

fn enable_tray(app: &mut tauri::App) {
    use tauri::menu::{MenuBuilder, MenuItem, IsMenuItem};

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
        for preset in &*presets {  // Dereference the MutexGuard
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
                // Handle preset selection
            }
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
        .invoke_handler(tauri::generate_handler![save_preset, get_presets])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}