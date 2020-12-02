pub fn count_valid_passwords<'s>(
    s: &'s str,
    policy: impl for<'r> Fn(&'r PasswordEntry<'s>) -> bool,
) -> usize {
    s.split('\n')
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(|s| PasswordEntry::parse_str(s).unwrap())
        .filter(policy)
        .count()
}

pub struct PasswordEntry<'p> {
    range: (usize, usize),
    c: char,
    pass: &'p str,
}

impl<'p> PasswordEntry<'p> {
    fn is_valid_policy_1(&self) -> bool {
        let n = self.pass.chars().filter(|c| *c == self.c).count();

        n >= self.range.0 && n <= self.range.1
    }

    fn is_valid_policy_2(&self) -> bool {
        let a = self
            .pass
            .chars()
            .nth(self.range.0 - 1)
            .map(|c| c == self.c)
            .unwrap();

        let b = self
            .pass
            .chars()
            .nth(self.range.1 - 1)
            .map(|c| c == self.c)
            .unwrap();

        a ^ b
    }

    fn parse_str(s: &'p str) -> Option<Self> {
        let mut parts = s.split(':');
        let range_and_char = parts.next()?;
        let pass = parts.next()?.trim();

        let mut parts = range_and_char.split(' ');
        let range = parts.next()?;
        let c = parts.next()?;
        let c = c.chars().next()?;

        let mut range = range.split('-');
        let a = range.next()?.parse::<usize>().ok()?;
        let b = range.next()?.parse::<usize>().ok()?;

        Some(Self {
            range: (a, b),
            c,
            pass,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DATA: &'static str = include_str!("data.txt");

    #[test]
    fn part_1() {
        assert_eq!(
            2,
            count_valid_passwords(DATA, PasswordEntry::is_valid_policy_1)
        );
    }

    #[test]
    fn part_2() {
        assert_eq!(
            1,
            count_valid_passwords(DATA, PasswordEntry::is_valid_policy_2)
        );
    }
}
