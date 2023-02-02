mod types;

fn main() {
    tauri::Builder::default()
        .manage(state)
        //.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
