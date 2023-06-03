// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use reqwest;
use std::error::Error;
use std::fmt;



#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for CustomError {}


#[tauri::command(async)]
async fn get_data(url: String) -> String {
    let res = reqwest::get(&url).await;
    match res {
        Ok(response) => {
            println!("Status: {}", response.status());
            let body = response.text().await;
            match body {
                Ok(body) => {
                    return body;
                }
                Err(err) => {
                    eprintln!("Failed to read response body: {:?}", err);
                }
            }
        }
        Err(err) => {
            eprintln!("Failed to send request: {:?}", err);
        }
    }
    //404
    return String::from("404");
}


#[tauri::command]
fn close_window(window: Window) {
    window.close().unwrap();
}

#[tauri::command]
fn maximize_window(window: Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap();
    } else {
        window.maximize().unwrap();
    }
}

#[tauri::command]
fn minimize_window(window: Window) {
    window.minimize().unwrap();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            close_window,
            maximize_window,
            minimize_window,
            get_data
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
