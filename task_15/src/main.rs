fn hand_made_quick_sort<T: Ord>(data: &mut [T]) {
    if data.len() <= 1 {
        return;
    }
    let get_pivot = |data: &mut [T]| {
        let len = data.len();
        let pivot_index = len / 2;
        data.swap(pivot_index, len - 1);
        let mut i = 0;

        for j in 0..len - 1 {
            if data[j] <= data[len - 1] {
                data.swap(i, j);
                i += 1;
            }
        }
        data.swap(i, len - 1);
        i
    };
    let pivot = get_pivot(data);
    hand_made_quick_sort(&mut data[0..pivot]);
    hand_made_quick_sort(&mut data[pivot + 1..]);
}

fn main() {
    let mut data = (0..10).chain(20..30).chain(43..500).collect::<Vec<i32>>();
    data.reverse();
    data.sort_unstable();
    println!("{:?}", data);
    let mut data2 = vec![2, 4, 8, 18, 20, 46, 123, 120, 87, 64, 42];
    hand_made_quick_sort(&mut data2);
    println!("{:?}", data2)
}
