use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Frame(PhantomData<()>);

impl Frame {
    pub fn new() -> Self {
        Frame(PhantomData)
    }

    pub fn get_mass(&self) -> i32 {
        1000
    }
}
