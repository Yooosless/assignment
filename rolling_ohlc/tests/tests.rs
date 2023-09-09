use rolling_ohlc::RollingOHLC;
use rolling_ohlc::TickerEvent;
use rand::Rng;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rolling_ohlc() {
        // Write test cases to check the creation of a RollingOHLC instance
        // For example:
        let ohlc = RollingOHLC::new("TURBOUSDT", 300);
        assert_eq!(ohlc.symbol, "TURBOUSDT");
        assert_eq!(ohlc.window_size, 300);
        assert!(ohlc.data.is_empty());
    }

    #[test]
    fn test_add_event() {
        // Write test cases to check adding events to RollingOHLC
        // For example:
        let mut ohlc = RollingOHLC::new("TURBOUSDT", 300);
        let event = TickerEvent {
            s: "TURBOUSDT".to_string(),
            b: "0.3265".to_string(),
            B: "157258.3".to_string(),
            a: "0.3266".to_string(),
            A: "63052.9".to_string(),
            T: 1662022860204,
            E: 1662022860209,
        };
        ohlc.add_event(event);
        assert_eq!(ohlc.data.len(), 1);
    }

    // Add more test functions for other methods as needed
    #[test]
fn test_calculate_ohlc_no_events() {
    let ohlc = RollingOHLC::new("TURBOUSDT", 300);
    assert_eq!(ohlc.calculate_ohlc(), None);
}


#[test]
fn test_calculate_ohlc_single_event() {
    let mut ohlc = RollingOHLC::new("TURBOUSDT", 300);
    let event = TickerEvent {
        s: "TURBOUSDT".to_string(),
        b: "0.3265".to_string(),
        B: "157258.3".to_string(),
        a: "0.3266".to_string(),
        A: "63052.9".to_string(),
        T: 1662022860204,
        E: 1662022860209,
    };
    ohlc.add_event(event.clone());
    assert_eq!(
        ohlc.calculate_ohlc(),
        Some((event.T, 0.3265, 0.3265, 0.3265, 0.3265))
    );
}

#[test]
fn test_trim_old_data_empty_data() {
    let mut ohlc = RollingOHLC::new("TURBOUSDT", 300);
    ohlc.trim_old_data();
    assert!(ohlc.data.is_empty());
}

#[test]
fn test_property_based_ohlc() {
    let mut ohlc = RollingOHLC::new("TURBOUSDT", 300);
    
    let mut rng = rand::thread_rng();
    
    for _ in 0..100 {
        let timestamp = rng.gen_range(1662022860000..1662022870000);  
        let price = rng.gen_range(0.325..0.330); 
        let event = TickerEvent {
            s: "TURBOUSDT".to_string(),
            b: price.to_string(),
            B: "0.0".to_string(),
            a: price.to_string(),
            A: "0.0".to_string(),
            T: timestamp,
            E: timestamp + rng.gen_range(1..100),  
        };
        ohlc.add_event(event);
    }
    
    if let Some((_, _, max_price, min_price, _)) = ohlc.calculate_ohlc() {
        assert!(max_price >= min_price);
    }
}

}
