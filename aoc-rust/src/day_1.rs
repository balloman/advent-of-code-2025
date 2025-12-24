fn find_password(input: Vec<i32>) -> i32 {
    let mut current_value = 50;
    let mut total_times = 0;
    for value in input {
        current_value += value;

        current_value %= 100;
        // println!(
        //     "{0} {current_value}",
        //     match value.is_positive() {
        //         true => format!("R{}", value),
        //         false => format!("L{}", value.abs()),
        //     }
        // );
        if current_value == 0 {
            total_times += 1;
        }
    }
    total_times
}

enum FindPasswordError {
    Io(std::io::Error),
    Parse(std::num::ParseIntError),
}

impl std::fmt::Display for FindPasswordError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            FindPasswordError::Io(e) => write!(f, "IO Error: {}", e),
            FindPasswordError::Parse(e) => write!(f, "Parse Error: {}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::day_1::{FindPasswordError, find_password};
    use std::fs;

    #[test]
    fn test_password() {
        let input = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        let expected_output = 3;

        let actual_output = find_password(input);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn find_actual_password() {
        let contents_result: Result<Vec<i32>, _> = fs::read_to_string("input/day_1.txt")
            .map_err(FindPasswordError::Io)
            .and_then(|s| {
                s.lines()
                    .map(|line| line.replace("L", "-"))
                    .map(|line| line.replace("R", ""))
                    .map(|line| line.parse::<i32>().map_err(FindPasswordError::Parse))
                    .collect()
            });
        match contents_result {
            Ok(contents) => {
                let answer = find_password(contents);
                println!("Result is {answer}")
            }
            Err(err) => panic!("Error is {err}"),
        };
    }
}
