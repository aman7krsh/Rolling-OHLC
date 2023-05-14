#[derive(Debug, PartialEq, PartialOrd)]
pub struct Stream{
    pub prices:Vec<f64>,
    pub timestamps:Vec<u64>,
}

impl Stream{
    pub fn new()->Stream{
        Stream { 
            prices: Vec::new(), 
            timestamps: Vec::new() 
        }
    }

    pub fn add_new_price(&mut self, value:f64, time:u64){
        self.prices.push(value);
        self.timestamps.push(time);
    }
}