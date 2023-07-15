// use core::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct Xorshift32 {
    pub state: u32,
}

#[derive(Debug, Clone)]
pub struct Xorshift64 {
    pub state: u64,
}

#[derive(Debug, Clone)]
pub struct Xorshift128 {
    pub state: u128,
}

#[derive(Debug, Clone)]
pub struct XorshiftSize {
    pub state: usize,
}

// fn now() -> u128 {
//     SystemTime::now()
//         .duration_since(UNIX_EPOCH)
//         .expect("if you are using this before 1970, what are you doing?")
//         .as_nanos()
// }

fn create_state() -> u128 {
    // now()
    let num = 42u8;
    let address = &num as *const u8;
    address as u128
}

macro_rules! impl_xorshift {
    ($obj: ident, $type: ident, 4) => {
        impl $obj {
            pub fn gen(&mut self) -> $type {
                self.state ^= self.state << 13;
                self.state ^= self.state >> 17;
                self.state ^= self.state << 5;
                self.state
            }
        }

        impl_xorshift!($obj, $type);
    };

    ($obj: ident, $type: ident, 8) => {
        impl $obj {
            pub fn gen(&mut self) -> $type {
                self.state ^= self.state << 13;
                self.state ^= self.state >> 7;
                self.state ^= self.state << 17;
                self.state
            }
        }
        impl_xorshift!($obj, $type);
    };

    ($obj: ident, $type: ident, 16) => {
        impl $obj {
            pub fn gen(&mut self) -> $type {
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

                self.state = (p | q | r | s) as $type;
                return self.state;
            }
        }
        impl_xorshift!($obj, $type);
    };

    ($obj: ident, $type: ident) => {
        impl $obj {
            pub fn new() -> $obj {
                $obj {
                    state: create_state() as $type,
                }
            }

            pub fn new_with_seed(state: $type) -> $obj {
                $obj { state: state }
            }

            pub fn gen_f64(&mut self) -> f64 {
                self.gen() as f64 / core::$type::MAX as f64
            }

            pub fn gen_f32(&mut self) -> f32 {
                self.gen() as f32 / core::$type::MAX as f32
            }
        }

        impl Iterator for $obj {
            type Item = $type;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.gen())
            }
        }
    };
}

#[cfg(target_pointer_width = "32")]
impl_xorshift!(XorshiftSize, usize, 4);
#[cfg(target_pointer_width = "64")]
impl_xorshift!(XorshiftSize, usize, 8);
impl_xorshift!(Xorshift32, u32, 4);
impl_xorshift!(Xorshift64, u64, 8);
impl_xorshift!(Xorshift128, u128, 16);

#[cfg(test)]
mod tests {
    use super::{Xorshift128, Xorshift32, Xorshift64, XorshiftSize};
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

    #[test]
    fn xorshiftsize_gen_f64() {
        let mut random = XorshiftSize::new_with_seed(core::usize::MAX - 50);
        assert_eq!(0.000000002945747915668491, random.gen_f64());
        assert_eq!(0.18730243001117602, random.gen_f64());
    }

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
