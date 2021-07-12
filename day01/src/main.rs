fn main() {
    let expenses = include_str!("input.txt")
        .split('\n')
        .map(|i| i.parse::<i32>().unwrap_or(0)) // adds 0 to the end 
                                                // due to trailing '\n'
        .collect::<Vec<i32>>();

    for i in expenses.iter() {
        for j in expenses.iter() {
            for k in expenses.iter() {
                if *i + *k + *j == 2020 {
                    println!("{}-{}-{}", i, j, k);
                    println!("{}", i * j * k);
                }
            }
        }
    }
}
