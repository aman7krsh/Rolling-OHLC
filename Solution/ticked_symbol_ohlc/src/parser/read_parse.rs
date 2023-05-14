use std::fs::File;
use std::path::Path;
use std::io::{self, BufRead};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
extern crate rolling_ohlc;


pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn parser(path:&str)->Option<HashMap<String, Vec<String>>>
{
    if let Ok(lines) = read_lines(path)
    {
        //Providing output in a Hashmap named "parsed_data"
        let mut parsed_data: HashMap<String, Vec<String>> = HashMap::new();
    
        for line in lines 
        {
            if let Ok(input_line) = line 
            {
                //Removing first & last curly braces from a line 
                let cur_line = (&input_line[1..input_line.len() - 1]).to_string();
                let vec_str:Vec<&str> =cur_line.split(',').collect();
                for key_val in vec_str.iter()
                {
                    if let Some((mut key,mut val)) = key_val.split_once(':')
                        {
                            //Removing quotes from keys & vals
                            let mut chars = key.chars();
                            chars.next();
                            chars.next_back();
                            key = chars.as_str();

                            //Values corresponding to 'T' don't have quotes
                            if key != "T"
                            {
                                let mut chars = val.chars();
                                chars.next();
                                chars.next_back();
                                val = chars.as_str();
                            }
                            //Storing keys & vals in a "parsed_data"Hashmap
                            match parsed_data.entry(key.to_string())
                            {
                                Entry::Vacant(e) => {e.insert(vec![val.to_string()]);},
                                Entry::Occupied(mut e) => {e.get_mut().push(val.to_string())}
                            }
                        }
                }
            }
        }
        return Some(parsed_data);
    }
    return None;
}



