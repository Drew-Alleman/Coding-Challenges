/*
Link: https://edabit.com/challenge/dmTWccZAo2QSr3yFR
Description: A pandigital number contains all digits (0-9) at least once. 
Write a function that takes an integer, returning true if the integer 
is pandigital, and false otherwise.

Examples:
isPandigital(98140723568910) ➞ true
isPandigital(90864523148909) ➞ false
// 7 is missing.
isPandigital(112233445566778899) ➞ false
*/

//Snake_case
fn is_Pandigital(number:i64) -> bool {
    let mut required_numbers:Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let number_list:Vec<char> = number.to_string().chars().collect();
    for num in number_list {
        if required_numbers.contains(&num) {
            let index = required_numbers.iter().position(|x| *x == num).unwrap();
            required_numbers.remove(index);
        }
    }
    if required_numbers.len() == 0 {
        return true
    }
    return false
}

fn main() {
    println!("{}",is_Pandigital(1234567890));
    println!("{}",is_Pandigital(98140723568910));
    println!("{}",is_Pandigital(90864523148909));
    println!("{}",is_Pandigital(112233445566778899));
}