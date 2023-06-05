use itertools::Itertools;

pub fn parse_elves_caloric_rations(input: &str) -> Vec<Vec<u32>> {
    input
        .split("\n\n")
        .map(|rations_pack| {
            rations_pack
                .trim()
                .lines()
                .map(|pack_entry| {
                    pack_entry
                        .parse()
                        .unwrap()
            })
            .collect()
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let elf_calorie_packs = parse_elves_caloric_rations(input);

    let result: u32 = elf_calorie_packs
        .into_iter()
        .map(|rations_pack| rations_pack.into_iter().sum())
        .max()
        .unwrap();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let elf_calorie_packs = parse_elves_caloric_rations(input);

    // Sort elements
    // I wish I knew of a more clever way of doing this, but it works so...
    let sorted_packs: Vec<u32> = elf_calorie_packs
        .into_iter()
        .map(|rations_pack| {
            rations_pack
                .into_iter()
                .sum()
        })
        .sorted_by(|a: &u32, b| b.cmp(a))
        .collect();

    Some(sorted_packs[0..3].iter().sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);

    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);

        assert_eq!(part_one(&input), Some(100));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);

        assert_eq!(part_two(&input), None);
    }
}
