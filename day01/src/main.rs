fn main() {
    let expenses = include_str!("input.txt")
        .lines()
        .map(|i| i.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    'outer: for i in expenses.iter() {
        for j in expenses.iter() {
            if i == j {
                continue;
            }

            for k in expenses.iter() {
                if i == k || j == k {
                    continue;
                }

                if *i + *k + *j == 2020 {
                    println!("({}, {}, {})", i, j, k);
                    println!("product: {}", i * j * k);

                    break 'outer;
                }
            }
        }
    }
}
