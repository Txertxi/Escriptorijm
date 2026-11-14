// Evita la finestra de consola a Windows en la versió final.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use tauri::Manager;
use tauri_plugin_autostart::{MacosLauncher, ManagerExt};
use tauri_plugin_window_state::{AppHandleExt, Builder as WindowState, StateFlags};

fn main() {
    tauri::Builder::default()
        // Si ja n'hi ha una còpia oberta (per exemple, l'autoinici + doble clic), no n'obre una altra.
        .plugin(tauri_plugin_single_instance::init(|_app, _args, _cwd| {}))
        // Autoinici amb Windows: escriu a HKCU\Software\Microsoft\Windows\CurrentVersion\Run.
        .plugin(tauri_plugin_autostart::init(MacosLauncher::LaunchAgent, None))
        // Només recorda la POSICIÓ: la mida l'ajusta la pàgina al contingut.
        .plugin(
            WindowState::default()
                .with_state_flags(StateFlags::POSITION)
                .build(),
        )
        .setup(|app| {
            let autostart = app.autolaunch();
            let dir = app.path().app_config_dir()?;
            let _ = fs::create_dir_all(&dir);
            let marker = dir.join("autostart-init");

            if !marker.exists() {
                // Primera execució: s'activa l'autoinici sol (sense drecera a shell:startup).
                let _ = autostart.enable();
                let _ = fs::write(&marker, "1");
            } else if autostart.is_enabled().unwrap_or(false) {
                // Si l'usuari ho té activat, es reescriu l'entrada perquè apunti
                // sempre a la ruta actual de l'.exe (per si l'ha mogut).
                let _ = autostart.enable();
            }
            // Si l'usuari ho ha desactivat a Configuració, no es torna a activar.
            Ok(())
        })
        // Desa la posició cada cop que es mou la finestra, així no es perd
        // si Windows tanca el procés en apagar l'equip.
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Moved(_) = event {
                let _ = window.app_handle().save_window_state(StateFlags::POSITION);
            }
        })
        .run(tauri::generate_context!())
        .expect("error en executar el giny");
}
