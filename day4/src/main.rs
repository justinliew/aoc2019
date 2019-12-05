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
        let mut last = digits[0];
        let mut valid = false;
        for i in 1..digits.len() {
            if digits[i] == last {
                valid = true;
            }
            if digits[i] < last {
                valid = false;
                break;
            }
            last = digits[i];
        }
        if valid {
            total += 1;
        }
    }
    println!("We have {}", total);
}
