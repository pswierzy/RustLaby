struct Tablica<T, const N: usize> {
    tab: [T; N]
}

impl <T: std::cmp::PartialOrd + Copy, const N: usize>Tablica<T, N> {
    fn max(&self) -> Option<T>{
        if self.tab.is_empty() {
            return None;
        }

        let mut maks = self.tab[0];
        for &num in &self.tab[1..] {
            if num > maks{
                maks = num;
            }
        }
        Some(maks)
    }
}

impl <T: std::cmp::PartialOrd + Copy + Default + Into<f32> + std::ops::AddAssign, const N: usize>Tablica<T, N> {
    fn mean(&self) -> Option<f32> {
        if self.tab.is_empty() {
            return None;
        }

        let mut sum: T = T::default();
        for &num in &self.tab {
            sum += num;
        }
        let float_sum: f32 = sum.into();
        Some(float_sum / self.tab.len() as f32)
    }
}

fn main() {

    let nums = Tablica{tab: [4, 5, 2, 8, 8, 10, 2, 1, -1, -20]};
    println!("{:?}", &nums.max());
    println!("{:?}", &nums.mean())

}