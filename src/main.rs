use bevy::prelude::*;
use bevy_egui::{egui, EguiContext, EguiPlugin};
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
        // self.sender.try_send(message).is_ok()
        // self.receiver.try_recv()

        let sender = interface.sender.clone();
        let receiver = interface.receiver.clone();
        if ui.button("metamask").clicked() {
            wasm_bindgen_futures::spawn_local(async move {
                let _ = sender.try_send("eth_requestAccounts".to_string()).is_ok();
            });
        }
        if let Ok(message) = receiver.try_recv() {
            let message = message.clone();
            ui.label(message.to_string());
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
