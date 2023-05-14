use std::f64;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct Ohlc{
   pub  o:f64,
   pub  h:f64,
   pub  l:f64,
   pub  c:f64,
}

impl Ohlc{
    pub fn new()->Ohlc{
        Ohlc { o: 0.0_f64, 
               h: 0.0_f64, 
               l: f64::MAX, 
               c: 0.0_f64 }
    }
    pub fn reset(&mut self){
        self.o = 0.0_f64;
        self.h = 0.0_f64;
        self.l = f64::MAX;
        self.c = 0.0_f64;
    }
}
