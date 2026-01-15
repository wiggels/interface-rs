use std::cmp::Ordering;

/// Compares two strings using natural sort order.
/// Natural sort order compares numbers in a way humans expect,
/// so "swp10" comes after "swp2" instead of "swp10" being less than "swp2" as in ASCII sorting.
///
/// # Arguments
/// - `a`: The first string to compare.
/// - `b`: The second string to compare.
///
/// # Returns
/// An `Ordering` indicating the result of the comparison.
pub fn natural(a: &str, b: &str) -> Ordering {
    let mut a_iter = a.chars().peekable();
    let mut b_iter = b.chars().peekable();

    loop {
        match (a_iter.peek(), b_iter.peek()) {
            (Some(a_c), Some(b_c)) => {
                if a_c.is_ascii_digit() && b_c.is_ascii_digit() {
                    // Extract full numbers
                    let mut a_num = String::new();
                    while let Some(c) = a_iter.peek() {
                        if c.is_ascii_digit() {
                            a_num.push(*c);
                            a_iter.next();
                        } else {
                            break;
                        }
                    }

                    let mut b_num = String::new();
                    while let Some(c) = b_iter.peek() {
                        if c.is_ascii_digit() {
                            b_num.push(*c);
                            b_iter.next();
                        } else {
                            break;
                        }
                    }

                    // Safe to unwrap: we only collected digit characters above
                    let a_int = a_num.parse::<u64>().expect("a_num contains only digits");
                    let b_int = b_num.parse::<u64>().expect("b_num contains only digits");

                    match a_int.cmp(&b_int) {
                        Ordering::Equal => continue,
                        other => return other,
                    }
                } else {
                    // Compare characters
                    match a_c.cmp(b_c) {
                        Ordering::Equal => {
                            a_iter.next();
                            b_iter.next();
                            continue;
                        }
                        other => return other,
                    }
                }
            }
            (Some(_), None) => return Ordering::Greater,
            (None, Some(_)) => return Ordering::Less,
            (None, None) => return Ordering::Equal,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_natural_basic() {
        assert_eq!(natural("swp1", "swp2"), Ordering::Less);
        assert_eq!(natural("swp10", "swp2"), Ordering::Greater);
        assert_eq!(natural("swp10", "swp10"), Ordering::Equal);
        assert_eq!(natural("swp100", "swp20"), Ordering::Greater);
        assert_eq!(natural("swp2", "swp2"), Ordering::Equal);
        assert_eq!(natural("swp10s1", "swp10s2"), Ordering::Less);
        assert_eq!(natural("swp10", "swp9s1"), Ordering::Greater);
    }

    #[test]
    fn test_sort_natural_different_prefixes() {
        assert_eq!(natural("eth0", "swp0"), Ordering::Less);
        assert_eq!(natural("lo", "eth0"), Ordering::Greater);
        assert_eq!(natural("bridge", "bond0"), Ordering::Greater);
    }

    #[test]
    fn test_sort_natural_vlan_interfaces() {
        assert_eq!(natural("vlan1", "vlan2"), Ordering::Less);
        assert_eq!(natural("vlan10", "vlan9"), Ordering::Greater);
        assert_eq!(natural("vlan100", "vlan1000"), Ordering::Less);
    }

    #[test]
    fn test_sort_natural_empty_and_numeric() {
        assert_eq!(natural("", ""), Ordering::Equal);
        assert_eq!(natural("a", ""), Ordering::Greater);
        assert_eq!(natural("", "a"), Ordering::Less);
        assert_eq!(natural("1", "2"), Ordering::Less);
        assert_eq!(natural("10", "2"), Ordering::Greater);
    }

    #[test]
    fn test_sort_natural_leading_zeros() {
        // Leading zeros should be treated as the same number
        assert_eq!(natural("swp01", "swp1"), Ordering::Equal);
        assert_eq!(natural("swp001", "swp1"), Ordering::Equal);
    }

    #[test]
    fn test_sort_vec() {
        let mut interfaces = vec!["swp10", "swp2", "swp1", "eth0", "lo", "swp20"];
        interfaces.sort_by(|a, b| natural(a, b));
        assert_eq!(interfaces, vec!["eth0", "lo", "swp1", "swp2", "swp10", "swp20"]);
    }
}