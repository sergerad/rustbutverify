use nom::{
    bytes::complete::is_not,
    character::complete::{char, line_ending},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};
use std::{collections::HashMap, fmt::Debug, hash::Hash, str::FromStr};

/// Parses a newline-separated list of node pairs.
///
/// Node pairs are separated by tabs.
pub fn parse_adjacency_list<T, E>(input: &str) -> IResult<&str, HashMap<T, Vec<T>>>
where
    T: Eq + Hash + Clone + FromStr<Err = E>,
    E: Debug,
{
    // Parse the list of adjacent nodes.
    let (remaining, edges) = separated_list1(
        line_ending,
        tuple((
            is_not("\t\n"), // First node: must not contain tab or newline.
            char('\t'),     // Exactly one tab.
            is_not("\t\n"), // Second node: must not contain tab or newline.
        )),
    )(input)?;

    // Construct the adjacency list.
    let mut adjacency_list: HashMap<T, Vec<T>> = HashMap::new();
    for (from, _, to) in edges {
        adjacency_list
            .entry(T::from_str(from).unwrap())
            .or_default()
            .push(T::from_str(to).unwrap());
    }
    Ok((remaining, adjacency_list))
}
