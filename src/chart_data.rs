use crate::client::LinerData;


pub enum Chart{
    UnKnown,
    Blank,
    Liner{
        title: String,
        data_set: Vec<LinerData>
    }
}