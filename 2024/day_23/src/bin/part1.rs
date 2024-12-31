fn main() {
    println!("Part1!");

    let input = include_str!("../input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Connection {
    nodes: Vec<String>,
}

fn parse(input: &str) -> Vec<Connection> {
    let mut ret: Vec<Connection> = vec![];

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();

        println!("{:?} - {:?}", a, b);

        let mut conn = Connection { nodes: Vec::new() };

        let mut added = false;

        for connection in &mut ret {
            if connection.nodes.iter().any(|s| s == a) {
                // println!("{:?} contains from line {:?}", connection, line);
                connection.nodes.push(b.to_string());
                added = true;
                break;
            } else if connection.nodes.iter().any(|s| a == b) {
                connection.nodes.push(a.to_string());
                added = true;
                break;
            }
        }
        if !added {
            conn.nodes.push(a.to_string());
            conn.nodes.push(b.to_string());
            ret.push(conn);
        }
        println!("{:?}", ret);
        println!();
    }

    ret
}

fn part1(input: &str) -> i32 {
    0
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let dummy = "kh-tc
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

        let temp = parse(&dummy);
        println!("{:?}", temp);

        assert_eq!(part1(dummy), 1);
    }
}
