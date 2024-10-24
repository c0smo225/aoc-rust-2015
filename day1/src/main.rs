use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let mut count : i32 = 0;
    let mut place : i32 = 1;

    for x in input.chars(){
        if x == '('{
            count += 1;
        } else{
            count -= 1;
        }

        if count == -1{
            done(count, place);
            return;
        }

        place += 1;
    }
}

fn done(count : i32, place : i32) {
    println!("count : {}, place : {}", count, place);
}
