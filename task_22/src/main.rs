fn main() {
    let index=2;
   let mut data=vec![1,2,3,4,5,6];
   data.remove(index);
   assert_ne!(3,data[index])
}
