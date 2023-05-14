extern crate rolling_ohlc;
use std::collections::HashMap;
use rolling_ohlc::ohlc_rolling::rolling::RollingOhlc;


pub fn compute_ohlc(data:&HashMap<String, Vec<String>>, time_window:u64)->HashMap<&str, Vec<String>>{
    let asks_string = data.get(&"a".to_string()).unwrap();
    let bids_string = data.get(&"b".to_string()).unwrap();
    let t_string = data.get(&"T".to_string()).unwrap();
    let symbols = data.get(&"s".to_string()).unwrap();
    let asks: Vec<f64> = asks_string.iter().map(|x| x.parse::<f64>().unwrap()).collect();
    let bids: Vec<f64> = bids_string.iter().map(|x| x.parse::<f64>().unwrap()).collect();
    let t_stmps: Vec<u64> = t_string.iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let prices: Vec<f64> = asks.iter().zip(bids.iter()).map(|(&a, &b)| (a+b)/2.0).collect();
    
    //Hashmap "symbol_rohlc" will store RollingOhlc instance for each symbol
    let mut symbol_rohlc: HashMap<String, RollingOhlc> = HashMap::new();
    
    //Hashmap output will store desired o/p
    let mut output: HashMap<&str, Vec<String>> = HashMap::new();
    output.insert("symbol", Vec::new());
    output.insert("timestamp", Vec::new());
    output.insert("open", Vec::new());
    output.insert("high", Vec::new());
    output.insert("low", Vec::new());
    output.insert("close", Vec::new());
    
    let total_ticked_data = prices.len();
    for i in 0..total_ticked_data{
        let roll_ohlc = symbol_rohlc.entry(symbols[i].clone())
                                                      .or_insert(RollingOhlc::new(time_window));
        roll_ohlc.rolling_olhc(prices[i],t_stmps[i]);
    
        let time = t_stmps[i].to_string();
        let open = format!("{:.6}", roll_ohlc.ohlc.o);
        let high = format!("{:.6}", roll_ohlc.ohlc.h);
        let low = format!("{:.6}", roll_ohlc.ohlc.l);
        let close = format!("{:.6}", roll_ohlc.ohlc.c);
        (output.get_mut("timestamp").unwrap()).push(time);
        (output.get_mut("open").unwrap()).push(open);
        (output.get_mut("high").unwrap()).push(high);
        (output.get_mut("low").unwrap()).push(low);
        (output.get_mut("close").unwrap()).push(close);
        (output.get_mut("symbol").unwrap()).push(symbols[i].clone());
    }
    return output;
}
