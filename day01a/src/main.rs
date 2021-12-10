use std::fs;

fn parse_file() -> Vec<u32> {
    return fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

fn depth_increse(input: Vec<u32>) -> u32 {
    let mut times_increase: u32 = 0;
    for i in 0..input.len()-1 {
        let next_idx = i.checked_add(1).unwrap();
        if input[i] < input[next_idx] {
            times_increase = times_increase.checked_add(1).unwrap();
        }
    }

    return times_increase
}

fn main() {

    let v = parse_file();
    let result = depth_increse(v);
    println!("{:?}", result);
}
