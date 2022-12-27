extern crate core;
mod chart_data;
mod client;
mod figure;


use std::borrow::Borrow;
use futures::FutureExt;
use crate::client::*;
use crate::client::plot_client::PlotClient;

mod prelude{
    use crate::{client, figure, chart_data};
    pub use figure::Figure;
    pub use chart_data::Chart;
    pub use chart_data::Chart::Liner;
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

        let data1: Vec<Data2D> = (0..10000).into_iter().map(|i|{
            Data2D{x: i as _, y: (i as f64/2.0) as _}
        }).collect();
        let data2: Vec<Data2D> = (0..10000).into_iter().map(|i|{
            Data2D{x: i as _, y: i as _}
        }).collect();


        Figure::new("test_figure2")
            // .with_sharp(2, 2)
            .add_chart(Liner{title: "t1".to_string(), data_set: vec![
                LinerData{
                    name: "line1".to_string(),
                    // color: 0xFFEF9A9A,
                    color: 0,
                    // data: vec![Data2D{x:0.0, y: 1.0}, Data2D{x:1.0, y: 2.0}, Data2D{x:4.0, y: 3.0}],
                    data: data1
                },
                LinerData{
                    name: "line2".to_string(),
                    color: 0,
                    data: data2,
                }
            ] })
            // .add_chart(Liner{title: "t2".to_string(), data_set: vec![] })
            // .add_chart(Liner{title: "t3".to_string(), data_set: vec![] })
            // .add_chart(Liner{title: "t4".to_string(), data_set: vec![] })
            .show();


        assert_eq!(4, 4);
    }

    #[test]
    fn test_clear() {
        use super::prelude::*;

        clear();
        assert_eq!(4, 4);
    }
}
