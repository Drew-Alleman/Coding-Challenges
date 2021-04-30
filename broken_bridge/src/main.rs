/*
Link: https://edabit.com/challenge/udrQ2ckXP9cXNXiSk
Challenge: Create a function which validates whether a bridge is safe to walk on (i.e. has no gaps in it to fall through).
Examples:
isSafeBridge("####") ➞ true
isSafeBridge("## ####") ➞ false
isSafeBridge("#") ➞ true
*/

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


fn main() {
    println!("is_safe_bridge('####')...........{}",is_safe_bridge("####"));
    println!("is_safe_bridge('## ####')........{}",is_safe_bridge("## ####"));
    println!("is_safe_bridge('#')..............{}",is_safe_bridge("#"));
    println!("is_safe_bridge('###a#####')......{}",is_safe_bridge("###a#####"));
}
/*
Output:
is_safe_bridge('####')...........true
is_safe_bridge('## ####')........false
is_safe_bridge('#')..............true
is_safe_bridge('###a#####')......false
*/