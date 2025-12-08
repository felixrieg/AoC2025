use core::time;
use std::fs;

pub fn read_input(day: u8, example: bool) -> String {
    let filename = match example {
        true => format!("inputs/day{:02}_example.txt", day),
        false => format!("inputs/day{:02}.txt", day),
    };
    fs::read_to_string(&filename)
        .unwrap_or_else(|_| panic!("Input-Datei {} nicht gefunden", filename))
}

pub fn read_lines(day: u8, example: bool) -> Vec<String> {
    read_input(day, example)
        .lines()
        .map(|s| s.to_string())
        .collect()
}

#[allow(dead_code)]
pub fn read_numbers(day: u8, example: bool) -> Vec<i64> {
    read_lines(day, example)
        .iter()
        .filter_map(|s| s.parse::<i64>().ok())
        .collect()
}

pub fn read_input_and_split(day: u8, example: bool, separator: &str) -> Vec<String> {
    read_input(day, example)
        .trim()
        .split(separator)
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.trim().to_string())
        .collect()
}

pub fn split_on_empty_lines(input: &[String]) -> Vec<Vec<String>> {
    input
        .split(|line| line.trim().is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| group.to_vec())
        .collect()
}

pub fn convert_to_matrix(input: &[String]) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

pub fn print_grid_header() {
    println!("+---------------------------------------------------+");
    println!("| Day  | Part                         | Duration    |");
    println!("+---------------------------------------------------+");
}

pub fn print_grid(
    day: u8,
    solution1: usize,
    solution2: usize,
    time1: time::Duration,
    time2: time::Duration,
) {
    println!(
        "| {:>2}   | Part 1: {:<20} |  {:>3}.{:03} ms |",
        day,
        solution1,
        time1.subsec_millis(),
        time1.subsec_micros() % 1000
    );
    println!("|------+--------------------------------------------|");
    println!(
        "|      | Part 2: {:<20} |  {:>3}.{:03} ms |",
        solution2,
        time2.subsec_millis(),
        time2.subsec_micros() % 1000
    );
    println!("+---------------------------------------------------+");
}

pub fn print_time(time: time::Duration) {
    println!(
        "|      |                              | {:>4}.{:03} ms |",
        time.subsec_millis(),
        time.subsec_micros() % 1000
    );
    println!("+---------------------------------------------------+");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // Helper: Erstelle Test-Eingabedatei
    fn create_test_input_file(day: u8, content: &str) {
        let _ = fs::create_dir_all("inputs");
        let filename = format!("inputs/day{:02}.txt", day);
        fs::write(&filename, content).expect("Fehler beim Schreiben der Test-Datei");
    }

    // Helper: Lösche Test-Eingabedatei
    fn cleanup_test_file(day: u8) {
        let filename = format!("inputs/day{:02}.txt", day);
        let _ = fs::remove_file(&filename);
    }

    #[test]
    fn test_read_input_success() {
        let day = 99;
        let content = "Hello\nWorld";
        create_test_input_file(day, content);

        let result = read_input(day, false);

        assert_eq!(result, "Hello\nWorld");
        cleanup_test_file(day);
    }

    #[test]
    #[should_panic(expected = "Input-Datei")]
    fn test_read_input_file_not_found() {
        let day = 98;
        cleanup_test_file(day);
        let _ = read_input(day, false);
    }

    #[test]
    fn test_read_lines_basic() {
        let day = 97;
        let content = "line1\nline2\nline3";
        create_test_input_file(day, content);

        let lines = read_lines(day, false);

        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "line1");
        assert_eq!(lines[1], "line2");
        assert_eq!(lines[2], "line3");
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_lines_empty_file() {
        let day = 96;
        let content = "";
        create_test_input_file(day, content);

        let lines = read_lines(day, false);

        assert_eq!(lines.len(), 0);
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_lines_single_line() {
        let day = 95;
        let content = "only one line";
        create_test_input_file(day, content);

        let lines = read_lines(day, false);

        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "only one line");
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_input_and_split_comma() {
        let day = 94;
        let content = "apple,banana,cherry";
        create_test_input_file(day, content);

        let parts = read_input_and_split(day, false, ",");

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "apple");
        assert_eq!(parts[1], "banana");
        assert_eq!(parts[2], "cherry");
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_input_and_split_with_whitespace() {
        let day = 93;
        let content = "  apple , banana , cherry  ";
        create_test_input_file(day, content);

        let parts = read_input_and_split(day, false, ",");

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "apple");
        assert_eq!(parts[1], "banana");
        assert_eq!(parts[2], "cherry");
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_input_and_split_custom_separator() {
        let day = 92;
        let content = "one;two;three";
        create_test_input_file(day, content);

        let parts = read_input_and_split(day, false, ";");

        assert_eq!(parts.len(), 3);
        assert_eq!(parts[0], "one");
        assert_eq!(parts[1], "two");
        assert_eq!(parts[2], "three");
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_numbers_parsing() {
        let day = 91;
        let content = "123\n456\nnot a number\n789";
        create_test_input_file(day, content);

        let numbers = read_numbers(day, false);

        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0], 123);
        assert_eq!(numbers[1], 456);
        assert_eq!(numbers[2], 789);
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_numbers_negative() {
        let day = 90;
        let content = "-123\n456\n-789";
        create_test_input_file(day, content);

        let numbers = read_numbers(day, false);

        assert_eq!(numbers.len(), 3);
        assert_eq!(numbers[0], -123);
        assert_eq!(numbers[1], 456);
        assert_eq!(numbers[2], -789);
        cleanup_test_file(day);
    }

    #[test]
    fn test_read_numbers_empty_file() {
        let day = 89;
        let content = "not numbers\nat all";
        create_test_input_file(day, content);

        let numbers = read_numbers(day, false);

        assert_eq!(numbers.len(), 0);
        cleanup_test_file(day);
    }

    #[test]
    fn test_split_on_empty_lines_basic() {
        let input = vec![
            "line1".to_string(),
            "line2".to_string(),
            "".to_string(),
            "line3".to_string(),
            "line4".to_string(),
        ];

        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[1].len(), 2);
        assert_eq!(groups[0], vec!["line1".to_string(), "line2".to_string()]);
        assert_eq!(groups[1], vec!["line3".to_string(), "line4".to_string()]);
    }

    #[test]
    fn test_split_on_empty_lines_multiple_groups() {
        let input = vec![
            "a".to_string(),
            "b".to_string(),
            "".to_string(),
            "c".to_string(),
            "".to_string(),
            "d".to_string(),
            "e".to_string(),
        ];

        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].len(), 2);
        assert_eq!(groups[1].len(), 1);
        assert_eq!(groups[2].len(), 2);
    }

    #[test]
    fn test_split_on_empty_lines_whitespace() {
        let input = vec!["line1".to_string(), "   ".to_string(), "line2".to_string()];

        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0], vec!["line1".to_string()]);
        assert_eq!(groups[1], vec!["line2".to_string()]);
    }

    #[test]
    fn test_split_on_empty_lines_no_groups() {
        let input = vec!["line1".to_string(), "line2".to_string()];
        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].len(), 2);
    }

    #[test]
    fn test_split_on_empty_lines_empty_input() {
        let input: Vec<String> = vec![];
        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 0);
    }

    #[test]
    fn test_split_on_empty_lines_only_empty() {
        let input = vec!["".to_string(), "".to_string()];
        let groups = split_on_empty_lines(&input);

        assert_eq!(groups.len(), 0);
    }
}
