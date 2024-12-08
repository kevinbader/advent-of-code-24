use std::{
    collections::{HashMap, HashSet},
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
    let mut midpoint_sum = 0;
    for update in updates {
        if is_correctly_ordered(&update, &rules) {
            midpoint_sum += middle_page_number_of(&update);
        }
    }

    println!("Sum of middle page numbers of correctly ordered updates: {midpoint_sum}");
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
