use async_channel::{bounded, Receiver, Sender};
use bevy::prelude::*;
use bevy::tasks::IoTaskPool;
use web3::transports::eip_1193;
use web3::types::H160;

#[macro_use]
mod console;

mod task;

pub struct MetaMaskPlugin;
impl Plugin for MetaMaskPlugin {
    fn build(&self, app: &mut App) {
        // Belongs here, probably. Compiles, definitely.
        let _task_pool = app.world.resource::<IoTaskPool>().0.clone();

        /*
        let (ws_tx, ws_rx) = crossbeam_channel::unbounded();
        app.insert_resource(WsListener::new(task_pool, ws_tx))
            .insert_resource(WsAcceptQueue { ws_rx })
            .add_system(accept_ws_from_queue.system());
                */

        app.add_startup_system(setup_comm)
            .add_state(AppState::Ready)
            .add_system_set(
                SystemSet::on_update(AppState::LoadingAddr).with_system(addr_response_system),
            );
    }
}

pub struct MetamaskChannel {
    // Um, we want this structure...but where
    //web3: Option<web3::Web3<eip_1193::Eip1193>>,
    addr_rx: Receiver<H160>,
    pub addr_tx: Sender<H160>,
}

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    LoadingAddr,
    Ready,
}

#[derive(Default)]
pub struct AppData {
    pub user_wallet_addr: Option<H160>,
}

fn setup_comm(mut commands: Commands) {
    let (addr_tx, addr_rx) = bounded(1);
    commands.insert_resource(MetamaskChannel { addr_rx, addr_tx });

    let provider = eip_1193::Provider::default().unwrap();
    if let Some(_p) = provider {
        debug!("{:?}", _p);
        commands.insert_resource(AppData::default());
    }
}

// web_sys::console::log_1(&"Hello, world!".into());

pub async fn request_account(addr_tx: &Sender<H160>) {
    let provider = eip_1193::Provider::default().unwrap().unwrap();
    let transport = eip_1193::Eip1193::new(provider);
    //let web3 = web3::Web3::new(transport);

    // serde_json::value::Value
    // transport.execute("eth_requestAccounts", vec![]) ???
    //let addrs = web3.eth().request_accounts().await.unwrap();
    use web3::Transport;
    let addrs = transport
        .execute("eth_requestAccounts", vec![])
        .await
        .unwrap();

    match addrs {
        serde_json::value::Value::Array(x) => {
            console::console_log!("addrs: {:?}", x);
            //let a = x.as_array().unwrap();

            //let aagh = format!("{:?}", x);
            //web_sys::console::log_1(aagh);

            // let foo: String = format!("{:?}", x).to_string();
            // let bar = foo.to_owned();
            // web_sys::console::log_1(bar.into());
        }
        _ => {
            // let wat = &"Hello, world2!".into();
            // web_sys::console::log_1(wat);
            // debug!("Oops was: {:?}", addrs)
        }
    }
    //Vec<Address>
    addr_tx.send(web3::types::Address::default()).await.unwrap();
    // if !addrs.is_empty() {
    //     addr_tx.send(addrs[0]).await.unwrap();
    // }
}

fn addr_response_system(
    metamask_ch: ResMut<MetamaskChannel>,
    mut app_data: ResMut<AppData>,
    mut app_state: ResMut<State<AppState>>,
) {
    if let Ok(addr) = metamask_ch.addr_rx.try_recv() {
        app_data.user_wallet_addr = Some(addr);
        app_state.set(AppState::Ready).unwrap();
    }
}
