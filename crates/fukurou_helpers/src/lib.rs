lazy_static::lazy_static! {
    static ref THREAD_POOL: rayon::ThreadPool = rayon::ThreadPoolBuilder::new().build().unwrap();
}

///
pub fn run_threaded<Func>(callback: Func)
where
    Func: FnOnce() + Send + 'static,
{
    THREAD_POOL.spawn(callback);
}
