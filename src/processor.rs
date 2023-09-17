use std::result;

use textdistance::nstr::damerau_levenshtein;

pub struct Detector {
    original_string: String,
    input_string: String,
    duplicate_ratio: f64,
}

impl Detector {
    pub fn new(original_string: String, input_string: String) -> Self {
        Detector {
            original_string,
            input_string,
            duplicate_ratio: 0.0,
        }
    }

    pub fn original_string(&self) -> &String {
        &self.original_string
    }

    pub fn input_string(&self) -> &String {
        &self.input_string
    }

    pub fn duplicate_ratio(&self) -> f64 {
        self.duplicate_ratio
    }

    pub fn set_original_string(&mut self, original_string: String) {
        self.original_string = original_string;
    }

    pub fn set_input_string(&mut self, input_string: String) {
        self.input_string = input_string;
    }

    fn from_result_to_ratio(result: f64) -> f64 {
        1.0 - result
    }

    pub fn compute_ratio(&mut self) -> f64 {
        let textdistance_result = damerau_levenshtein(&self.original_string, &self.input_string);
        self.duplicate_ratio = Self::from_result_to_ratio(textdistance_result);
        self.duplicate_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector_new() {
        let original_string = String::from("这是一个测试文档，用于测试软件的功能。");
        let input_string = String::from("它是测试文档，来测试软件功能");

        let detector = Detector::new(original_string, input_string);

        assert_eq!(
            detector.original_string,
            String::from("这是一个测试文档，用于测试软件的功能。")
        );
        assert_eq!(
            detector.input_string,
            String::from("它是测试文档，来测试软件功能")
        );
    }

    #[test]
    fn test_detector_getter() {
        let detector = Detector::new(
            String::from("这是一个测试文档，用于测试软件的功能。"),
            String::from("它是测试文档，来测试软件功能"),
        );

        assert_eq!(
            detector.original_string(),
            &String::from("这是一个测试文档，用于测试软件的功能。")
        );
        assert_eq!(
            detector.input_string(),
            &String::from("它是测试文档，来测试软件功能")
        );
        assert!(detector.duplicate_ratio() == 0.0);
    }

    #[test]
    fn test_detector_setter() {
        let mut detector = Detector::new(String::new(), String::new());

        detector.set_original_string(String::from("这是一个测试文档，用于测试软件的功能"));
        detector.set_input_string(String::from("它是测试文档，来测试软件功能。"));

        assert_eq!(
            detector.original_string,
            String::from("这是一个测试文档，用于测试软件的功能")
        );

        assert_eq!(
            detector.input_string,
            String::from("它是测试文档，来测试软件功能。")
        );
    }

    // reverse (minus by 1) and reserve 2 decimal places (half-up rounding)
    fn round_two_decimal(num: f64) -> f64 {
        (num * 100.0).round() / 100.0
    }

    #[test]
    fn test_detector_compute_ratio_partial_similar() {
        let mut detector = Detector::new(
            String::from("这是一个测试文档，用于测试软件的功能"),
            String::from("它是测试文档，来测试软件功能。"),
        );

        let ratio_rounded = round_two_decimal(detector.compute_ratio());

        assert!(
            ratio_rounded > 0.0 && ratio_rounded < 1.0,
            "ratio {} is not greater than 0.0",
            ratio_rounded
        );
    }

    #[test]
    fn test_detector_compute_ratio_not_similar() {
        let mut detector = Detector::new(
            String::from("这是一个测试文档，用于测试软件的功能"),
            String::from("完全不同的文字来确定重复率是否为0"),
        );

        let ratio_rounded = round_two_decimal(detector.compute_ratio());

        assert!(
            ratio_rounded <= (0.0 + 0.06),
            "ratio {} is not approximately equal to 0.0",
            ratio_rounded
        ); // allow ±6% error (lower is better)
    }

    #[test]
    fn test_detector_compute_ratio_almost_same() {
        let mut detector = Detector::new(
            String::from("接下来，测试完全相同的文字的重复率。"),
            String::from("接下来，测试完全相同的文字的重复率。"),
        );

        let ratio_rounded = round_two_decimal(detector.compute_ratio());

        assert!(
            ratio_rounded >= (1.0 - 0.06),
            "ratio {} is not approximately equal to 1.0",
            ratio_rounded
        ); // allow ±6% error (upper is better)
    }
}
