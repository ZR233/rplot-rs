use crate::client::{async_use_client, ChartSetLiner, ChartSetType, Sharp};
use crate::client;
use prost::Message;
use prost_types::Any;
use crate::prelude::Chart;

pub struct Figure{
    name: String,
    sharp: Option<Sharp>,
    charts: Vec<Chart>,
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
            self.charts.push(Chart::UnKnown);
        }

        self
    }
    pub fn add_chart(&mut self, chart: Chart)-> &mut Self{
        if self.sharp.is_none() {
            self.charts.push(chart);
            return self;
        }


        for i in 0..self.charts.len() {
            match self.charts[i] {
                Chart::UnKnown => {
                    self.charts[i] = chart;
                    return self;
                }
                _ => {}
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
        let mut charts = vec![];

        for chart in &self.charts {
            let mut pb_chart = client::Chart::default();
            pb_chart.r#type = ChartSetType::Blank as _;
            match chart {
                Chart::UnKnown => {}
                Chart::Blank => {}
                Chart::Liner { title, data_set } => {

                    pb_chart.title = title.to_string();
                    pb_chart.r#type = ChartSetType::Liner as _;
                    pb_chart.chart_set = Some(
                        Any{
                            type_url: "com.zr.rplot/plot.ChartSetLiner".to_string(),
                            value: ChartSetLiner{ data_set: data_set.clone() }.encode_to_vec(),
                        }
                    );
                }
            }

            charts.push(pb_chart);
        }


        let request = tonic::Request::new(client::Figure{
            id: 0,
            name: self.name.clone(),
            sharp: self.sharp.clone(),
            charts,
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