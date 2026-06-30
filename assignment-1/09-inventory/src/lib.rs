use std::collections::HashMap;

pub fn restock(
    inventory: Vec<(String, u32)>,
    more: Vec<(String, u32)>,
) -> Vec<(String, u32)> {
    let mut map: HashMap<String, u32> = HashMap::new();

    for (name, qty) in inventory {
        *map.entry(name).or_insert(0) += qty;
    }

    for (name, qty) in more {
        *map.entry(name).or_insert(0) += qty;
    }

    map.into_iter().collect()
}

pub fn summary(inventory: &[(String, u32)]) -> String {
    let items = inventory.len();
    let mut units = 0;

    for (_, qty) in inventory {
        units += qty;
    }

    format!("{} items, {} units", items, units)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(name: &str, qty: u32) -> (String, u32) {
        (name.to_string(), qty)
    }

    fn sorted(mut v: Vec<(String, u32)>) -> Vec<(String, u32)> {
        v.sort_by(|a, b| a.0.cmp(&b.0));
        v
    }

    #[test]
    fn restock_combines_duplicate_names() {
        let merged = restock(vec![item("apple", 3)], vec![item("apple", 2)]);
        assert_eq!(sorted(merged), vec![item("apple", 5)]);
    }

    #[test]
    fn restock_keeps_disjoint_items() {
        let merged = restock(vec![item("apple", 3)], vec![item("banana", 2)]);
        assert_eq!(sorted(merged), vec![item("apple", 3), item("banana", 2)]);
    }

    #[test]
    fn restock_both_empty() {
        let merged = restock(vec![], vec![]);
        assert_eq!(merged, Vec::<(String, u32)>::new());
    }

    #[test]
    fn restock_one_empty() {
        let merged = restock(vec![item("apple", 3)], vec![]);
        assert_eq!(sorted(merged), vec![item("apple", 3)]);
    }

    #[test]
    fn restock_mixed_overlap() {
        let merged = restock(
            vec![item("apple", 3), item("banana", 1)],
            vec![item("banana", 2), item("cherry", 4)],
        );
        assert_eq!(
            sorted(merged),
            vec![item("apple", 3), item("banana", 3), item("cherry", 4)]
        );
    }

    #[test]
    fn summary_three_items_seventeen_units() {
        let inv = vec![item("apple", 3), item("banana", 7), item("cherry", 7)];
        assert_eq!(summary(&inv), "3 items, 17 units");
    }

    #[test]
    fn summary_empty_inventory() {
        let inv: Vec<(String, u32)> = vec![];
        assert_eq!(summary(&inv), "0 items, 0 units");
    }

    #[test]
    fn borrow_then_consume_compiles() {
        let inv = vec![item("apple", 3)];
        let more = vec![item("apple", 2)];
        let _ = summary(&inv);
        let merged = restock(inv, more);
        assert_eq!(sorted(merged), vec![item("apple", 5)]);
    }
}