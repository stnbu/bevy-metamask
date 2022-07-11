//use async_net::{AsyncToSocketAddrs, TcpListener, TcpStream};

use bevy::prelude::{Commands, IntoSystem, Plugin, Res, ResMut};
use bevy::tasks::{IoTaskPool, Task};

// crossbeam "...is an alternative to std::sync::mpsc with more features and better performance."
use crossbeam_channel::{unbounded, Receiver, Sender};
//use async_channel::{unbounded, Receiver, Sender};

use futures::{select, FutureExt, SinkExt, StreamExt};

pub struct Eip1193Plugin;

use bevy::prelude::*;

impl Plugin for Eip1193Plugin {
    fn build(&self, app: &mut App) {
        let task_pool: IoTaskPool =
            bevy::tasks::IoTaskPool(app.world.resource::<IoTaskPool>().0.clone());
        // let task_pool = app
        //     .world()
        //     .get_resource::<IoTaskPool>()
        //     .expect("IoTaskPool not found")
        //     .clone();
        // This is the same as e.g.
        //     let (addr_tx, addr_rx) = bounded(1);
        // `bounded` returns `Sender` and `Receiver`
        // Q: Why `unbounded` vs `bounded`?
        let (ws_tx, ws_rx) = unbounded();
        app.insert_resource(Eip1193Listener::new(task_pool, ws_tx))
            .insert_resource(Eip1193AcceptQueue { ws_rx })
            .add_system(accept_ws_from_queue.system());
    }
}

pub struct Eip1193Listener {
    // Look at eip_1193.rs and picture what goes here...
    task_pool: IoTaskPool,
    // One wonders what this needs to be in bevy-metamask!! Maybe Sender<Eip1193> ...?
    ws_tx: Sender<String>,
}

pub struct Eip1193AcceptQueue {
    ws_rx: Receiver<String>,
}

impl Eip1193Listener {
    pub fn new(task_pool: IoTaskPool, ws_tx: Sender<String>) -> Self {
        Self { task_pool, ws_tx }
    }

    /*use async_channel::{bounded, Receiver, Sender};
        let (ws_tx, ws_rx) = crossbeam_channel::unbounded();
    let (addr_tx, addr_rx) = bounded(1);*/

    /*
    pub fn connect(&self) {
        let provider = eip_1193::Provider::default().unwrap().unwrap();
        let transport = eip_1193::Eip1193::new(provider);
        let web3 = web3::Web3::new(transport);

        let task_pool = self.task_pool.clone();
        let ws_tx = self.ws_tx.clone();
        let task = self.task_pool.spawn(async move {
            loop {
                match listener.accept().await {
                    Ok((stream, addr)) => {
                        // log::debug!("new connection from {}", addr);
                        let ws_tx = ws_tx.clone();
                        let accept = async move {
                            //connect up here?
                        };
                        task_pool.spawn(accept).detach();
                    }
                    Err(e) => {
                        // log::error!("error accepting a new connection: {}", e);
                    }
                }
            }
        });

        task.detach();
    }*/
}

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

pub fn accept_ws_from_queue(
    mut commands: Commands,
    pool: Res<IoTaskPool>,
    queue: ResMut<Eip1193AcceptQueue>,
) {
    for mut websocket in queue.ws_rx.try_iter() {
        let (message_tx, io_message_rx) = async_channel::unbounded::<String>();
        let (io_message_tx, message_rx) = async_channel::unbounded::<String>();

        let io = pool.spawn(async move {
            loop {
                let mut from_channel = io_message_rx.recv().fuse();
                let mut from_ws = websocket;
                select! {
                    message = from_channel => if let Ok(message) = message {
                        let _ =  websocket.send(message).await;
                    } else {
                        break;
                    },
                    message = from_ws => if let Some(Ok(message)) = message {
                        let _ = io_message_tx.send(message).await;
                    } else {
                        break;
                    },
                    complete => break,
                }
            }
        });
        commands.spawn().insert(Eip1193Connection {
            _io: io,
            sender: message_tx,
            receiver: message_rx,
        });
    }
}
