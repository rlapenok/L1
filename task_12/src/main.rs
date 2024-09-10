use std::collections::HashSet;

fn main() {
    let multitude1=(0..=5).chain(12..=56).collect::<HashSet<i32>>();
    let multitude2=(3..=5).chain(30..=56).collect::<HashSet<i32>>();
    let intersect=multitude1.intersection(&multitude2).collect::<HashSet<&i32>>();
    println!("{:?}",intersect)

}
