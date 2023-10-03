pub mod cal_time {
    use round::{round, round_up};
    use thousands::Separable;

    pub fn cal_time(GFLOP_64: u64, Entropy: f64) {
        let MD5_Hashrate: u64 = (GFLOP_64 * 124) * 1000000;
        let combinations = round(f64::powf(2.0, Entropy), 0);

        //println!("MD5 Hashrate: {}", MD5_Hashrate);
        println!("Possible Combinations {}", combinations.separate_with_commas());
        let time_sec = round_up((combinations / MD5_Hashrate as f64) / 2.0, 0);
        println!("Time: {}", time_sec.separate_with_commas());
        // 29,586,412,362,451 / 51584000000 = 573 seconds to crack
    }
}
