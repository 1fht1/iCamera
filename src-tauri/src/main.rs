// Prevents an additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

use serde::Deserialize;
use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu, SystemTrayMenuItem, SystemTraySubmenu, Wry};
use tauri_plugin_store::{JsonValue, StoreCollection, with_store};

fn get_form_store(app: &AppHandle, key: String) -> Result<JsonValue, tauri_plugin_store::Error> {
    let stores = app.state::<StoreCollection<Wry>>();
    let path = PathBuf::from("~/camera/store.json");
    with_store(app.app_handle(), stores, path, |store| {
        Ok(store.get(&key).unwrap().clone())
    })
}

fn tray_menu(video_devices_menu: SystemTrayMenu) -> SystemTrayMenu {
    SystemTrayMenu::new()
        .add_submenu(SystemTraySubmenu::new(
            "选择摄像头",
            video_devices_menu,
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_submenu(SystemTraySubmenu::new(
            "窗口大小",
            SystemTrayMenu::new()
                .add_item(CustomMenuItem::new("window_size_large".to_string(), "大"))
                .add_item(CustomMenuItem::new("window_size_medium".to_string(), "中"))
                .add_item(CustomMenuItem::new("window_size_small".to_string(), "小"))
                .add_item(CustomMenuItem::new("window_size_tiny".to_string(), "迷你")),
        ))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("reload".to_string(), "重新加载"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("shape".to_string(), "形状"))
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(CustomMenuItem::new("quit".to_string(), "退出"))
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn reload(app: AppHandle) {
    app.emit_all("reload", "").unwrap();
}

#[derive(Deserialize, Debug)]
struct VideoDevice {
    label: String,
    device_id: String,
}

#[tauri::command]
fn update_video_menu(app: AppHandle, video_list: Vec<VideoDevice>) {
    let mut video_devices_menu = SystemTrayMenu::new();
    for VideoDevice { label, device_id } in video_list {
        video_devices_menu = video_devices_menu.add_item(CustomMenuItem::new("video".to_owned() + &*device_id, label));
    }
    app.tray_handle().set_menu(tray_menu(video_devices_menu)).unwrap()
}

fn menu_item_click_handler(app: &AppHandle, id: String) {
    match id.as_str() {
        "reload" => {
            app.emit_all("reload", "").unwrap();
        }
        "shape" => {
            app.emit_all("shape", "").unwrap()
        }
        "quit" => {
            app.exit(0);
        }
        device_id if device_id.starts_with("video") => {
            app.emit_all("videoSelect", device_id.strip_prefix("video")).unwrap()
        }
        window_size_id if window_size_id.starts_with("window_size_") => {
            app.emit_all("windowSize", window_size_id.strip_prefix("window_size_")).unwrap()
        }
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![reload, update_video_menu])
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
