use tonic::codegen::Body;
use crate::client::{async_use_client, Chart, ChartSetType, NewFigureReply, Sharp};
use crate::client;
use futures::FutureExt;
use tonic::{Response, Status};
use crate::chart_data;

pub struct Figure{
    name: String,
    sharp: Option<Sharp>,
    charts: Vec<dyn chart_data::Chart>,
}

impl Figure {
   pub fn new(name: &str)->Self{
       Self{
           name: name.to_string(),
           sharp: None,
           charts: vec![],
       }
   }

    pub fn with_sharp(&mut self, width: usize, height: usize)->&mut Self{
        self.sharp = Some(Sharp{ width: width as _, height: height as _});
        let len = width * height;
        self.charts = Vec::with_capacity(len);
        for _ in 0..len {
            self.charts.push(Chart{
                title: "blank".to_string(),
                r#type: ChartSetType::Blank as _,
                chart_set: None,
            })
        }

        self
    }
    pub fn add_chart(&mut self, chart: Chart)-> &mut Self{
        if self.sharp.is_none() {
            self.charts.push(chart);
            return self;
        }


        for i in 0..self.charts.len() {
            if self.charts[i].r#type == ChartSetType::Blank as _ {
                self.charts[i] = chart;
                return self;
            }
        }
        panic!("figure [{}] charts(max: {}) is full", self.name, self.charts.len());
    }


    pub fn set_chart(&mut self, index: usize, chart: Chart)-> &mut Self{
        self.charts[index] = chart;
        self
    }


    pub fn show(&mut self){
        if self.sharp.is_none() {
            self.sharp = Some(Sharp{
                height: self.charts.len() as _,
                width: 1,
            })
        }


        let request = tonic::Request::new(client::Figure{
            id: 0,
            name: self.name.clone(),
            sharp: self.sharp.clone(),
            charts: self.charts.clone(),
        });

        async_use_client(|client| Box::pin(async move{
            match client.new_figure(request).await{
                Ok(_) => {}
                Err(e) => {
                    log::warn!("plot figure err: {:?}", e);
                }
            }
        }));
    }

}