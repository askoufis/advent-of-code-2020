use std::collections::HashSet;

#[derive(Debug, PartialEq)]
struct BagTree {
    bag_nodes: Vec<BagNode>,
}

#[derive(Debug, PartialEq)]
struct BagNode {
    bag_name: String,
    contained_in: Vec<BagEdge>,
    contains: Vec<BagEdge>,
}

#[derive(Debug, PartialEq)]
struct BagEdge {
    bag_name: String,
    count: usize,
}

#[derive(Debug, PartialEq)]
struct BagRule {
    bag_name: String,
    contains: Vec<ContainsRule>,
}

#[derive(Debug, PartialEq)]
struct ContainsRule {
    bag_name: String,
    count: usize,
}

impl BagRule {
    fn new(rule: &str) -> BagRule {
        let mut bag_rule_split = rule.split(" contain ");
        let bag_name = bag_rule_split
            .next()
            .unwrap()
            .strip_suffix(" bags")
            .unwrap();
        let contains_rules = bag_rule_split.next().unwrap().strip_suffix('.').unwrap();
        let contains: Vec<ContainsRule> = if contains_rules == "no other bags" {
            vec![]
        } else {
            contains_rules
                .split(", ")
                .into_iter()
                .map(|rule| {
                    let mut rule_split = rule.split(' ');
                    let count = rule_split.next().unwrap().parse().unwrap();
                    let mut contains_bag_name = String::from(rule_split.next().unwrap());
                    contains_bag_name.push(' ');
                    contains_bag_name.push_str(rule_split.next().unwrap());
                    ContainsRule {
                        bag_name: contains_bag_name,
                        count,
                    }
                })
                .collect()
        };

        BagRule {
            bag_name: String::from(bag_name),
            contains,
        }
    }
}

impl BagTree {
    fn new(capacity: usize) -> Self {
        BagTree {
            bag_nodes: Vec::with_capacity(capacity),
        }
    }

    fn add_bag_rule(&mut self, bag_rule: BagRule) {
        if let None = self
            .bag_nodes
            .iter()
            .position(|bag_node| bag_node.bag_name == bag_rule.bag_name)
        {
            let contains = bag_rule
                .contains
                .iter()
                .map(|rule| BagEdge {
                    bag_name: rule.bag_name.clone(),
                    count: rule.count,
                })
                .collect();
            let new_node = BagNode {
                bag_name: bag_rule.bag_name.clone(),
                contained_in: vec![],
                contains,
            };

            self.add_bag_node(new_node);
        }

        bag_rule.contains.iter().for_each(|rule| {
            let contained_in_edge = BagEdge {
                bag_name: bag_rule.bag_name.clone(),
                count: rule.count,
            };

            match self.find_bag_node_index(&rule.bag_name) {
                // There is already a node for this bag
                Some(index) => {
                    // We can safely unwrap here since we know the index exists
                    let bag_node = self.bag_nodes.get_mut(index).unwrap();
                    bag_node.contained_in.push(contained_in_edge);
                }
                // There is no existing node for this bag
                None => {
                    let new_node = BagNode {
                        bag_name: rule.bag_name.clone(),
                        contained_in: vec![contained_in_edge],
                        contains: vec![],
                    };

                    self.bag_nodes.push(new_node);
                }
            }
        });
    }

    fn add_bag_node(&mut self, bag_node: BagNode) {
        self.bag_nodes.push(bag_node);
    }

    fn find_bag_node_index(&self, bag_name: &str) -> Option<usize> {
        self.bag_nodes
            .iter()
            .position(|node| node.bag_name == bag_name)
    }

    fn find_num_ancestors(&self, bag_name: &str) -> usize {
        let ancestors: HashSet<String> = self
            .get_ancestors(bag_name)
            .iter()
            .map(|a| a.clone())
            .collect();

        ancestors.len()
    }

