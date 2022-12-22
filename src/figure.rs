use crate::client::{Chart, ChartSetType, Sharp};

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
            self.charts.push(Chart{
                title: "blank".to_string(),
                r#type: ChartSetType::Blank as _,
                chart_set: None,
            })
        }

        self
    }
    pub fn add_chart(&mut self, chart: Chart)-> &mut Self{
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




    pub fn show(&self)->Result<(), std::io::Error>{




        Ok(())
    }

}