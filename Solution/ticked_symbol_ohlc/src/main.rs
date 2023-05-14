mod parser;
use std::fs::File;
use std::io::Write;
use parser::read_parse::parser;
use parser::compute::compute_ohlc;
extern crate rolling_ohlc;

fn main() 
{
    let payload_path = "./../../Data/dataset-a.txt";
    let write_path = "./../../Data/ohlc-5m-a.txt";
    if let Some(ticked_data) = parser(payload_path){
        let time_window = 5;
        let ticked_ohlc = compute_ohlc(&ticked_data, time_window);

        let sym_vec = ticked_ohlc.get("symbol").unwrap();
        let tstmp_vec = ticked_ohlc.get("timestamp").unwrap();
        let open_vec = ticked_ohlc.get("open").unwrap();
        let high_vec = ticked_ohlc.get("high").unwrap();
        let low_vec = ticked_ohlc.get("low").unwrap();
        let close_vec = ticked_ohlc.get("close").unwrap();

        let total_ticked_data = tstmp_vec.len();
        let mut output_file = File::create(write_path).expect("creation failed");
        for i in 0..total_ticked_data{

            let ohlc_string= format!("{{\"symbol\":{},\"timestamp\":{},\"open\":{},\"high\":{},\"low\":{},\"close\":{}}}\n",
                                            format!("\"{}\"",sym_vec[i].to_string()),
                                            tstmp_vec[i].to_string(),
                                            format!("\"{}\"",open_vec[i].to_string()),
                                            format!("\"{}\"",high_vec[i].to_string()),
                                            format!("\"{}\"",low_vec[i].to_string()),
                                            format!("\"{}\"",close_vec[i].to_string()));
            output_file.write(ohlc_string.as_bytes()).expect("write failed");
        }
        println!("Created an output file ohlc-5m-b.txt");  
    }
    else{
        println!("Unable to parse payload!!");
    }
}

