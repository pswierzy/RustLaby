struct Pair<T> {
    x: T,
    y: T
}

impl <T> Pair<T> {
    fn extract_x(self) -> T {
        self.x
    }
}

impl <T: std::cmp::PartialOrd + Copy> Pair<T> {
    fn bigger(&self) -> T {
        if self.x > self.y {
            self.x
        } else {
            self.y
        }
    }
}

fn main() {
    let _pi = Pair{x : 5, y : 3};
 
    let pf = Pair {x: 15.67f64, y : 12.0f64};

    println!("{}", pf.bigger())
}