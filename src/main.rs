use colored::*;
use std::fmt;

#[derive(PartialEq)]
struct Group {
    depth: u8,
}

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("depth", &self.depth)
            .finish()
    }
}

const DEBUG: bool = false;

fn main() {
    let (input, garbage_character_count) = &parse(include_str!("../data/input.txt"));
    let score = get_group_score(input);
    println!("Part 1: {score}");
    if DEBUG { println!("Part 1 (debug): {:?}", input.iter().map(|p| p.depth).collect::<Vec<u8>>()); }
    println!("Part 2: {garbage_character_count}");
}

fn parse(input: &str) -> (Vec<Group>, u16) {
    let mut groups = Vec::new();
    let mut depth: u8 = 1;
    let mut is_within_garbage: bool = false;
    let mut last_valid_character_closed_group: bool = false;
    let mut garbage_character_count: u16 = 0;

    if DEBUG { println!("c\tdepth\tin_garb\tclosed\tgarbaage"); }

    for (pos, c) in input.char_indices() {
        let is_cancelled: bool = is_cancelled(c, pos, input);
        // Increment if this is garbage.
        garbage_character_count += should_increment_garbage_character_count(is_within_garbage, c, is_cancelled) as u16;

        if DEBUG { debug_iteration(c, depth, is_within_garbage, last_valid_character_closed_group, garbage_character_count); }

        if is_valid_character(is_within_garbage, c, is_cancelled) {
            match c {
                '<' => {
                    is_within_garbage = true;
                    if last_valid_character_closed_group { continue; }
                }
                '>' => {
                    is_within_garbage = false;
                    if last_valid_character_closed_group { continue; }
                }
                '{' => {
                    depth -= last_valid_character_closed_group as u8;
                    groups.push(Group {depth});
                    depth += 1;
                    last_valid_character_closed_group = false;
                }
                '}' => {
                    depth -= last_valid_character_closed_group as u8;
                    last_valid_character_closed_group = true;
                    continue;
                }
                ',' => {
                    if last_valid_character_closed_group { continue; }
                }
                _ => {
                    last_valid_character_closed_group = false;
                }
            }
        }
    }

    (groups, garbage_character_count)
}

fn is_valid_character(is_within_garbage: bool, c: char, is_cancelled: bool) -> bool {
    !is_cancelled && (!is_within_garbage || c == '>')
}

fn should_increment_garbage_character_count(is_within_garbage: bool, c: char, is_cancelled: bool) -> bool {
    is_within_garbage && c != '>' && !is_cancelled
}

fn debug_iteration(c: char, depth: u8, is_within_garbage: bool, last_valid_character_closed_group: bool, garbage_character_count: u16) {
    if is_within_garbage {
        println!("{}\t{}\t{}\t{}\t{}", c.to_string().red(), depth.to_string().red(), is_within_garbage.to_string().red(), last_valid_character_closed_group.to_string().red(), garbage_character_count.to_string().red());
    } else {
        println!("{}\t{}\t{}\t{}\t{}", c, depth, is_within_garbage, last_valid_character_closed_group, garbage_character_count);
    }
}

fn is_cancelled(c: char, pos: usize, input: &str) -> bool {
    if c == '!' {
        return true;
    } else if pos >= 1 && input.chars().nth(pos-1).unwrap() == '!' {
        let mut num_consecutive_cancels = 1;
        let mut positions_back = 2;
        // While the previous character is a cancel `!`, continue to iterate back through
        // characters, counting the number of consecutive cancels in `num_consecutive_cancels`.
        //
        // @todo I do not understand why Clippy considers the first comparison in the conditional
        // below to be absurd or unused. If you remove it, you can/will encounter out-of-bounds,
        // fatal error(s).
        #[allow(clippy::absurd_extreme_comparisons)]
        #[allow(unused_comparisons)]
        while (pos - positions_back) >= 0 && input.chars().nth(pos-positions_back).unwrap() == '!' {
            num_consecutive_cancels += 1;
            positions_back += 1;
        }
        // If this character, `c`, is preceded by an odd number of cancel characters, `!`, then
        // this character, `c`, is cancelled.
        if num_consecutive_cancels % 2 == 1 {
            return true;
        }
    }

    false
}

