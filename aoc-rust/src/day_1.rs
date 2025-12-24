fn find_password_part_1(input: Vec<i32>) -> i32 {
    let mut current_value = 50;
    let mut total_times = 0;
    for value in input {
        current_value += value;

        current_value %= 100;
        if current_value == 0 {
            total_times += 1;
        }
    }
    total_times
}

fn find_password_part_2(input: Vec<i32>) -> i32 {
    let mut current_value = 50;
    let mut total_times = 0;
    for value in input {
        let previous_value = current_value;
        let unmodified_value = current_value + value;

        // We can determine the number of times it was set to zero by how many full rotations
        // were performed
        // Dividing the increment by the width of the lock is the minimum number of times we have
        // a zero
        let set_to_zero = value / 100;
        total_times += set_to_zero.abs();
        let bounded = unmodified_value % 100;
        current_value = if bounded >= 0 { bounded } else { 100 + bounded };
        if current_value == 0 {
            total_times += 1;
        }

        /*
        Additionally, we should check on if we pointed in one direction, but ended up having a
        greater bounded value. This means we had to have passed zero. If we're at or passed zero,
         than we don't need to count it
         */
        if ((value < 0 && current_value > previous_value)
            || (value > 0 && current_value < previous_value))
            && current_value != 0
            && previous_value != 0
        {
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
    use crate::day_1::{FindPasswordError, find_password_part_1, find_password_part_2};
    use std::fs;

    #[test]
    fn test_password() {
        let input = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        let expected_output = 3;

        let actual_output = find_password_part_1(input);

        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_password_part_2() {
        let input = vec![-68, -30, 48, -5, 60, -55, -1, -99, 14, -82];
        let other_input = vec![-1000];
        let expected_output = 10;

        let actual_output = find_password_part_2(input);

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
                let answer = find_password_part_1(contents);
                println!("Result is {answer}")
            }
            Err(err) => panic!("Error is {err}"),
        };
    }

    #[test]
    fn find_actual_password_part_2() {
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
                let answer = find_password_part_2(contents);
                println!("Result is {answer}")
            }
            Err(err) => panic!("Error is {err}"),
        };
    }
}
