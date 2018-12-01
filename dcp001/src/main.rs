/*
Daily Coding Problem #1:

"
 This problem was recently asked by Google.
 Given a list of numbers and a number k, return whether any two numbers from the list add up to k.
 For example, given [10, 15, 3, 7] and k of 17, return true since 10 + 7 is 17.

 Bonus: Can you do this in one pass?
"
*/
fn has_sum_of_two(input: &[i32], k: i32) -> bool {
    for i in 0..input.len() {
        for j in 0..input.len() {
            if i != j && input[i] + input[j] == k {
                return true;
            }
        }
    }

    return false;
}

fn main() {
    let input = [10, 15, 3, 7];
    let k = 20;
    if has_sum_of_two(&input, k) {
        println!("True");
    } else {
        println!("False");
    }
}
