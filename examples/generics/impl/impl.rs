struct Tup (f64,);
struct GenTup<T>(T,);

// impl of Tup
impl Tup {
    fn value(&self) -> &f64 { &self.0 }
}

// impl of GenTup for a generic type `T`
impl <T> GenTup<T> {
    fn value(&self) -> &T { &self.0 }
}

fn main() {
    let x = Tup(3.0);
    let y = GenTup(3i32);
    
    println!("{}, {}", x.value(), y.value());
}

