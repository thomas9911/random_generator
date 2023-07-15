use std::time::{SystemTime, UNIX_EPOCH};

pub type Xorshift32 = Xorshift<u32>;
pub type Xorshift64 = Xorshift<u64>;
pub type Xorshift128 = Xorshift<u128>;
pub type XorshiftSize = Xorshift<usize>;

pub trait RandomGeneratorWithSeed<T> {
    fn new_with_seed(input: T) -> Self;
    fn new() -> Self;
}

pub trait RandomGeneratable<T> {
    fn gen(&mut self) -> T;
    fn max_size(&self) -> T;
}

pub struct Xorshift<T> {
    pub state: T,
}

impl RandomGeneratable<u32> for Xorshift<u32> {
    fn gen(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }

    fn max_size(&self) -> u32 {
        u32::max_value()
    }
}

impl RandomGeneratable<u64> for Xorshift<u64> {
    fn gen(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn max_size(&self) -> u64 {
        u64::max_value()
    }
}

impl RandomGeneratable<usize> for Xorshift<usize> {
    #[cfg(target_pointer_width = "32")]
    fn gen(&mut self) -> usize {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
    #[cfg(target_pointer_width = "64")]
    fn gen(&mut self) -> usize {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }

    fn max_size(&self) -> usize {
        usize::max_value()
    }
}

impl RandomGeneratable<u128> for Xorshift<u128> {
    fn gen(&mut self) -> u128 {
        let mut a = self.state.wrapping_shl(0).wrapping_shr(96) as u32;
        let mut b = self.state.wrapping_shl(32).wrapping_shr(96) as u32;
        let mut c = self.state.wrapping_shl(64).wrapping_shr(96) as u32;
        let mut d = self.state.wrapping_shl(96).wrapping_shr(96) as u32;

        let mut t = d;
        let s = a;

        d = c;
        c = b;
        b = a;

        t ^= t << 11;
        t ^= t >> 8;
        a = t ^ s ^ (s >> 19);

        let p = (a as u128).wrapping_shl(96);
        let q = (b as u128).wrapping_shl(64);
        let r = (c as u128).wrapping_shl(32);
        let s = (d as u128).wrapping_shl(0);

        self.state = p | q | r | s;
        return self.state;
    }

    fn max_size(&self) -> u128 {
        u128::max_value()
    }
}

macro_rules! impl_random_generator {
    ($type: ty) => {
        impl RandomGeneratorWithSeed<$type> for Xorshift<$type> {
            fn new_with_seed(input: $type) -> Self {
                Self { state: input }
            }

            fn new() -> Self {
                Self::new_with_seed(create_state() as $type)
            }
        }
    };
}

macro_rules! impl_iterator {
    ($obj: ident, $type: ty) => {
        impl Iterator for $obj {
            type Item = $type;
            fn next(&mut self) -> Option<Self::Item> {
                Some(self.gen())
            }
        }
    };
}

impl_random_generator!(u32);
impl_random_generator!(usize);
impl_random_generator!(u64);
impl_random_generator!(u128);

impl_iterator!(Xorshift32, u32);
impl_iterator!(Xorshift64, u64);
impl_iterator!(Xorshift128, u128);
impl_iterator!(XorshiftSize, usize);

fn now() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("if you are using this before 1970, what are you doing?")
        .as_nanos()
}

fn create_state() -> u128 {
    now()
}

#[cfg(test)]
mod tests {
    use super::{
        RandomGeneratable, RandomGeneratorWithSeed, Xorshift128, Xorshift32, Xorshift64,
        XorshiftSize,
    };
    #[test]
    fn xorshift32_with_empty_state_does_not_work() {
        let mut random = Xorshift32 { state: 0 };
        assert_eq!(0, random.gen());
        assert_eq!(0, random.gen());
    }

    #[test]
    fn xorshift32_returns_not_nill() {
        let mut random = Xorshift32::new();
        assert_ne!(0, random.gen());
        assert_ne!(0, random.gen());
    }

    #[test]
    fn xorshift32_with_seeds_returns_predictability() {
        let mut random = Xorshift32::new_with_seed(123456);
        assert_eq!(3044438244, random.gen());
        assert_eq!(372467569, random.gen());
    }

    #[test]
    fn xorshift64_with_empty_state_does_not_work() {
        let mut random = Xorshift64 { state: 0 };
        assert_eq!(0, random.gen());
        assert_eq!(0, random.gen());
    }

    #[test]
    fn xorshift64_returns_not_nill() {
        let mut random = Xorshift64::new();
        assert_ne!(0, random.gen());
        assert_ne!(0, random.gen());
    }

    #[test]
    fn xorshift64_with_seeds_returns_predictability() {
        let mut random = Xorshift64::new_with_seed(123456123456);
        assert_eq!(2433743613504487332, random.gen());
        assert_eq!(5448107948928174171, random.gen());
    }

    #[test]
    fn xorshiftsize_with_empty_state_does_not_work() {
        let mut random = XorshiftSize { state: 0 };
        assert_eq!(0, random.gen());
        assert_eq!(0, random.gen());
    }

    #[test]
    fn xorshiftsize_returns_not_nill() {
        let mut random = XorshiftSize::new();
        assert_ne!(0, random.gen());
        assert_ne!(0, random.gen());
    }

    #[test]
    fn xorshiftsize_with_seeds_returns_predictability() {
        let mut random = XorshiftSize::new_with_seed(123456);
        assert_eq!(132366047211908, random.gen());
        assert_eq!(8689640596666966631, random.gen());
    }

    // #[test]
    // fn xorshiftsize_gen_f64() {
    //     let mut random = XorshiftSize::new_with_seed(std::usize::MAX - 50);
    //     assert_eq!(0.000000002945747915668491, random.gen_f64());
    //     assert_eq!(0.18730243001117602, random.gen_f64());
    // }

    #[test]
    fn xorshift128_with_empty_state_does_not_work() {
        let mut random = Xorshift128 { state: 0 };
        assert_eq!(0, random.gen());
        assert_eq!(0, random.gen());
    }

    #[test]
    fn xorshift128_returns_not_nill() {
        let mut random = Xorshift128::new();
        assert_ne!(0, random.gen());
        assert_ne!(0, random.gen());
    }

    #[test]
    fn xorshift128_with_seeds_returns_predictability() {
        let mut random = Xorshift128::new_with_seed(123456123456123456);
        assert_eq!(74896665221921414396857867002210327216, random.gen());
        assert_eq!(188163241731024871939078947111379664896, random.gen());
    }

    #[test]
    fn impl_iterator_works() {
        let random = XorshiftSize::new();

        assert_eq!(random.take(10).size_hint(), (0, Some(10)))
    }
}
