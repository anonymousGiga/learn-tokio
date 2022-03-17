pub struct IterTest {
    pub inner: Vec<String>,
}

impl IterTest {
    fn find_and_update(&mut self) {
        for v in &mut self.inner {
            if v == "" {
                *v = "test".to_string()
            }
        }
    }
}
