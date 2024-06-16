mod commands;
mod utils;

use commands::{get_app_dir, greet};
use tauri::{
    webview::PageLoadPayload, App, Builder, Webview, WebviewUrl, WebviewWindowBuilder, Window,
    WindowEvent, Wry,
};
use tauri_plugin_log::{Target, TargetKind};
use tracing::{debug, info};
use utils::log_dir;

pub fn app() -> anyhow::Result<Builder<Wry>> {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(logger().build())
        .invoke_handler(tauri::generate_handler![greet, get_app_dir])
        .setup(setup)
        .on_page_load(page_load_handler)
        .on_window_event(window_event_handler);
    Ok(builder)
}

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up the app");

    let handle = app.handle();

    #[cfg(desktop)]
    {
        handle.plugin(tauri_plugin_window_state::Builder::default().build())?;
    }

    let mut builder = WebviewWindowBuilder::new(app, "main", WebviewUrl::default());

    #[cfg(desktop)]
    {
        builder = builder
            .user_agent(&format!("Hn app - {}", std::env::consts::OS))
            .title("Hacker News")
            .inner_size(1200., 800.)
            .min_inner_size(800., 600.)
            .resizable(true)
            .content_protected(true);
    }

    let webiview = builder.build()?;

    #[cfg(debug_assertions)]
    webiview.open_devtools();

    // initialize updater..

    Ok(())
}

// on_page_load is called when the webview is created.
fn page_load_handler(webview: &Webview, _payload: &PageLoadPayload<'_>) {
    info!("Page loaded on {:?}", webview.label());
}

// signature: Fn(&Window<R>, &WindowEvent)
fn window_event_handler(window: &Window, event: &WindowEvent) {
    debug!("Window event {:?} on {:?}", event, window.label());

    if let WindowEvent::CloseRequested { api, .. } = event {
        info!("Close requested on {:?}", window.label());
        if window.label() == "main" {
            api.prevent_close();
            window.hide().unwrap()
        }
    }
}

fn logger() -> tauri_plugin_log::Builder {
    tauri_plugin_log::Builder::default()
        .targets([
            Target::new(TargetKind::Webview),
            Target::new(TargetKind::Folder {
                path: log_dir(),
                file_name: None,
            }),
            Target::new(TargetKind::Stdout),
        ])
        .level(tracing::log::LevelFilter::Info)
}
