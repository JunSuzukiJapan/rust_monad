use show::*;  // lib.rsで'mod show’しているので、ここでuseすることができる。
use monad::*; // 同上

pub enum Either<E, A> {
    Left(E),
    Right(A),
}

impl<E: Show, A: Show> Show for Either<E, A> {
    fn show(self) -> String {
        match self {
            Either::Right(a) => format!("Right({})", a.show()),
            Either::Left(e) => format!("Left({})", e.show()),
        }
    }
}

impl<E, A, B> Monad<A, B, Either<E, B>> for Either<E, A> {
    fn return_(a: A) -> Either<E, A> {
        Either::Right(a)
    }

    fn bind<F>(self, f: F) -> Either<E, B> where F: Fn(A) -> Either<E, B> {
        match self {
            Either::Right(a) => f(a),
            Either::Left(e) => Either::Left(e),
        }
    }
}