    fn get_ancestors(&self, bag_name: &str) -> Vec<String> {
        // The unwrap assumes you've filled out the tree
        let start_index = self.find_bag_node_index(bag_name).unwrap();
        let start_node = self.bag_nodes.get(start_index).unwrap();
        println!("{:#?}", start_node);
        if start_node.contained_in.len() == 0 {
            vec![]
        } else {
            let mut direct_ancestors: Vec<String> = start_node
                .contained_in
                .iter()
                .map(|contained_in| contained_in.bag_name.clone())
                .collect();
            let mut node_ancestors: Vec<String> = start_node
                .contained_in
                .iter()
                .map(|contained_in| self.get_ancestors(&contained_in.bag_name))
                .flatten()
                .collect();
            direct_ancestors.append(&mut node_ancestors);
            direct_ancestors
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> BagTree {
    let mut bag_tree = BagTree::new(input.lines().count());

    input.lines().for_each(|line| {
        bag_tree.add_bag_rule(BagRule::new(line));
    });

    bag_tree
}

#[aoc(day7, part1)]
fn part1(bag_tree: &BagTree) -> usize {
    bag_tree.find_num_ancestors("shiny gold")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn double_bag_rule_new_test() {
        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bag_rule = BagRule::new(rule);
        let expected = BagRule {
            bag_name: String::from("light red"),
            contains: vec![
                ContainsRule {
                    bag_name: String::from("bright white"),
                    count: 1,
                },
                ContainsRule {
                    bag_name: String::from("muted yellow"),
                    count: 2,
                },
            ],
        };

        assert_eq!(bag_rule, expected);
    }

    #[test]
    fn single_bag_rule_new_test() {
        let rule = "light red bags contain 1 bright white bag.";
        let bag_rule = BagRule::new(rule);
        let expected = BagRule {
            bag_name: String::from("light red"),
            contains: vec![ContainsRule {
                bag_name: String::from("bright white"),
                count: 1,
            }],
        };

        assert_eq!(bag_rule, expected);
    }

    #[test]
    fn empty_bag_rule_new_test() {
        let rule = "light red bags contain no other bags.";
        let bag_rule = BagRule::new(rule);
        let expected = BagRule {
            bag_name: String::from("light red"),
            contains: vec![],
        };

        assert_eq!(bag_rule, expected);
    }

    #[test]
    fn add_bag_rule_test() {
        let mut bag_tree = BagTree::new(3);

        let rule = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let bag_rule = BagRule::new(rule);

        bag_tree.add_bag_rule(bag_rule);

        let expected = BagTree {
            bag_nodes: vec![
                BagNode {
                    bag_name: String::from("light red"),
                    contained_in: vec![],
                    contains: vec![
                        BagEdge {
                            bag_name: String::from("bright white"),
                            count: 1,
                        },
                        BagEdge {
                            bag_name: String::from("muted yellow"),
                            count: 2,
                        },
                    ],
                },
                BagNode {
                    bag_name: String::from("bright white"),
                    contained_in: vec![BagEdge {
                        bag_name: String::from("light red"),
                        count: 1,
                    }],
                    contains: vec![],
                },
                BagNode {
                    bag_name: String::from("muted yellow"),
                    contained_in: vec![BagEdge {
                        bag_name: String::from("light red"),
                        count: 2,
                    }],
                    contains: vec![],
                },
            ],
        };
        assert_eq!(bag_tree, expected);
    }

    #[test]
    fn find_num_ancestors_test() {
        let mut bag_tree = BagTree::new(3);

        let rule1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let rule2 = "pale orange bags contain 2 light red bags.";
        let bag_rule1 = BagRule::new(rule1);
        let bag_rule2 = BagRule::new(rule2);

        bag_tree.add_bag_rule(bag_rule1);
        bag_tree.add_bag_rule(bag_rule2);

        let red_bag_ancestors = 1;
        let white_bag_ancestors = 2;
        let yellow_bag_ancestors = 2;

        assert_eq!(bag_tree.find_num_ancestors("light red"), red_bag_ancestors);
        assert_eq!(
            bag_tree.find_num_ancestors("bright white"),
            white_bag_ancestors
        );
        assert_eq!(
            bag_tree.find_num_ancestors("muted yellow"),
            yellow_bag_ancestors
        );
    }

    #[test]
    fn get_ancestors_test() {
        let mut bag_tree = BagTree::new(3);

        let rule1 = "light red bags contain 1 bright white bag, 2 muted yellow bags.";
        let rule2 = "pale orange bags contain 2 light red bags.";
        let bag_rule1 = BagRule::new(rule1);
        let bag_rule2 = BagRule::new(rule2);

        bag_tree.add_bag_rule(bag_rule1);
        bag_tree.add_bag_rule(bag_rule2);

        let expected_red_ancestors: Vec<String> = vec![String::from("pale orange")];
        let expected_orange_ancestors: Vec<String> = vec![];

        let red_ancestors = bag_tree.get_ancestors("light red");
        let orange_ancestors = bag_tree.get_ancestors("pale orange");

        assert_eq!(red_ancestors, expected_red_ancestors);
        assert_eq!(orange_ancestors, expected_orange_ancestors);
    }

    #[test]
    fn part1_test() {
        let input = r"light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        // println!("{}", input);
        let generated_input = input_generator(&input);
        println!("{:#?}", generated_input);
        let result = part1(&generated_input);
        let expected = 4;

        assert_eq!(result, expected);
    }
}
