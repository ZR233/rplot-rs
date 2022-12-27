
tonic::include_proto!("plot");
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use lazy_static::lazy_static;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tonic::transport::{Channel};
use crate::client::plot_client::PlotClient;
use futures::FutureExt;

lazy_static! {
    pub static ref RT: Runtime = {
        tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build().unwrap()
    };
    static ref DST: String = init_gprc_dst();
    pub static ref CLIENT: Arc<Mutex<Option<PlotClient<Channel>>>> = Arc::new(Mutex::new(None)) ;
}
fn init_gprc_dst()->String{
    let host = "localhost:50051";
    format!("http://{}", host)
}


fn init_client()-> Option<PlotClient<Channel>>{
    let client = RT.block_on(async{
        let client = PlotClient::connect(DST.as_str()).await;

         match client {
             Ok(c) => {Some(c)}
             Err(e) => {
                 log::warn!("{} connect err: {:?}", DST.as_str(), e);
                 None
             }
         }
    });
    client
}

pub fn async_use_client<F>(func:  F)
where for<'a> F:  FnOnce(&'a mut PlotClient<Channel>)-> Pin<Box<dyn Future<Output = () > +'a>>
{

    RT.block_on( async move{
        let mut c = CLIENT.lock().await;

        if c.is_none() {
            let client = PlotClient::connect(DST.as_str()).await;
            match client {
                Ok(new_c) => {
                    *c = Some(new_c);
                }
                Err(e) => {
                    log::warn!("{} connect err: {:?}", DST.as_str(), e);
                    return;
                }
            }
        }

        let client = match c.as_mut() {
            None => {
                return;
            }
            Some(c) => {
                c
            }
        };

        func(client).await;
    });
}

pub fn clear(){
    async_use_client(|client|async{
        let request = tonic::Request::new(Empty{});
        match client.clear(request).await{
            Ok(_) => {}
            Err(e) => {
                log::warn!("clear err: {:?}", e);
            }
        };
    }.boxed());


}