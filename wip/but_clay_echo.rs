use bevy::prelude::{App, Commands, Entity, IntoSystem, Query, Res};
use bevy::MinimalPlugins;
use bevy_ws_server::{ReceiveError, WsConnection, WsListener, WsPlugin};

fn main() {
    App::build()
        .add_plugins(MinimalPlugins)
        .add_plugin(WsPlugin)
        .add_startup_system(startup.system())
        .add_system(receive_message.system())
        .run();
}

fn startup(listener: Res<WsListener>) {
    // I think this is where the long-lived web3 object goes.
    listener.listen("127.0.0.1:8080");
}

fn receive_message(mut commands: Commands, connections: Query<(Entity, &WsConnection)>) {
    for (entity, conn) in connections.iter() {
        loop {
            match conn.receive() {
                Ok(message) => {
                    conn.send(message);
                }
                Err(ReceiveError::Empty) => break,
                Err(ReceiveError::Closed) => {
                    commands.entity(entity).despawn();
                    break;
                }
            }
        }
    }
}
