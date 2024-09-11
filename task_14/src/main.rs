use std::{any::Any, sync::Arc};


fn define_type<T>(value:&dyn Any)->Option<&T>
    where T:Any+'static
{
        value.downcast_ref::<T>()
}

#[derive(Debug,PartialEq)]
struct Hello;


fn main() {
    let s1=String::from("asdasd");
    let s2=Hello;
    assert_eq!(None,define_type::<i32>(&s1));
    assert_eq!(Some(&Hello),define_type::<Hello>(&s2));
}