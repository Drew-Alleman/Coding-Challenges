/*
A tetrahedral number, or triangular pyramidal number, is a figurate number that represents a 
pyramid with a triangular base and three sides, called a tetrahedron. ~ From Wikipedia

Link: https://edabit.com/challenge/zRCyxKBBmr4F2x4Bv

Challenge: A tetrahedron is a pyramid with a triangular base and three sides. A tetrahedral number is a number of items within a tetrahedron.
Create a function that takes an integer n and returns the nth tetrahedral number.
Formula:
t = n(n+1)(n+2) = n^3
     ----------   ----
          6        3!
Examples:
tetra(2) ➞ 4
tetra(5) ➞ 35
tetra(6) ➞ 56
*/
fn tetra(n:i32)  -> i32 {
    n * (n + 1) * (n + 2) / 6
}

fn main() {
    println!("{}",tetra(2));
    println!("{}",tetra(5)); 
    println!("{}",tetra(6)); 
}

/*
Output: 
4
35
56
*/