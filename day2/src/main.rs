use std::fs;

fn main() {
    let mut total_area = 0;
    let input = fs::read_to_string("./input.txt")
        .expect("Should have been able to read the file");

    let mut count = 0;
    let mut l = "".to_owned();
    let mut w = "".to_owned();
    let mut h = "".to_owned();

    for x in input.chars() {
        if x.is_numeric() {
            match count {
                0 => l.push_str(&x.to_string()),
                1 => w.push_str(&x.to_string()),
                2 => h.push_str(&x.to_string()),
                _ => return,
            }
        } else if x.is_whitespace() || x.is_ascii_whitespace() {
            total_area += calc_surface_area(l.parse::<i32>().unwrap(), w.parse::<i32>().unwrap(), h.parse::<i32>().unwrap());
            count = 0;
            l = "".to_string();
            w = "".to_string();
            h = "".to_string();
        } else if x == 'x' {
            count += 1;
        }
    }

    println!("{}", total_area);

}

fn calc_surface_area(l : i32, w : i32, h : i32) -> i32 {
    let first = 2*l*w;
    let second = 2*l*h;
    let third = 2*w*h;
    return first+second+third;
}
