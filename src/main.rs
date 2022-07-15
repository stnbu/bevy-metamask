use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
use mbutils;
use metamask_bevy as metamask;

fn main() {
    let mut app = App::new();

    #[cfg(target_arch = "wasm32")]
    app.add_system(handle_browser_resize);

    app.add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(startup)
        .add_plugin(metamask::task::Eip1193Plugin)
        .add_system(ui_example)
        .run();
}

fn startup(task: Res<metamask::task::Eip1193Task>) {
    task.spawn();
}

fn ui_example(
    mut egui_context: ResMut<EguiContext>,
    interface: Res<metamask::task::Eip1193Interface>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        let sender = interface.sender.clone();
        let receiver = interface.receiver.clone();
        if ui.button("metamask").clicked() {
            mbutils::console_log!("Button was clicked.");
            wasm_bindgen_futures::spawn_local(async move {
                match sender.try_send("eth_requestAccounts".to_string()) {
                    Ok(()) => {
                        mbutils::console_log!("Sent to interface sender.");
                    }
                    Err(err) => {
                        mbutils::console_log!("Not sent to interface sender: {}", err);
                    }
                };
            });
        }
        match receiver.try_recv() {
            Ok(message) => {
                let message = message.clone();
                mbutils::console_log!("A message received from interface: {}", message.to_string());
                //ui.label(message.to_string());
            }
            Err(err) => {
                mbutils::console_log!("Failed to receive from interface: {}", err);
            }
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn handle_browser_resize(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let wasm_window = web_sys::window().unwrap();
    let (target_width, target_height) = (
        wasm_window.inner_width().unwrap().as_f64().unwrap() as f32,
        wasm_window.inner_height().unwrap().as_f64().unwrap() as f32,
    );

    if window.width() != target_width || window.height() != target_height {
        window.set_resolution(target_width, target_height);
    }
}
