use std::fs;

fn read_contents() -> String{
    let contents = fs::read_to_string("src/day1/input.txt").expect("Could not read file"); 

    return contents;
}


pub fn solve() {
    // Part 1
    let contents = read_contents();

    let elf = contents.split("\n\n");

    let mp = elf.into_iter().map(|e| e.lines().into_iter()
    .map(|x| x.parse::<u32>().unwrap()).sum());

    let m: u32 = mp.max().unwrap();

    println!("{}", m);

    // Part 2

    let mut a = contents.split("\n\n").into_iter().map(|e| e.lines().into_iter()
    .map(|x| x.parse::<u32>().unwrap()).sum()).collect::<Vec<u32>>();

    a.sort_by(|a, b| b.cmp(a));

    println!("{} {} {}", a[0], a[1], a[2]);

    let sum = a[0] + a[1] + a[2];

    println!("{}", sum);
}