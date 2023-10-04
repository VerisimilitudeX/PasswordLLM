pub mod cal_time {
    use round::{round, round_up};
    use thousands::Separable;
    #[allow(non_snake_case)]

    pub fn cal_time(GFLOP_64: u64, Entropy: f64) -> u64 {
        let MD5_Hashrate: u64 = (GFLOP_64 * 124) * 1000000;
        let combinations = round(f64::powf(2.0, Entropy), 0);

        println!("Possible Combinations {}", combinations.separate_with_commas());
        let time_sec = round_up((combinations / MD5_Hashrate as f64) / 2.0, 0);
        println!("Time: {} seconds", time_sec.separate_with_commas());
        return time_sec as u64
        // 29,586,412,362,451 / 51584000000 = 573 seconds to crack
    }
}
pub mod test {
    #[test]
    #[allow(non_snake_case)]
    
    pub fn check_time() {
        let GFLOP_64: u64 = 416;
        let Entropy = 44.75;

        use crate::cal_time;
        let result = cal_time(GFLOP_64, Entropy);

        assert_eq!(result, 287)
    }
}
