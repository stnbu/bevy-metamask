use async_channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task};
use web3::transports::eip_1193;

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

impl Eip1193Listener {
    pub fn new(task_pool: IoTaskPool, eip1193_tx: Sender<serde_json::value::Value>) -> Self {
        Self {
            task_pool,
            eip1193_tx,
        }
    }

    ////////

    pub fn _dodo(&self) {
        // let listener = futures::executor::block_on(TcpListener::bind(bind_to))
        //     .expect("cannot bind to the address");

        // the transport = ...

        let task_pool = self.task_pool.clone();
        let eip1193_tx = self.eip1193_tx.clone();

        let provider = eip_1193::Provider::default().unwrap().unwrap();
        let transport = eip_1193::Eip1193::new(provider);

        // let addrs = transport
        //     .execute("eth_requestAccounts", vec![])
        //     .await
        //     .unwrap();

        let task = self.task_pool.spawn(async move {
            // we have a transport. the only thing to do is relay along the
            // responses to Eip1193AcceptQueue ....??
            loop {
                format!("{:?}{:?}{:?}", task_pool, eip1193_tx, transport);
                /////
            }
        });
        task.detach();
    }
}

pub struct Eip1193AcceptQueue {
    eip1193_rx: Receiver<serde_json::value::Value>,
}

impl Eip1193AcceptQueue {
    pub fn new(eip1193_rx: Receiver<serde_json::value::Value>) -> Self {
        Self { eip1193_rx }
    }
}

////

#[derive(Component)]
pub struct Eip1193Connection {
    _io: Task<()>,
    sender: async_channel::Sender<String>,
    receiver: async_channel::Receiver<String>,
}

pub use async_channel::TryRecvError as ReceiveError;
impl Eip1193Connection {
    pub fn send(&self, message: String) -> bool {
        self.sender.try_send(message).is_ok()
    }

    pub fn receive(&self) -> Result<String, ReceiveError> {
        self.receiver.try_recv()
    }
}
