// 可可桌宠 v0.1 — Tauri 入口 + DB + 托盘
pub mod db; pub mod commands;

use tauri::Manager;
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化数据库
    let db_path = get_db_path();
    let database = db::Database::open(&db_path).expect("DB init failed");

    tauri::Builder::default()
        .manage(database)
        .setup(|app| {
            let show = MenuItemBuilder::with_id("show", "显示").build(app)?;
            let hide = MenuItemBuilder::with_id("hide", "隐藏").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;
            let menu = MenuBuilder::new(app).item(&show).item(&hide).separator().item(&quit).build()?;
            TrayIconBuilder::new().menu(&menu).tooltip("可可桌宠")
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show" => { if let Some(w) = app.get_webview_window("main") { w.show().ok(); w.set_focus().ok(); } }
                    "hide" => { if let Some(w) = app.get_webview_window("main") { w.hide().ok(); } }
                    "quit" => app.exit(0),
                    _ => {}
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
            commands::add_minutes, commands::login_today
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
