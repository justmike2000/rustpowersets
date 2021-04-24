use std::collections::HashSet;


fn find_power_sets(power_sets: HashSet<Vec<String>>) -> HashSet<Vec<String>> {
    let mut current_sets: HashSet<Vec<String>> = HashSet::new();
    for i in power_sets.iter() {
        for c in current_sets.clone().iter() {
            let mut combine = vec![];
            combine.append(&mut i.clone());
            combine.append(&mut c.clone());
            current_sets.insert(combine);
        }
        current_sets.insert(i.to_vec());
    }
    current_sets
}

fn main() {

    let our_set: HashSet<_> = [vec![],
                               vec!['a'.to_string()],
                               vec!['b'.to_string()],
                               vec!['c'.to_string()]].iter().cloned().collect();

    let powersets = find_power_sets(our_set);
    
    println!("{:?}", powersets);

}
