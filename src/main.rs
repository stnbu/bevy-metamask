use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
#[macro_use]
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
    mut interface: ResMut<metamask::task::Eip1193Interface>,
) {
    egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
        let sender = interface.sender.clone();
        let receiver = interface.receiver.clone();
        if ui.button("metamask").clicked() {
            mbutils::console_log!("foo2");
            wasm_bindgen_futures::spawn_local(async move {
                mbutils::console_log!("about to send");
                match sender.try_send("eth_requestAccounts".to_string()) {
                    Ok(()) => {
                        mbutils::console_log!("xxx yay, sent");
                    }
                    Err(err) => {
                        mbutils::console_log!("xxx boo, not sent: {:?}", err);
                    }
                };
            });
        }
        match receiver.try_recv() {
            Ok(message) => {
                mbutils::console_log!("xxxx yay, received");
                let message = message.clone();
                ui.label(message.to_string());
            }
            Err(err) => {
                mbutils::console_log!("xxxx boo, not received: {:?}", err);
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
