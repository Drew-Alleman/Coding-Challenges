# Coding-Challenges
Coding Challenges from various websites

## Broken Bridge
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/broken_bridge/src/main.rs)
```rust
fn is_safe_bridge(bridge:&str) -> bool {
    if bridge.contains(" ") { // If gap in bridge
        return false
    } 
    for c in bridge.chars() {
        if c != '#' { // If char is not bridge block
            return false
        }
    }
    return true
}
```


## Broken Keyboard
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/broken_keyboard/src/main.rs)
```rust
fn find_broken_keys(correct:&str,typed:&str) -> Vec<char> {
    let mut broken_keys:Vec<char> = Vec::new(); // Create empty vector of broken characters
    let correct_list: Vec<char> = correct.chars().collect();
    let typed_list: Vec<char> = typed.chars().collect();
    count = 0;
    while count < correct.len() {
        if correct_list[count] != typed_list[count] {
            broken_keys.push(correct_list[count]);
        }
        count += 1;
    }
    // Return list and remove dups
    return broken_keys.into_iter().unique().collect();
}
```
## Multiples of 3 or 5
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/multiples_of_3_or_5/src/main.rs)
```rust
fn is_multiple(number:u32) -> bool {
    number % 3 == 0 || number % 5 == 0
}
```
## nth Tetrahedral
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/n_tetrahedral/src/main.rs)
```rust
fn tetra(n:i32)  -> i32 {
    n * (n + 1) * (n + 2) / 6
}
```
## is Pandigital
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/pandigital_numbers/src/main.rs)
```rust
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
```
## Two Makes Ten
[Full code link](https://github.com/Drew-Alleman/Coding-Challenges/blob/main/two_makes_ten/src/main.rs)
```rust
fn makes_ten(x:i32,y:i32) -> bool {
    if x == 10 || y == 10 {
        return true
    } else if x + y == 10 {
        return true
    } else {
        return false
    }
}
```
