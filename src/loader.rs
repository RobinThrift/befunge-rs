
use std::io::File;
use std::string::String;
use std::io::BufferedReader;

pub fn load_file(path: &str) -> Vec<String> {
    let mut file = BufferedReader::new(File::open(&Path::new(path)));

    let lines: Vec<String> = file.lines().map(|l| l.unwrap()).collect();

    return lines;
}

#[cfg(test)]
mod tests {
    #[test]
    fn basic_loader_test() {
        let output = super::load_file("./tests/fixtures/loader_test.txt");
        assert_eq!(output, vec!["This\n".to_string(), "Is\n".to_string(), "The\n".to_string(), "Loader\n".to_string(), "Test\n".to_string()]);
    }
}
