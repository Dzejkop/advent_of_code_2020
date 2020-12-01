pub fn solution_1(mut ns: Vec<u32>) -> Option<u32> {
    ns.sort();

    recursive_combinations_over_sorted(&ns, 1, 2020, 0)
}

pub fn solution_2(mut ns: Vec<u32>) -> Option<u32> {
    ns.sort();

    recursive_combinations_over_sorted(&ns, 2, 2020, 0)
}

fn recursive_combinations_over_sorted(
    ns: &[u32],
    i: usize,
    target_sum: u32,
    sum_so_far: u32,
) -> Option<u32> {
    if ns.is_empty() {
        return None;
    }

    if i == 0 {
        return ns
            .iter()
            .copied()
            .filter(|n| n + sum_so_far == target_sum)
            .next();
    }

    ns.iter()
        .copied()
        .enumerate()
        .filter(|(_, n)| sum_so_far + n < target_sum)
        .filter_map(|(idx, n)| {
            let matching = recursive_combinations_over_sorted(
                &ns[idx + 1..],
                i - 1,
                target_sum,
                sum_so_far + n,
            )?;

            Some(n * matching)
        })
        .next()
}

#[cfg(test)]
mod tests {
    use super::*;
    const NUMS: &'static str = include_str!("nums.txt");

    fn input_data() -> Vec<u32> {
        NUMS.split('\n')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
    }

    #[test]
    fn solution_1_works() {
        let ns = input_data();

        assert_eq!(514579, solution_1(ns).unwrap());
    }

    #[test]
    fn solution_2_works() {
        let ns = input_data();

        assert_eq!(241861950, solution_2(ns).unwrap());
    }
}
