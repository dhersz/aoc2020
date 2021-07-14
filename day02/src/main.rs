fn main() {
    let passwords = include_str!("input.txt").lines();

    let valid_passwords_part1 = passwords
        .to_owned()
        .filter(|i| is_valid_password_part1(i))
        .count();

    println!("Part 1: {}", valid_passwords_part1);

    let valid_passwords_part2 = passwords
        .to_owned()
        .map(PasswordInfo::from)
        .filter(PasswordInfo::is_valid)
        .count();

    println!("Part 2: {}", valid_passwords_part2);
}

fn is_valid_password_part1(s: &str) -> bool {
    let mut password_info = s.split(' ');

    // the number of times the specified letter can appear
    let mut entries = password_info.next().unwrap().split('-');
    let lower_bound = entries.next().unwrap().parse::<usize>().unwrap();
    let upper_bound = entries.next().unwrap().parse::<usize>().unwrap();

    // the letter itself
    let letter = password_info.next().unwrap().chars().next().unwrap();

    // and the password
    let password = password_info.next().unwrap();

    let total_appearances = password.matches(letter).count();

    total_appearances >= lower_bound && total_appearances <= upper_bound
}

// trying a more structured answer on part 2 to practice some stuff

#[derive(Debug)]
struct PasswordInfo<'a> {
    low_position: usize,
    high_position: usize,
    letter: char,
    password: &'a str,
}

impl<'a> PasswordInfo<'a> {
    fn from(s: &str) -> PasswordInfo {
        let mut info_splits = s.split(' ');

        // the number of times the specified letter can appear
        let mut pos = info_splits.next().unwrap().split('-');
        let low_pos = pos.next().unwrap().parse::<usize>().unwrap();
        let high_pos = pos.next().unwrap().parse::<usize>().unwrap();

        // the letter itself
        let lett = info_splits.next().unwrap().chars().next().unwrap();

        // and the password
        let pass = info_splits.next().unwrap();

        PasswordInfo {
            low_position: low_pos,
            high_position: high_pos,
            letter: lett,
            password: pass,
        }
    }

    fn is_valid(&self) -> bool {
        let relevant_chars = self
            .password
            .chars()
            .enumerate()
            .filter(|(pos, _val)| {
                pos + 1 == self.low_position || pos + 1 == self.high_position
            })
            .map(|i| i.1)
            .collect::<Vec<char>>();

        (self.letter == relevant_chars[0] || self.letter == relevant_chars[1])
            && (relevant_chars[0] != relevant_chars[1])
    }
}
