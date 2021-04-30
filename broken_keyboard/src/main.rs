use itertools::Itertools;
/*
Link: https://edabit.com/challenge/pbK3wzLDvdwfEKgMt
Challenge: Given what is supposed to be typed and what is actually typed, 
write a function that returns the broken key(s). The function looks like:

findBrokenKeys(correct phrase, what you actually typed)

Examples:
findBrokenKeys("happy birthday", "hawwy birthday") ➞ ["p"]
findBrokenKeys("starry night", "starrq light") ➞ ["y", "n"]
findBrokenKeys("beethoven", "affthoif5") ➞ ["b", "e", "v", "n"]

Notes:
* Broken keys should be ordered by when they first appear in the sentence.
* Only one broken key per letter should be listed.
* Letters will all be in lower case.
*/

// Using snake case 
fn find_broken_keys(correct:&str,typed:&str) -> Vec<char> {
    let mut broken_keys:Vec<char> = Vec::new(); // Create empty vector of broken characters
    let correct_list: Vec<char> = correct.chars().collect();
    let typed_list: Vec<char> = typed.chars().collect();
    let mut count = 0;
    while count < correct.len() {
        if correct_list[count] != typed_list[count] {
            broken_keys.push(correct_list[count]);
        }
        count += 1;
    }
    // Return list and remove dups
    return broken_keys.into_iter().unique().collect();
}

fn main() {
    println!("{:?}",find_broken_keys("happy birthday", "hawwy birthday"));
    println!("{:?}",find_broken_keys("beethoven", "affthoif5"));
    println!("{:?}",find_broken_keys("starry night", "starrq light"));
}

/*
Output:
['p']
['b', 'e', 'v', 'n']
['y', 'n']
*/