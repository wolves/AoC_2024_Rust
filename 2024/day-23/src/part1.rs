use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::prelude::UnGraphMap;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, edges) = parse(input).map_err(|e| {
        miette::miette!("parsing failed {}", e)
    })?;

    let g = &UnGraphMap::<&str, ()>::from_edges(&edges);

    let result = g
        .nodes()
        .flat_map(|node| {
            g.neighbors(node)
                .tuple_combinations()
                .filter(move |(a, b)| {
                    g.contains_edge(a, b)
                        && [node, a, b]
                            .iter()
                            .any(|n| n.starts_with("t"))
                })
                .map(move |(a, b)| {
                    let mut nodes = [node, a, b];
                    nodes.sort();
                    nodes
                })
        })
        .unique()
        .count();

    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        line_ending,
        separated_pair(alpha1, tag("-"), alpha1),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn";
        assert_eq!("7", process(input)?);
        Ok(())
    }
}
