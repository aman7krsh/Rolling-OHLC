pub mod ohlc_rolling;
use ohlc_rolling::ohlc::Ohlc;
use ohlc_rolling::rolling::RollingOhlc;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rolling_ohlc_5min() {
        let mut roll_ohlc_5min = RollingOhlc::new(5);
        roll_ohlc_5min.rolling_olhc(10.0, 1683730618000);
        roll_ohlc_5min.rolling_olhc(50.0, 1683730619000);
        roll_ohlc_5min.rolling_olhc(20.0, 1683730628001);
        roll_ohlc_5min.rolling_olhc(7.0, 1683730928000);
        assert_eq!(roll_ohlc_5min.ohlc.o, 20.0);
        assert_eq!(roll_ohlc_5min.ohlc.h, 20.0);
        assert_eq!(roll_ohlc_5min.ohlc.l, 7.0);
        assert_eq!(roll_ohlc_5min.ohlc.c, 7.0);
    }
}
