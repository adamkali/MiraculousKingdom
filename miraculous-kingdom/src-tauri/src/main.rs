mod types;
mod api;

fn main() {
    tauri::Builder::default()
        .manage(state)
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
