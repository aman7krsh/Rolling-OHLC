use crate::Ohlc;
use super::{Stream, ceil_binary_search,SegmentTree};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RollingOhlc{
    pub stream:Stream,
    pub ohlc:Ohlc,
    pub window:u64,
    pub seg_tree:SegmentTree,
}

impl RollingOhlc{
    pub fn new(w:u64)->RollingOhlc{
        RollingOhlc { stream: Stream::new(), 
                      ohlc: Ohlc::new(), 
                      window:w,
                      seg_tree : SegmentTree::new(), 
                    }
    }

    pub fn rolling_olhc(&mut self, value:f64, cur_time:u64)
    {
        self.ohlc.reset();
        self.stream.add_new_price(value, cur_time);
        
        let recent_idx: usize = self.stream.prices.len() - 1;  
        let t_recent = self.stream.timestamps[recent_idx];

        let window_epoch_time = self.window*(60*1000); //Converting 'T' size window to linux epochs
        let t_start = t_recent - window_epoch_time + 1; //Expected start time in 'T' size window

        //Using Binary Search finding Ceil of start time, Time Complexity: O(log(n))
        let tstart_idx = ceil_binary_search(&t_start, &self.stream.timestamps).unwrap();

        //Using Dynamically updating Segment Tree
        //Updating Segment Tree nodes as new price arrives, Time Complexity: O(log(n))
        self.seg_tree.update_tree_node(recent_idx, value);

        //Querying min & max price in 'T' size window (i.e b/w  tstart_idx & recent_idx)
        //Query operation: Time Complexity: O(log(n))
        let (low,high) = self.seg_tree.query(tstart_idx, recent_idx + 1);
        self.ohlc.l = low;
        self.ohlc.h = high;
        self.ohlc.o = self.stream.prices[tstart_idx];
        self.ohlc.c = value;


        //Algorithm: Binary Search + Dynamic Segment tree 
        //Total Time Complexity:
        //Finding tstart_idx using Ceil Binary Search: O(log(n))
        //Updating new price in Segment tree: O(log(n))
        //Querying min/max in 'T' size window: O(log(n))
        //Total time complexity: O(log(n)) for single query of OHLC at arrival of (new_price, new_timestamp)
        //Total time complexity: O(Q*log(n)) for 'Q' queries
        
    }
}