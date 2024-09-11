use std::collections::HashSet;

fn main() {
    let hello="hello";
    let world="world";
    let success="success";
    let unique_closure=|data:&str|{
        let mut vault=HashSet::new();
        data.to_lowercase().chars().find(|&char|{
            !vault.insert(char)
        }).is_none()
    };
    assert_eq!(false,unique_closure(hello));
    assert_eq!(true,unique_closure(world));
    assert_ne!(true,unique_closure(success));
    
}
