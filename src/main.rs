// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    use tauri::Manager;
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
                Ok(matches) => {
                    if let Some(arg) = matches.args.get("urlPath") {
                        let path = arg.value.clone().to_string();
                        let main_window  = app.get_window("main").expect("window not found");
                        if path != "false" {
                            main_window
                                .eval(&format!("window.location.pathname = {}", path))
                                .expect("failed to execute JavaScript code");
                        }
                    }
                }
                Err(_) => {}
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
