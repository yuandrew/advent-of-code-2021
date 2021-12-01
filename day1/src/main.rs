use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::slice::Windows;

fn read<R: Read>(io: R) -> Result<Vec<i64>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn main() {
    println!("part 1");
    let file = File::open("input.txt").unwrap();
    let data = read(file).unwrap();
    // Test case
    let test_data = vec![1, 2, 3, 4, 4, 3, 2, 5];
    let asdf = count_increase(&test_data);
    println!("test case: {}", asdf);
    // Actual
    let count = count_increase(&data);
    println!("Final count: {}", count);
    println!("other method: {}", count_increase1(&data));

    println!("part 2");
    // Test case
    let test_data = vec![1, 2, 3, 4, 5];
    let result = test_data.windows(3);
    println!("test case: {}", count_window_increase(result));
    // Actual
    let file = File::open("src/input.txt").unwrap();
    let data = read(file).unwrap();
    let result = data.windows(3);
    println!("count: {}", count_window_increase(result));
    println!("other method: {}", part_2(&data));
}

fn count_window_increase(window: Windows<i64>) -> i64 {
    let data = window.collect::<Vec<_>>();
    // println!("data: {:?}", data);
    let mut count = 0;
    for (index, v) in data[1..].iter().enumerate() {
        let sum: i64 = v.iter().sum();
        let prev_sum: i64 = data[index].iter().sum();
        // println!("sum: {}", sum);
        if sum > prev_sum {
            count += 1;
        }
    }
    count
}

fn part_2(data: &[i64]) -> i64 {
    let data = data
        .iter()
        .zip(data.iter().skip(1))
        .zip(data.iter().skip(2))
        .map(|((a, b), c)| (a + b + c))
        .collect::<Vec<_>>();

    let mut count = 0;
    for val in data.iter().zip(data.iter().skip(1)) {
        if *val.0 < *val.1 {
            count += 1;
        }
    }
    count
}

fn count_increase(data: &[i64]) -> i64 {
    let mut count = 0;
    for (index, v) in data[1..].iter().enumerate() {
        if *v > data[index] {
            count += 1;
        }
    }
    count
}

fn count_increase1(data: &[i64]) -> i64 {
    let mut count = 0;
    for v in data.iter().zip(data.iter().skip(1)).collect::<Vec<_>>() {
        if *v.1 > *v.0 {
            count += 1;
        }
    }
    count
}
