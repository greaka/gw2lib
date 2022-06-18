use std::future::Future;

std::thread_local! {
    static RT: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("build shell runtime");
}

pub(crate) fn block<F, T>(fut: F) -> T
where
    F: Future<Output = T>,
{
    RT.with(|rt| rt.block_on(fut))
}
