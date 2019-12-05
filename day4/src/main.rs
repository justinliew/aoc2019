fn number_to_vec(n: u32) -> Vec<u32> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect()
}

fn main() {
    let lower = 245318;
    let upper = 765747;

    let mut total = 0;
    for value in lower..=upper {
        let digits = number_to_vec(value);
        let num_digits = digits.len();
        let mut last = digits[0];
        let mut valid = false;
        for i in 0..num_digits {
            if digits[i] < last {
                valid = false;
                break;
            }

            let found = match i {
                0 => {
                    digits[0] == digits[1] &&
                    digits[1] != digits[2]
                },
                4 => {
                    digits[4] == digits[5] &&
                    digits[4] != digits[3]
                }
                5 => {
                    false
                },
                _ => {
                    digits[i] == digits[i+1] &&
                    digits[i-1] != digits[i] &&
                    digits[i+1] != digits[i+2]
                }
            };
            if found {
                valid = true;
            }
            last = digits[i];
        }

        if valid {
            total += 1;
        }
    }
    println!("We have {}", total);
}
