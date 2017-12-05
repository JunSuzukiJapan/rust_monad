use functor::*;

/*
pub trait Monad<A, B, MB> {
    fn return_(a: A) -> Self;
    fn bind<F>(self, f: F) -> MB where F: Fn(A) -> MB;
}
*/
pub trait Monad<A, B, MB>: Functor<A, B, MB> {
    fn return_(a: A) -> Self;
    fn bind<F>(self, f: F) -> MB where F: Fn(A) -> MB;
}

pub fn return_<MA, A, M: Monad<A, A, MA>>(a: A) -> M {
    M::return_(a)
}