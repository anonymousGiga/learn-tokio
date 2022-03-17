pub trait TestTrait {
    type Error;
}

impl TestTrait for () {
    type Error = ();
}

pub struct Test<'a, T: TestTrait> {
    inner: Vec<&'a T>,
}

impl<'a, T: TestTrait> Test<'a, T> {
    pub fn add(&mut self, v: &'a T) {
        self.inner.push(v);
    }
}
