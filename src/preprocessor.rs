use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

pub fn trim_and_convert(path: &PathBuf) -> io::Result<String> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut result = String::new();

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();
        result.push_str(line);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_full_functionalities() {
        // Create test files
        let file_path = PathBuf::from("./original.txt");
        fs::write(
            &file_path,
            "这是一个测试文档，用于测试软件的功能。\n\
            正如你所见，这是第二行文本。",
        )
        .unwrap();

        let result = trim_and_convert(&file_path).unwrap();
        assert_eq!(
            result,
            "这是一个测试文档，用于测试软件的功能。正如你所见，这是第二行文本。"
        );

        // Clean up
        fs::remove_file(&file_path).unwrap();
    }
}
