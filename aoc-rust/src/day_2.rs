use std::ops::Range;

/// A function that finds bad ids given an input.
/// The input is defined as a list of ranges where the output are any numbers in the range that
/// consist of a pattern repeated twice summed together
/// ## Examples
/// ```rs
/// let input = String::from("11-22,95-115");
/// let expected_output = 11 + 22 + 99;
/// assert_eq!(find_bad_ids(input), expected_output);
/// ```
fn find_bad_ids(input: String) -> i64 {
    let mut total_sum = 0;
    // Split the input into ranges
    input.split(',').for_each(|range| {
        // For each range, create a Range that consists of the start and end + 1
        let split_range = range
            .split('-')
            .map(|id| match id.parse::<i64>() {
                Ok(val) => val,
                Err(e) => panic!("Encountered error parsing {id} into a string: {}", e),
            })
            .collect::<Vec<i64>>();
        let range = Range {
            start: split_range[0],
            end: split_range[1] + 1,
        };
        // Now that we have a Range, we can perform operations over it
        range.for_each(|x| {
            let num_string = x.to_string();
            let char_count = num_string.chars().count();
            // A number with an odd number of characters could never be the range
            if !char_count.is_multiple_of(2) {
                return;
            }
            // A number with an even number of characters counts if the first n / 2 digits = the
            // last n / 2 digits
            let half_width = char_count / 2;
            let half_slice = &num_string[0..half_width];
            let end_slice = &num_string[half_width..];
            if half_slice == end_slice {
                total_sum += x;
            }
        })
    });

    total_sum
}

#[cfg(test)]
mod test {
    use crate::day_2::find_bad_ids;
    use std::fs;

    #[test]
    fn test_bad_ids() {
        let input = String::from(
            "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124",
        );
        let result = find_bad_ids(input);

        assert_eq!(result, 1_227_775_554);
    }

    #[test]
    fn find_actual_ids() {
        let input = fs::read_to_string("input/day_2.txt").unwrap();
        println!("{0}", find_bad_ids(input));
    }
}
