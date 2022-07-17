use std::{future::Future, marker::Send};

std::thread_local! {
    static RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build shell runtime");
}

#[cfg(feature = "blocking")]
pub(crate) fn block<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    RT.with(|rt| rt.block_on(fut))
}

pub(crate) fn spawn<F: Future + Send + 'static>(task: F)
where
    <F as Future>::Output: Send + 'static,
{
    #[cfg(not(feature = "blocking"))]
    tokio::spawn(task);

    #[cfg(feature = "blocking")]
    std::thread::spawn(|| block(task));
}
