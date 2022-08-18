#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  AppHandle,
  SystemTray,
  SystemTrayEvent,
  CustomMenuItem,
  SystemTrayMenu,
  generate_handler,
};

use rand::seq::SliceRandom;

fn update_tray_locale(locale: String, handle: AppHandle) {
  let tray = handle.tray_handle();
  let settings = tray.get_item("settings");
  match locale.as_str() {
    "en" => settings.set_title("Settings").unwrap(),
    "zh" => settings.set_title("设置").unwrap(),
    "zhtw" => settings.set_title("設置").unwrap(),
    "ja" => settings.set_title("設定").unwrap(),
    _=>{},
  }
}

fn tray() -> SystemTray {
  let update_locale = CustomMenuItem::new("update_locale", "Random Update Locale");
  let settings = CustomMenuItem::new("settings", "Settings(init)");
  let tray_menu = SystemTrayMenu::new()
    .add_item(update_locale)
    .add_item(settings);

  let tray = SystemTray::new().with_menu(tray_menu);

  return tray;
}

fn tray_event(handle: &AppHandle, event: SystemTrayEvent) {
    match event {
      SystemTrayEvent::MenuItemClick { id, .. } => {
        // println!("{:?}", id);
        match id.as_str() {
          "update_locale" => {
            let locales = vec!["en", "zh", "zhtw", "ja"];
            let locale = locales.choose(&mut rand::thread_rng());
            update_tray_locale(locale.unwrap().to_string(), handle.clone());
          },
          _ => {}
        }
      }
      _ => {}
    }
}



#[tauri::command]
async fn update_locale(locale: String, handle: AppHandle) -> Result<(), String> {
  update_tray_locale(locale, handle);
  Ok(())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(generate_handler![update_locale])
    .system_tray(tray())
    .on_system_tray_event(tray_event)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
