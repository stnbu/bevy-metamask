use async_channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;

pub struct Eip1193Plugin;
impl Plugin for Eip1193Plugin {
    fn build(&self, app: &mut App) {
        let task_pool = IoTaskPool(app.world.resource::<IoTaskPool>().0.clone());
        let (eip1193_tx, eip1193_rx) = unbounded();
        app.insert_resource(Eip1193Listener::new(task_pool, eip1193_tx))
            .insert_resource(Eip1193AcceptQueue { eip1193_rx });
    }
}

pub struct Eip1193Listener {
    task_pool: IoTaskPool,
    eip1193_tx: Sender<serde_json::value::Value>,
}

pub struct Eip1193AcceptQueue {
    eip1193_rx: Receiver<serde_json::value::Value>,
}

impl Eip1193Listener {
    pub fn new(task_pool: IoTaskPool, eip1193_tx: Sender<serde_json::value::Value>) -> Self {
        Self {
            task_pool,
            eip1193_tx,
        }
    }
}
