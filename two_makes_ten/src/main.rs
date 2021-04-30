/* 
Link: https://edabit.com/challenge/jweDwK44Yyow8gXJs
Challenge: Create a function that takes two arguments. 
Both arguments are integers, a and b. Return true if 
one of them is 10 or if their sum is 10.

Examples
makes_ten(9, 10) ➞ true
makes_ten(9, 9) ➞ false
makes_ten(1, 9) ➞ true
*/

fn makes_ten(x:i32,y:i32) -> bool {
    if x == 10 || y == 10 {
        return true
    }
    else if x + y == 10 {
        return true
    } else {
        return false
    }
}

fn main() {
    println!("{}", makes_ten(9, 10)); // Expected: true
    println!("{}", makes_ten(9, 9)); // Expected: false
    println!("{}", makes_ten(1, 9)); // Expected: true
}
/* 
Output:
true
false
true
*/