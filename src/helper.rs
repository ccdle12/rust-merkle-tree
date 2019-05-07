/// Returns the size of the whole tree given, the number of leafs.
pub(crate) fn calculate_tree_size(leafs: u64) -> u64 {
    if leafs == 1 {
        return leafs;
    }

    return leafs + calculate_tree_size(leafs / 2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tree_size_1() {
        let node_count = calculate_tree_size(2);

        assert_eq!(node_count, 3);
    }

    #[test]
    fn node_calculation_2() {
        let node_count = calculate_tree_size(4);

        assert_eq!(node_count, 7);
    }

    #[test]
    fn node_calculation_3() {
        let node_count = calculate_tree_size(8);

        assert_eq!(node_count, 15);
    }
}
