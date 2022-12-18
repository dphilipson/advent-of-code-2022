use num::integer::ExtendedGcd;
use num::{Integer, Signed};

/// Applies the Chinese Remainder Theorem. Solves a system of congruences where
/// each congruence is given in the form `(a, n)`, which represents
/// `x = a (mod n)`. The result is returned in the same format `(b, m)`, meaning
/// that the solution to the system is `x = b (mod m)`.
pub fn solve_congruences<T: Copy + Integer + Signed>(congruences: &[(T, T)]) -> Option<(T, T)> {
    let (&head, rest) = congruences.split_first()?;
    let mut acc = head;
    for &c in rest {
        acc = solve_congruence_pair(acc, c)?;
    }
    Some(acc)
}

fn solve_congruence_pair<T: Copy + Integer + Signed>(
    (a, n): (T, T),
    (b, m): (T, T),
) -> Option<(T, T)> {
    let ExtendedGcd { gcd, x, y, .. } = n.extended_gcd(&m);
    let a_quotient = a / gcd;
    if a_quotient * gcd != a {
        return None;
    }
    let b_quotient = b / gcd;
    if b_quotient * gcd != b {
        return None;
    }
    let lcm = m / gcd * n;
    Some((modulus(a_quotient * m * y + b_quotient * n * x, lcm), lcm))
}

/// Like %, but always returns a positive value, even if the inputs are negative.
fn modulus<T: Copy + Integer + Signed>(a: T, m: T) -> T {
    let m = m.abs();
    ((a % m) + m) % m
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_solve_congruence_pair() {
        assert_eq!(solve_congruence_pair((1, 3), (3, 5)), Some((13, 15)));
        assert_eq!(solve_congruence_pair((3, 9), (9, 15)), Some((39, 45)));
        assert_eq!(solve_congruence_pair((3, 12), (4, 5)), Some((39, 60)));
        assert_eq!(solve_congruence_pair((2, 9), (9, 15)), None);
    }

    #[test]
    fn test_solve_congruences() {
        assert_eq!(solve_congruences(&[(0, 3), (3, 4), (4, 5)]), Some((39, 60)));
    }
}
