use point::Point;

fn main() {
    let p1=Point::new(2, 2);
    let p2=Point::new(4, 2);
    println!("{}",p1-p2)
}


mod point{
    use std::ops::{Mul, Sub};

    pub struct Point<T>
    
        where T:Copy
    {
        x:T,
        y:T
    }
    impl <T> Point<T>
    where T:Copy
    {
        pub fn new(x:T,y:T)->Self{
            Self { x, y }
        }
    }
    impl <T> Sub for Point<T> 
        where  T:Sub<Output = T>+Mul<Output = T>+Copy+Into<f64>,
    {
        type Output = f64;
        fn sub(self, rhs: Self) -> Self::Output {
            let dx=((self.x-rhs.x).into() as f64).powi(2);
            let dy=((self.y-rhs.y).into() as f64).powi(2);
            (dx+dy).sqrt()
        }
        
    }
}
