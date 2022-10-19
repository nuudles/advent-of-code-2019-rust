use std::fmt::Debug;

pub trait SelfPrint {
    fn print(self) -> Self;
}

impl <T: Debug> SelfPrint for T {
    fn print(self) -> Self {
        println!("{:?}", self);
        self
    }
}
