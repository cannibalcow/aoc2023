advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut reading = true;

    let mut row = 1;
    let mut c: usize = 0;
    let mut sum = 0;
    let mut last_was_linebreak = false;

    println!("{}", input);

    while reading {
        match input.chars().nth(c) {
            Some('\n') => {
                if last_was_linebreak {
                    return Some(sum);
                }
                row += 1;
                c += 1;
                last_was_linebreak = true;
            }
            Some(v) => {
                if v.is_numeric() {
                    let num_start = c.clone();
                    let mut num_end = num_start;
                    let mut inner_pos = c.clone();
                    let mut num = String::new();

                    loop {
                        let ch = input.chars().nth(inner_pos).unwrap();
                        if ch.is_numeric() == false {
                            num_end -= 1;
                            break;
                        }
                        num.push(input.chars().nth(inner_pos).unwrap());
                        num_end += 1;
                        inner_pos += 1;
                        c += 1;
                    }

                    have_adjacent(input, row, num_start, num_end);

                    println!(
                        "row: {row} num: [{num_start}, {num_end}]: {} {num}",
                        num.len()
                    );
                }
                c += 1;
                last_was_linebreak = false;
            }
            None => return Some(sum),
        }
    }
    Some(sum)
}

fn have_adjacent(input: &str, row: usize, num_start: usize, num_end: usize) -> bool {
    let left = if num_start == 0 {
        input.chars().nth(num_start)
    } else {
        input.chars().nth(num_start - 1)
    };

    println!("left: {:?}", left);
    let right = input.chars().nth(num_end + 1);
    println!("right: {:?}", right);

    let above_row = input.lines().nth(row - 1);
    let bellow_row = input.lines().nth(row);

    let mut above = vec![];
    let mut bellow = vec![];
    for i in num_start..num_end {
        if above_row.is_some() {
            above.push(above_row.unwrap().chars().nth(i));
        }
        if bellow_row.is_some() {
            bellow.push(bellow_row.unwrap().chars().nth(i));
        }
    }

    println!("above: {:?}", above);
    println!("bellow: {:?}", bellow);

    false
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
