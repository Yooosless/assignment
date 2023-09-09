use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rolling_ohlc::{RollingOHLC, TickerEvent};

fn main() {
    let window_size = 300;
    let filename = "C:\\Users\\Admin\\Desktop\\assignment\\04092023-Shahid-Afridi\\data\\dataset-b.txt"; //file path 
    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut ohlc_map: HashMap<String, RollingOHLC> = HashMap::new();

    for line in reader.lines() {
        if let Ok(line) = line {
           // println!("Read line: {:?}", line); // Debug output
    
            match serde_json::from_str::<TickerEvent>(&line) {
                Ok(event) => {
                    let symbol = event.s.clone();
                   // println!("Symbol: {:?}", symbol); // Debug output
    
                    let ohlc = ohlc_map
                        .entry(symbol.clone())
                        .or_insert_with(|| RollingOHLC::new(&symbol, window_size));
    
                    // Add the event to the RollingOHLC instance
                    ohlc.add_event(event);
    
                    // Calculate and print rolling OHLC values
                    if let Some((timestamp, open, high, low, close)) = ohlc.calculate_ohlc() {
                        println!(
                            "Symbol: {}, Timestamp: {}, Earliest: {}, Highest: {}, Lowest: {}, Latest: {}",
                            symbol, timestamp, open, high, low, close
                        );
                    }
                }
                Err(err) => {
                    println!("Failed to parse JSON: {:?}", err);
                }
            }
        }
    }
    
}
