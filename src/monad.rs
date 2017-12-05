pub trait Monad<A, B, MB> {
    fn return_(a: A) -> Self;
    fn bind<F>(self, f: F) -> MB where F: Fn(A) -> MB;
}

