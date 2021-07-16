fn main() {
    let tree_map = TreeMap::from(include_str!("input.txt"));

    // part 1
    let first_toboggan = Toboggan::from(Slope { right: 3, down: 1 });
    println!("First part: {}", tree_map.count_trees(&first_toboggan));

    // part 2
    let toboggan11 = Toboggan::from(Slope { right: 1, down: 1 });
    let toboggan31 = Toboggan::from(Slope { right: 3, down: 1 });
    let toboggan51 = Toboggan::from(Slope { right: 5, down: 1 });
    let toboggan71 = Toboggan::from(Slope { right: 7, down: 1 });
    let toboggan12 = Toboggan::from(Slope { right: 1, down: 2 });

    let tob_vec =
        vec![toboggan11, toboggan31, toboggan51, toboggan71, toboggan12];
    let mut prod = 1;

    println!("\nNow for second part:");
    for i in tob_vec {
        let trees = tree_map.count_trees(&i);
        println!(
            "Toboggan right {}, down {}: {}",
            i.slope.right, i.slope.down, trees
        );
        prod *= trees;
    }
    println!("Tree's product: {}", prod);
}

#[derive(Debug)]
struct TreeMap {
    data: Vec<Vec<char>>,
}

impl TreeMap {
    fn from(s: &str) -> Self {
        let data = s
            .lines()
            .map(|i| i.chars().collect::<Vec<char>>())
            .collect::<Vec<_>>();

        TreeMap { data }
    }

    fn get(&self, i: usize, j: usize) -> &char {
        if i >= self.height() {
            panic!("'i' out of bounds")
        };

        let j = if j >= self.data[0].len() {
            j % self.data[0].len()
        } else {
            j
        };

        &self.data[i][j]
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn count_trees(&self, toboggan: &Toboggan) -> usize {
        let mut count = 0;
        let mut i = 0;
        let mut j = 0;

        while i < self.height() {
            if *self.get(i, j) == '#' {
                count += 1;
            }

            i += toboggan.slope.down;
            j += toboggan.slope.right;
        }

        count
    }
}

#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
}

#[derive(Debug)]
struct Toboggan {
    slope: Slope,
}

impl Toboggan {
    fn from(slope: Slope) -> Self {
        Toboggan { slope }
    }
}
