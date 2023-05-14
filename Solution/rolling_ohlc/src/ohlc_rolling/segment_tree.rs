// Assuming ticked symbol arrives at every 1 ms
// Total ticked symbol in 1 min = 1*60*1000 => 60,000
// Assuming 7 hours of trading time in a day
// So,Total ticked symbol in 7 hrs ==> 7*60*60000 = 2,52,00,000
// So to support single day trading we can safely assume 
// approx 3,00,00,000 vector size to hold prices & timestamps


const N:usize = 3_00_00_000;
const VEC_SIZE: usize = N << 1;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SegmentTree{
    pub min_segtree:Vec<f64>,  
    pub max_segtree:Vec<f64>,
}

impl SegmentTree{
    pub fn new()-> Self{
        SegmentTree{
            min_segtree:vec![0.0;VEC_SIZE],
            max_segtree:vec![0.0;VEC_SIZE],
        }      
    }
    
    pub fn update_tree_node(&mut self, mut index:usize, value:f64){
        index = N + index;

        // set value at position 'index'
        self.min_segtree[index] = value;
        self.max_segtree[index] = value;
           
        // move upward and update parents
        while index>1{
            self.min_segtree[index >> 1] = f64::min(self.min_segtree[index] , self.min_segtree[index^1]);
            self.max_segtree[index >> 1] = f64::max(self.max_segtree[index] , self.max_segtree[index^1]);
            index >>= 1;
        }
    }  

    pub fn query(&mut self, mut left:usize, mut right:usize)->(f64,f64){
        let mut res_min = f64::MAX;
        let mut res_max = f64::MIN;
        
        left += N;
        right += N;

        while left < right {
            if left & 1 == 1{
                res_min = f64::min(res_min , self.min_segtree[left]);
                res_max = f64::max(res_max , self.max_segtree[left]);
                left += 1;
            }
            if right & 1 == 1{
                right -= 1;
                res_min = f64::min(res_min , self.min_segtree[right]);
                res_max = f64::max(res_max , self.max_segtree[right]);
            }
            left >>= 1;
            right >>= 1;
        }
        return (res_min, res_max);
    }   
}