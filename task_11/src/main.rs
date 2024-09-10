use std::collections::BTreeMap;

fn main() {
    let input_data = vec![-25.4, -27.0, 13.0, 19.0, 15.5, 24.5, -21.0, 32.5,31.5,18.2,30.0,40.0];
    let mut output_data=BTreeMap::new();
    input_data.into_iter().for_each(|num|{
        //-25.4=> -2.54=>upper=-30.0
        let lower=(num / 10.0 as f64).floor()*10.0;
        let upper=lower+10.0;
        let interval=(lower.to_string(),upper.to_string());
        output_data.entry(interval).or_insert_with(Vec::new).push(num)
    });
    output_data.into_iter().for_each(|(interval,temps)|{
        println!("[{}, {}): {:?}", interval.0, interval.1, temps);
    })
}
