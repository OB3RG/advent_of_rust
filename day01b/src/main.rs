use std::fs;

fn parse_file() -> Vec<u32> {
    return fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| l.parse::<u32>().unwrap())
        .collect()
}

fn parse_into_chunks(input: Vec<u32>) -> Vec<u32> {
    let mut vec: Vec<u32> = Vec::new();
    for i in 0..input.len()-2 {
        let one = i;
        let two = i.checked_add(1).unwrap();
        let three = i.checked_add(2).unwrap();

        let sum: u32 = input[one].checked_add(input[two]).unwrap().checked_add(input[three]).unwrap();
        vec.push(sum)
    }

    return vec
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

    let p = parse_file();
    let v = parse_into_chunks(p);
    let result = depth_increse(v);
    println!("{:?}", result);
}
