use show::*;

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