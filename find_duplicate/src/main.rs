use std::collections::HashSet;

fn contains_duplicates(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for num in nums {
        if !seen.insert(num) {
            return true; // Encontrou uma duplicata
        }
    }
    false // Não encontrou duplicatas
}

fn main() {
    let nums1 = vec![1, 2, 3, 4, 5];
    let nums2 = vec![1, 2, 2, 3, 4];

    let has_duplicates_nums1 = contains_duplicates(nums1);
    let has_duplicates_nums2 = contains_duplicates(nums2);

    println!("Array 1 contém duplicatas? {}", if has_duplicates_nums1 { "Sim" } else { "Não" });
    println!("Array 2 contém duplicatas? {}", if has_duplicates_nums2 { "Sim" } else { "Não" });
}
