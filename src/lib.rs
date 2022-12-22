extern crate core;

mod client;
mod figure;

use std::borrow::Borrow;
use futures::FutureExt;
use crate::client::*;
use crate::client::plot_client::PlotClient;

pub fn add(left: usize, right: usize) -> usize {

    async_use_client(| c|async{

        let request = tonic::Request::new(Figure{
            id: 0,
            name: "test".to_string(),
            sharp: Option::from(Sharp { width: 1, height: 1 }),
            charts: vec![],
        });


        let r = c.new_figure(request).await.unwrap();

        println!("RESPONSE={:?}",  r);

    }.boxed());

    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
