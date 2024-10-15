use std::io;

fn main() {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input).expect("failed to read line");

    let work : Vec<char> = input.chars().collect();

    let mut count = 0;
    let mut l_bracket = 0;
    let mut r_bracket = 0;

    for x in work{
        if x == '('{
            count += 1;
            l_bracket += 1;
        } else if x == ')'{
            count -= 1;
            r_bracket += 1;
        }
    }

    println!("{}, {}, {}", count, l_bracket, r_bracket);
}
