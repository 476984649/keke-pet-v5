// 可可桌宠 v0.5 — Tauri 入口 + DB + 托盘 + AI
pub mod db; pub mod commands; pub mod ai; pub mod mood; pub mod journal; pub mod memory;

use commands::AppMemory;
use memory::MemoryStore;
use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use std::sync::Mutex;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let database = db::Database::open(&get_db_path()).expect("DB init failed");
    let memory = AppMemory(Mutex::new(MemoryStore::new(50)));

    tauri::Builder::default()
        .manage(database)
        .manage(memory)
        .setup(|app| {
            let show = MenuItemBuilder::with_id("show", "显示").build(app)?;
            let hide = MenuItemBuilder::with_id("hide", "隐藏").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app).item(&show).item(&hide).separator().item(&quit).build()?;
            TrayIconBuilder::new().menu(&menu).tooltip("可可桌宠")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => { if let Some(w) = app.get_webview_window("main") { w.show().ok(); w.set_focus().ok(); } }
                    "hide" => { if let Some(w) = app.get_webview_window("main") { w.hide().ok(); } }
                    "quit" => app.exit(0), _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        if let Some(w) = tray.app_handle().get_webview_window("main") { w.show().ok(); w.set_focus().ok(); }
                    }
                }).build(app)?;
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_affection, commands::add_pet, commands::add_feed,
            commands::login_today, commands::ai_chat, commands::get_mood,
            commands::gen_journal, commands::get_journal,
            commands::remember, commands::recall
        ])
        .run(tauri::generate_context!())
        .expect("启动失败");
}

fn get_db_path() -> String {
    std::env::current_exe().ok()
        .and_then(|p| p.parent().map(|d| d.join("keke.db")))
        .map(|p| p.to_string_lossy().to_string())
        .unwrap_or_else(|| "keke.db".into())
}
