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

    pub fn compute_ratio(&mut self) -> f64 {
        self.duplicate_ratio = damerau_levenshtein(&self.original_string, &self.input_string);
        self.duplicate_ratio
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detector() {
        let original_string = String::from("这是一个测试文档，用于测试软件的功能。");
        let input_string = String::from("它是测试文档，来测试软件功能");

        let mut detector = Detector::new(original_string, input_string);

        assert_eq!(
            detector.original_string,
            String::from("这是一个测试文档，用于测试软件的功能。")
        );
        assert_eq!(
            detector.input_string,
            String::from("它是测试文档，来测试软件功能")
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

        // reverse (minus by 1) and reserve 2 decimal places (half-up rounding)
        fn reverse_and_round(num: f64) -> f64 {
            ((1.0 - num) * 100.0).round() / 100.0
        }

        let ratio_rounded = reverse_and_round(detector.compute_ratio());

        assert!(
            ratio_rounded > 0.0,
            "ratio {} is not greater than 0.0",
            ratio_rounded
        );

        detector.set_input_string(String::from("完全不同的文字来确定重复率是否为0"));

        assert_eq!(
            detector.input_string,
            String::from("完全不同的文字来确定重复率是否为0")
        );

        let ratio_rounded = reverse_and_round(detector.compute_ratio());

        assert!(
            ratio_rounded <= 0.05,
            "ratio {} is not less than or equal to 0.05",
            ratio_rounded
        ); // allow 5% error

        detector.set_original_string(String::from("接下来，测试完全相同的文字的重复率。"));
        detector.set_input_string(String::from("接下来，测试完全相同的文字的重复率。"));

        assert_eq!(
            detector.original_string,
            String::from("接下来，测试完全相同的文字的重复率。")
        );

        let ratio_rounded = reverse_and_round(detector.compute_ratio());

        assert!(
            ratio_rounded >= 0.95,
            "ratio {} is not greater than or equal to 0.95",
            ratio_rounded
        ); // allow 5% error
    }
}
