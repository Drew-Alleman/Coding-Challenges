
/*
Challenge: List all multiples of 3 or 5 for numbers 0-1000.
*/

fn is_multiple(number:u32) -> bool {
    number % 3 == 0 || number % 5 == 0
}

fn main() {
    let mut count:u32 = 1;
    while count != 1000 {
        if is_multiple(count) {
            println!("{}",count);
        }
        count += 1;
    }
}

/*
Output:
3
5
6
9
10
12
15
18
20
21
24
25
27
30
33
35
36
39
40
42
45
48
50
51
54
55
57
60
63
65
[...]
*/
