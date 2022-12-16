use bevy::prelude::*;
use client_websocket::connect;
use client_websocket::ClientName;
use futures::future::join_all;
use futures::future::ready;
use futures::prelude::*;
use lazy_static::lazy_static;
use nats_lite::nats;
use protocol::handle_client_op;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use client_websocket::Client;
const BUF_CAPACITY: usize = 128 * 1024;

use crate::*;
use js_sys::Array;
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::Mutex;
use tracing::error;
use wasm_bindgen_futures::spawn_local;
#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_namespace = window, js_name = game_server)]
    fn game_server() -> String;
    #[wasm_bindgen(js_namespace = window, js_name = web_bevy_events_fn)]
    fn web_bevy_events_fn() -> Array;
}

lazy_static! {
    static ref CLIENTS: Mutex<HashMap<ClientName, BoxClient2>> = Mutex::new(HashMap::new());
    static ref CLIENTS_TO_CONNECT: Mutex<HashMap<ClientName,(String,nats::ConnectInfo)>> =
    //Mutex::new([(ClientName(Cow::Borrowed("default")),(String::from("wss://localhost:9223/"),
    //Mutex::new([(ClientName(Cow::Borrowed("default")),(format!("ws://52.221.222.250:9223/"),
    //Mutex::new([(ClientName(Cow::Borrowed("default")),(format!("ws://{}:7083/", window().location().host().unwrap().split(":").collect::<Vec<&str>>().get(0).unwrap()),
    Mutex::new([(ClientName(Cow::Borrowed("default")),(game_server(),
    nats::ConnectInfo{
      verbose:false,
      pedantic:false,
      user_jwt:None,
      nkey:None,
      signature:None,
      user:Some(nats::SecureString::from(String::from("sample-user2"))),
      pass:Some(nats::SecureString::from(String::from("secret"))),
      name:None,
      //echo:true,
      echo:false,
      auth_token:None,
      headers:true,
      lang:String::from("nats.ws"),
      tls_required:false,
      version:String::from("1.1.0"),
    }
  ))].iter().cloned().collect());
}

pub fn connect_websocket() {
    //let servers=vec![String::from("wss://localhost:9222/")];
    info!("connect_websocketing");
    let servers = CLIENTS_TO_CONNECT.lock().unwrap();
    let future_arr = servers
        .iter()
        .map(|(c, s)| local_connect(c.clone(), s.clone()));
    let join_ = join_all(future_arr).then(|_l| ready(()));
    spawn_local(join_);
    info!("after connect_websocketing");
}
async fn local_connect(c: ClientName, s: (String, nats::ConnectInfo)) -> () {
    connect(c.clone(), s.0.clone())
        .then(|cz| {
            //let s_clone = s.clone();
            ready(
                cz.map(|(client, mut meta)| {
                    let c_clone = c.clone();
                    let mut tx = client.sender();

                    spawn_local(async move {
                        let c = nats::proto::ClientOp::Connect(s.1.clone());
                        let c = handle_client_op(c).unwrap();
                        tx.send(c).await.unwrap_or_else(|err| {
                            info!("err{}", err);
                        });

                        if let Some(m) = meta.next().await {
                            info!("close{:?}", m);
                            delay(1000).await;
                            local_connect(c_clone, s.clone()).await;
                        }
                    });
                    CLIENTS
                        .lock()
                        .unwrap()
                        .insert(c, std::boxed::Box::new(client));
                })
                .unwrap_or_else(|err| {
                    // spawn_local( async move{
                    //   delay(3000).await;
                    //   local_connect(c_clone,s_clone).await;
                    // });
                    error!("{}", err)
                }),
            )
        })
        .await
}
pub fn set_client(mut client_res: ResMut<Option<BoxClient>>) {
    let mut map = CLIENTS.lock().unwrap();
    for (_k, v) in map.drain() {
        if let Some(ref mut c) = *client_res {
            c.clients.push(v);
        } else {
            let mut bc = BoxClient::default();
            bc.clients = vec![v];
            *client_res = Some(bc);
        }
    }
    if let Some(ref mut _c) = *client_res {}
}

pub fn block_on<T>(future: impl Future<Output = T> + 'static) {
    wasm_bindgen_futures::spawn_local(async { future.map(|_| ()).await });
}
pub fn get_random_int(min: i32, max: i32) -> usize {
    ((js_sys::Math::floor(js_sys::Math::random()) as i32) * (max - min) + min) as usize
}
pub async fn delay(timeout_ms: i32) -> () {
    let p = js_sys::Promise::new(&mut |resolve, _| {
        let closure = Closure::wrap(Box::new(move || {
            //resolve(&42.into())
            resolve.call0(&JsValue::NULL).unwrap();
        }) as Box<dyn FnMut()>);

        set_timeout(&closure, timeout_ms);
        closure.forget();
    });
    wasm_bindgen_futures::JsFuture::from(p)
        .into_future()
        .await
        .unwrap();
    ()
}
fn set_timeout(f: &Closure<dyn FnMut()>, timeout_ms: i32) {
    window()
        .set_timeout_with_callback_and_timeout_and_arguments_0(
            f.as_ref().unchecked_ref(),
            timeout_ms,
        )
        .expect("should register `requestAnimationFrame` OK");
}
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}
