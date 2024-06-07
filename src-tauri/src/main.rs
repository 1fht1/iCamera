// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu};

fn tray_menu(video_devices_menu: SystemTrayMenu) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "选择摄像头",
            video_devices_menu,
        ))
        .add_item(CustomMenuItem::new("reload".to_string(), "重新加载"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("show".to_string(), "显示"))
        .add_item(CustomMenuItem::new("hide".to_string(), "隐藏"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn reload(app: AppHandle) {
    app.emit_all("reload", "").unwrap();
}

#[derive(Deserialize)]
struct VideoDevice {
    label: String,
    device_id: String,
}

#[tauri::command(rename_all = "snake_case")]
fn update_video_menu(app: AppHandle, video_list: Vec<VideoDevice>) {
    let mut video_devices_menu = SystemTrayMenu::new();
    for VideoDevice { label, device_id } in video_list {
        video_devices_menu = video_devices_menu.add_item(CustomMenuItem::new(device_id.to_string(), label.to_string()));
    }
    app.tray_handle().set_menu(tray_menu(video_devices_menu)).unwrap()
}

fn menu_item_click_handler(app: &AppHandle, id: String) {
    match id.as_str() {
        "reload" => {
            app.emit_all("reload", "").unwrap()
        }
        "show" => {
            println!("show");
        }
        "hide" => {
            println!("hide");
        }
        "quit" => {
            app.exit(0);
        }
        other => { app.emit_all("videoSelect", other).unwrap() }
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![reload])
        .invoke_handler(tauri::generate_handler![update_video_menu])
        .system_tray(SystemTray::new()
            .with_menu(tray_menu(
                SystemTrayMenu::new()
                    .add_item(CustomMenuItem::new("".to_string(), "请重新加载"))
            ))
        )
        .on_system_tray_event(|app, event| {
            match event {
                SystemTrayEvent::MenuItemClick { id: device_id, .. } => {
                    menu_item_click_handler(app, device_id);
                }
                SystemTrayEvent::DoubleClick { .. } => {
                    app.get_window("main").unwrap().show().unwrap();
                }
                _ => {}
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
