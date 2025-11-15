pub trait CacheInterface {
    type T;

    fn connect(host: String, port: String, db: String) -> Self::T where Self: Sized;
    fn new() -> Self where Self: Sized;
}

pub trait CacheFactory: Sized {
    fn get() -> Result<Self, String>;
}