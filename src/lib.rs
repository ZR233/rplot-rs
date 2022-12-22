extern crate core;
mod chart_data;
mod client;
mod figure;


use std::borrow::Borrow;
use futures::FutureExt;
use crate::client::*;
use crate::client::plot_client::PlotClient;

mod prelude{
    use crate::{client, figure};
    pub use figure::Figure;
    pub use client::Chart;
    pub use client::clear;

}



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


    #[test]
    fn test_figure() {
        use super::prelude::*;

        Figure::new("test_figure").show();


        assert_eq!(4, 4);
    }

    #[test]
    fn test_clear() {
        use super::prelude::*;

        clear();
        assert_eq!(4, 4);
    }
}
