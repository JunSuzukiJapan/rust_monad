use show::*;
use either::*;

pub trait Functor<A, B, FB> {
    fn fmap<F>(self, f: F) -> FB where F: Fn(A) -> B;
}

impl<E, A, B> Functor<A, B, Either<E, B>> for Either<E, A> {
    fn fmap<F>(self, f: F) -> Either<E, B> where F: Fn(A) -> B {
        match self {
            Either::Right(a) => Either::Right(f(a)),
            Either::Left(e) => Either::Left(e),
        }
    }
}

pub fn decorate<FA: Functor<A, String, FS>, FS: Functor<String, A, FA>, A: Show>(fa: FA) -> FS {
    fa.fmap(|a| format!("***{}***", a.show()))
}
