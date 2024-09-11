fn hand_made_binary_search<T:Ord>(data:&[T],target:&T)->Option<usize>{

    if data.is_empty(){
        return None
    }
    let mut start=0;
    let mut end=data.len()-1;
    while start<=end {
        let mid=start+(end-start)/2;
        //[1.2.3.4.5.6.7.8.9.10]
        if &data[mid]==target{
            return Some(mid)
        }else if &data[mid]<target {
            start=mid+1;
            
        }else {
            end=mid-1
        }
        
    }
    None
}



fn main() {
   let data=(1..20).chain(35..40).collect::<Vec<i32>>();
   assert_eq!(false,data.binary_search(&20).is_ok());
   assert_eq!(true,hand_made_binary_search(&data, &35).is_some());
}