fn get_group_score(groups: &[Group]) -> usize {
    groups.iter().map(|a| a.depth as usize).sum()
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__unit__parse {
    use super::*;

    #[test]
    fn one_group() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        assert_eq!(parse("{}"), (groups, 0 as u16));
    }

    #[test]
    fn adjacent_group() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 1 });
        assert_eq!(parse("{},{}"), (groups, 0 as u16));
    }

    #[test]
    fn nested_groups() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 2 });
        assert_eq!(parse("{{}}"), (groups, 0 as u16));
    }

    #[test]
    fn garbage() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        assert_eq!(parse("{<{},{},{{}}>}"), (groups, 10 as u16));
    }

    #[test]
    fn garbage_with_cancels() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 2 });
        assert_eq!(parse("{{<!>},{<!>},{<!>},{<a>}}"), (groups, 13 as u16));
    }

    #[test]
    fn sibling_groups_separated_by_garbage() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 1 });
        assert_eq!(parse("{},<!>>{}"), (groups, 0 as u16));
    }

    #[test]
    fn many_curly_braces_and_commas() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 2 });
        groups.push(Group { depth: 3 });
        groups.push(Group { depth: 3 });
        groups.push(Group { depth: 3 });
        groups.push(Group { depth: 4 });
        assert_eq!(parse("{{{},{},{{}}}}"), (groups, 0 as u16));
    }

    #[test]
    fn input_partial() {
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 2 });
        groups.push(Group { depth: 3 });
        groups.push(Group { depth: 4 });
        groups.push(Group { depth: 5 });
        groups.push(Group { depth: 6 });
        groups.push(Group { depth: 4 });
        groups.push(Group { depth: 5 });
        groups.push(Group { depth: 6 });
        groups.push(Group { depth: 7 });
        groups.push(Group { depth: 8 });
        groups.push(Group { depth: 9 });
        groups.push(Group { depth: 10 });
        groups.push(Group { depth: 10 });
        groups.push(Group { depth: 7 });
        groups.push(Group { depth: 6 });
        groups.push(Group { depth: 4 });
        groups.push(Group { depth: 5 });
        groups.push(Group { depth: 6 });
        groups.push(Group { depth: 7 });
        groups.push(Group { depth: 7 });
        groups.push(Group { depth: 6 });
        groups.push(Group { depth: 7 });
        groups.push(Group { depth: 8 });
        assert_eq!(parse("{{{{{{},<!!!>>}},{{{{{{{<a!u!>!!!>!!}<>},{}}},<!>},<!!a!>,<!!!!!>!>!>,<i!io!!,!!}i!!!>},<a!>},<>},{<!>,<i'i}u{!\"!!!!u!}>}},{}}},{{{{<i!!!>,<<!!e}!!!!!!i!>{>},{}},{{{<i\"!>aa>}"), (groups, 44 as u16));
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__unit__get_group_score {
    use super::*;

    #[test]
    fn positive_case() {
        let sum = 7;
        let mut groups = Vec::new();
        groups.push(Group { depth: 1 });
        groups.push(Group { depth: 2 });
        groups.push(Group { depth: 2 });
        groups.push(Group { depth: 2 });
        assert_eq!(get_group_score(groups.as_slice()), sum);
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests__unit__is_cancelled {
    use super::*;

    #[test]
    fn current_character() {
        let c = '!';
        let pos = 0;
        let input = "!{}";
        assert_eq!(is_cancelled(c, pos, input), true);
    }

    #[test]
    fn previous_character() {
        let c = '>';
        let pos = 4;
        let input = "{}<!>{}";
        assert_eq!(is_cancelled(c, pos, input), true);
    }

    #[test]
    fn double_exclamation() {
        let c = '>';
        let pos = 5;
        let input = "{}<!!>{}";
        assert_eq!(is_cancelled(c, pos, input), false);
    }

    #[test]
    fn triple_exclamation() {
        let c = '>';
        let pos = 6;
        let input = "{}<!!!>{}";
        assert_eq!(is_cancelled(c, pos, input), true);
    }

    #[test]
    fn negative_case() {
        let c = '{';
        let pos = 3;
        let input = "{},{}";
        assert_eq!(is_cancelled(c, pos, input), false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_group() {
        // {}, score of 1. 1 point.
        let (input, garbage_character_count) = &parse("{}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 1);
        assert_eq!(solution, 1);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_only_curly_braces() {
        // {{{}}}, 3 groups. 1+2+3=6 points.
        let (input, garbage_character_count) = &parse("{{{}}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 3);
        assert_eq!(solution, 6);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_curly_braces_and_commas() {
        // {{},{}}, 3 groups 1+2+2=5 points.
        let (input, garbage_character_count) = &parse("{{},{}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 3);
        assert_eq!(solution, 5);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_many_curly_braces_and_commas() {
        // {{{},{},{{}}}}, 6 groups. 1+2+3+3+3+4=16 points.
        let (input, garbage_character_count) = &parse("{{{},{},{{}}}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 6);
        assert_eq!(solution, 16);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_one_garbage_group() {
        // {<{},{},{{}}>}, 1 group. 1=1 points.
        let (input, garbage_character_count) = &parse("{<{},{},{{}}>}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 1);
        assert_eq!(solution, 1);
        assert_eq!(*garbage_character_count, 10);
    }

    #[test]
    fn test_many_garbage_groups() {
        // {<a>,<a>,<a>,<a>}, 1 group. 1=1 points.
        let (input, garbage_character_count) = &parse("{<a>,<a>,<a>,<a>}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 1);
        assert_eq!(solution, 1);
        assert_eq!(*garbage_character_count, 4);
    }

    #[test]
    fn test_many_groups_containing_garbage() {
        // {{<a>},{<a>},{<a>},{<a>}}, 5 groups. 1+2+2+2+2=9 points.
        let (input, garbage_character_count) = &parse("{{<a>},{<a>},{<a>},{<a>}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 5);
        assert_eq!(solution, 9);
        assert_eq!(*garbage_character_count, 4);
    }

    #[test]
    fn test_many_groups_with_cancels() {
        // {{<!>},{<!>},{<!>},{<a>}}, 2 groups. 1+2=3 points.
        let (input, garbage_character_count) = &parse("{{<!>},{<!>},{<!>},{<a>}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 2);
        assert_eq!(solution, 3);
        assert_eq!(*garbage_character_count, 13);
    }

    #[test]
    fn test_adjacent_groups_containing_garbage() {
        // {{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        let (input, garbage_character_count) = &parse("{{<ab>},{<ab>},{<ab>},{<ab>}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 5);
        assert_eq!(solution, 9);
        assert_eq!(*garbage_character_count, 8);
    }

    #[test]
    fn test_adjacent_groups_containing_garbage_with_double_exclamations() {
        // {{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        let (input, garbage_character_count) = &parse("{{<!!>},{<!!>},{<!!>},{<!!>}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 5);
        assert_eq!(solution, 9);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_cancelled_contents() {
        // {{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
        let (input, garbage_character_count) = &parse("{{<a!>},{<a!>},{<a!>},{<ab>}}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 2);
        assert_eq!(solution, 3);
        assert_eq!(*garbage_character_count, 17);
    }

    #[test]
    fn test_sibling_group_separated_by_garbage() {
        // This is a test case that was not explicitly mentioned in the instructions, but broke my
        // initial parsing logic since I (naively) assumed that the '{' would follow a ','.
        //
        // {},<!>>{}, score 1 + 1 = 2.
        let (input, garbage_character_count) = &parse("{},<!>>{}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 2);
        assert_eq!(solution, 2);
        assert_eq!(*garbage_character_count, 0);
    }

    #[test]
    fn test_large_input() {
        let (input, garbage_character_count) = &parse("{{{{{{},<!!!>>}},{{{{{{{<a!u!>!!!>!!}<>},{}}},<!>},<!!a!>,<!!!!!>!>!>,<i!io!!,!!}i!!!>},<a!>},<>},{<!>,<i'i}u{!\"!!!!u!}>}},{}}},{{{{<i!!!>,<<!!e}!!!!!!i!>{>},{}},{{{<i\"!>aa>}");
        let solution = get_group_score(input);
        assert_eq!(input.len(), 24);
        assert_eq!(solution, 143);
        assert_eq!(*garbage_character_count, 44);
    }
}
