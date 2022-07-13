use async_channel::{unbounded, Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::{IoTaskPool, Task};
//use futures::select;
//use web3::transports::eip_1193;
use web3::transports::eip_1193;

pub struct Eip1193Plugin;
impl Plugin for Eip1193Plugin {
    fn build(&self, app: &mut App) {
        let task_pool = IoTaskPool(app.world.resource::<IoTaskPool>().0.clone());
        let (task_send, interface_receive) = unbounded();
        let (interface_send, task_receive) = unbounded();
        app.insert_resource(Eip1193Task::new(task_pool, task_send, task_receive))
            .insert_resource(Eip1193Interface::new(interface_send, interface_receive));
    }
}

pub struct Eip1193Task {
    task_pool: IoTaskPool,
    sender: Sender<String>,
    receiver: Receiver<String>,
}

pub struct Eip1193Interface {
    sender: Sender<String>,
    receiver: Receiver<String>,
}

pub use async_channel::TryRecvError as ReceiveError;
impl Eip1193Interface {
    pub fn new(sender: Sender<String>, receiver: Receiver<String>) -> Self {
        Self { sender, receiver }
    }

    pub fn send(&self, message: String) -> bool {
        self.sender.try_send(message).is_ok()
    }

    pub fn receive(&self) -> Result<String, ReceiveError> {
        self.receiver.try_recv()
    }
}

impl Eip1193Task {
    pub fn new(task_pool: IoTaskPool, sender: Sender<String>, receiver: Receiver<String>) -> Self {
        Self {
            task_pool,
            sender,
            receiver,
        }
    }

    pub fn spawn(self) {
        let provider = eip_1193::Provider::default().unwrap().unwrap();
        use web3::Transport;
        let transport = eip_1193::Eip1193::new(provider);

        let task = self.task_pool.spawn(async move {
            if let Ok(message) = self.receiver.try_recv() {
                if let Ok(message) = transport.execute(&message, vec![]).await {
                    self.sender.try_send(message.to_string());
                }
            }
        });
        task.detach();
    }
}

////////
/*
    pub fn _dodo(&mut self) {
        // let listener = futures::executor::block_on(TcpListener::bind(bind_to))
        //     .expect("cannot bind to the address");

        // the transport = ...

        let task_pool = self.task_pool.clone();
        //let eip1193_tx = self.eip1193_tx.clone();

        let provider = eip_1193::Provider::default().unwrap().unwrap();
        use web3::Transport;
        let transport = eip_1193::Eip1193::new(provider);

        // let addrs = transport
        //     .execute("eth_requestAccounts", vec![])
        //     .await
        //     .unwrap();
        // let (eip1193_, io_message_rx) = async_channel::unbounded::<String>();
        // let (io_message_tx, receiver) = async_channel::unbounded::<String>();

    self.eip1193_tx.


          let task = self.task_pool.spawn(async move {
            let mut from_api = io_message_rx.recv(); //.fuse();
                                                     //let mut from_metamask = .recv().fuse();
                                                     // we have a transport. the only thing to do is relay along the
                                                     // responses to Eip1193AcceptQueue ....??
            loop {
                if let Ok(message) = from_api.await {
                    let foo = transport.execute(&message, vec![]).await.unwrap();
                    let msg = format!("{:?}", foo);
                    io_message_tx.send(msg);
                };
                //format!("{:?}{:?}{:?}", task_pool, eip1193_tx, transport);
                /////
            }
        });
        task.detach();
        self.insert_resource(Eip1193Connection {
            sender: sender,
            receiver: receiver,
        });
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

pub struct Eip1193Connection {
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

*/
