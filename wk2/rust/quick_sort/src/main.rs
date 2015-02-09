use std::old_io::BufferedReader;
use std::old_io::File;

fn main() {
    let mut file = BufferedReader::new(File::open(&Path::new("IntegerInput.txt")));
    let integers: Vec<usize> = file.lines().enumerate().map(|(i, x)| {
        x.unwrap()
        .as_slice()
        .trim()
        .parse::<usize>()
        .unwrap_or_else(|err| {
            panic!("Unable to parse int at line:{}: {}", i + 1, err)
        })
    }).collect();

    println!("Integers: {:?}", integers);
}
