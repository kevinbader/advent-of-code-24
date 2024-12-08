use std::{
    collections::{HashMap, HashSet, VecDeque},
    env::args,
    fs,
};

fn main() {
    let data = fs::read_to_string(args().nth(1).unwrap_or("input/day5.txt".into())).unwrap();
    let (rules, updates) = data
        .split_once("\n\n")
        .expect("rules -> empty line -> updates");
    let rules = parse_rules(rules);
    let updates = parse_updates(updates);

    // We check from back to front and take not what page numbers must not occur
    // earlier. If we find such a page, we know it's not in correct order.
    let mut midpoint_sum_already_ordered = 0;
    let mut midpoint_sum_newly_ordered = 0;
    for update in updates {
        if is_correctly_ordered(&update, &rules) {
            midpoint_sum_already_ordered += middle_page_number_of(&update);
        } else {
            let update = topological_sort(&update, &rules);
            midpoint_sum_newly_ordered += middle_page_number_of(&update);
        }
    }

    println!("Sum of middle page numbers of already correctly ordered updates: {midpoint_sum_already_ordered}");
    println!("Sum of middle page numbers of newly correctly ordered updates: {midpoint_sum_newly_ordered}");
}

fn is_correctly_ordered(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> bool {
    let mut forbidden: HashSet<u32> = HashSet::new();
    for pagenum in update.iter().rev() {
        if forbidden.contains(pagenum) {
            return false;
        }
        if let Some(later_pages) = rules.get(pagenum) {
            forbidden.extend(later_pages);
        }
    }
    true
}

fn topological_sort(update: &[u32], rules: &HashMap<u32, HashSet<u32>>) -> Vec<u32> {
    // Kahn's algorithm:
    // 1. Find list of nodes with no incoming edges.
    // 2. Remove edges from neighbors, then add node to result.
    // 3. Neighbors that no longer have any incoming edges are added to the processing queue.

    // The graph for the given update (= a subset of rules).
    let mut edges: HashMap<u32, HashSet<u32>> = HashMap::new();
    // Counters for the number of dependencies for any page in the update.
    let mut in_degree: HashMap<u32, usize> = HashMap::new();
    // Initialize all page numbers in the update:
    update.iter().for_each(|pagenum| {
        let prev_val = in_degree.insert(*pagenum, 0);
        assert_eq!(prev_val, None);
    });
    // Check all rules, and for those applicable to the update, increment in_degree.
    for (&earlier_page, later_pages) in rules {
        for &later_page in later_pages {
            if update.contains(&earlier_page) && update.contains(&later_page) {
                // This rule is relevant to this update; we count the inbound edge.
                *in_degree.get_mut(&later_page).unwrap() += 1;
                edges.entry(earlier_page).or_default().insert(later_page);
            }
        }
    }
    // Note that in_degree only contains page numbers that occur in the update.

    // We start with the pages that have no inbound edges.
    let mut queue: VecDeque<u32> = in_degree
        .iter()
        .filter_map(|(&pagenum, &edges)| if edges == 0 { Some(pagenum) } else { None })
        .collect();

    let mut result = Vec::new();
    while let Some(pagenum) = queue.pop_front() {
        result.push(pagenum);
        if let Some(later_pages) = edges.get(&pagenum) {
            for &later_page in later_pages.iter() {
                let deg = in_degree.entry(later_page).or_default();
                *deg -= 1;
                if *deg == 0 {
                    // All of the later_page's dependencies are already in result,
                    // so we can queue it for processing.
                    queue.push_back(later_page);
                }
            }
        }
    }

    result
}

fn middle_page_number_of(update: &[u32]) -> u32 {
    update[update.len() / 2]
}

fn parse_rules(rules_input: &str) -> HashMap<u32, HashSet<u32>> {
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    for (earlier_page, later_page) in rules_input.lines().map(|line| {
        let [earlier_page, later_page]: &[u32] = &line
            .split('|')
            .map(|s| s.parse::<u32>().expect("page number"))
            .collect::<Vec<u32>>()
        else {
            panic!("expected pattern X|Y");
        };
        (*earlier_page, *later_page)
    }) {
        rules.entry(earlier_page).or_default().insert(later_page);
    }
    rules
}

fn parse_updates(updates: &str) -> Vec<Vec<u32>> {
    updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<u32>().expect("page number"))
                .collect::<Vec<u32>>()
        })
        .collect()
}
