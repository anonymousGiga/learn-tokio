use std::sync::Arc;
use tokio::sync::Mutex;

pub struct Inner {
    pub inner: String,
}

impl Inner {
    pub async fn print(&self) {
        println!("{}", self.inner);
    }
}

#[derive(Clone)]
pub struct Wrapper {
    pub inner: Arc<Mutex<Inner>>,
}

#[tokio::main]
pub async fn test_main() {
    let inner = Inner {
        inner: String::new(),
    };

    let wrapper = Wrapper {
        inner: Arc::new(Mutex::new(inner)),
    };

    let wrapper2 = wrapper.clone();
    tokio::spawn(async move {
        let inner = wrapper.inner.lock().await;
        inner.print().await;
    });

    println!("{}", wrapper2.inner.lock().await.inner);
}
