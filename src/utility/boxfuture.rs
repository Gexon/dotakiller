use futures::future::{Future, FutureResult, IntoFuture, Lazy};

pub type _BoxFuture<T, E> = Box<Future<Item = T, Error = E> + Send>;

pub trait Boxable {
    fn to_boxed(self) -> Box<Self>;
}

impl<T, E> Boxable for FutureResult<T, E> {
    fn to_boxed(self) -> Box<Self>
        where
            Self: Sized,
    {
        Box::new(self)
    }
}

impl<F, R: IntoFuture> Boxable for Lazy<F, R> {
    fn to_boxed(self) -> Box<Self>
        where
            Self: Sized,
    {
        Box::new(self)
    }
}