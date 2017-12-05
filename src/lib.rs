// [Rustで(Either)モナドを作る](https://qiita.com/morikuni/items/8bdb39119379d60f628c)
// を実装してみた。

mod show;    // モジュール show, functor, monad, either の読み込み。
mod functor; //
mod monad;   //   lib.rsでモジュールを読み込んでいないと
mod either;  //   同じディレクトリの他ソースコードでそのモジュールを使えない。

#[cfg(test)]
mod tests {
    use show::Show;
    use functor::*;
    use monad::*;
    use either::*;

    #[test]
    fn test_show(){
        let r: Either<String, i32> = Either::Right(100);
        assert_eq!("Right(100)", r.show());

        let l: Either<String, i32> = Either::Left("Hello".to_string());
        assert_eq!("Left(Hello)", l.show());
    }

    #[test]
    fn test_either(){
        let r: Either<String, i32> = Either::Right(100);
        let r2 = r.bind(|i|
            Monad::<i32, i32, Either<String, i32>>::return_(i*2i32)
        );
        assert_eq!("Right(200)", r2.show());

        let l: Either<String, i32> = Either::Left("Hello".to_string());
        let l2 = l.bind(|i|
            Monad::<i32, i32, Either<String, i32>>::return_(i*2i32)
        );
        assert_eq!("Left(Hello)", l2.show());
    }

    #[test]
    fn test_div_by_3(){        
        fn div_by_3(i: i32) -> Either<String, i32> {
            println!("div {}", i);
            if i%3 == 0 {
                Either::Right(i/3)
            } else {
                Either::Left(format!("'div_by_3' failed when {}", i))
            }
        }

        fn try_div_by_3_twice(e: Either<String, i32>) -> Either<String, String> {
            e.bind(|i1|
                div_by_3(i1).bind(|i2|
                    div_by_3(i2).bind(|i3|
                        return_(format!("{} -> {} -> {}", i1, i2, i3))
            )))
        }

        let a = Either::Right(9);
        let b = try_div_by_3_twice(a);
        assert_eq!("Right(9 -> 3 -> 1)", b.show());

        let a = Either::Right(2);
        let b = try_div_by_3_twice(a);
        assert_eq!("Left('div_by_3' failed when 2)", b.show());

        let a = Either::Left("I'm Left!".to_string());
        let b = try_div_by_3_twice(a);
        assert_eq!("Left(I'm Left!)", b.show());
    }

    #[test]
    fn test_decorate(){
        fn div_by_3(i: i32) -> Either<String, i32> {
            println!("div {}", i);
            if i%3 == 0 {
                Either::Right(i/3)
            } else {
                Either::Left(format!("`div_by_3` failed when {}", i))
            }
        }

        fn try_div_by_3_twice(e: Either<String, i32>) -> Either<String, String> {
            e.bind(|i1|
                div_by_3(i1).bind(|i2|
                    div_by_3(i2).bind(|i3|
                        return_(format!("{} -> {} -> {}", i1, i2, i3))
            )))
        }

        let a = Either::Right(9);
        let b = try_div_by_3_twice(a);
        let c = decorate(b);
        assert_eq!("Right(***9 -> 3 -> 1***)", c.show());

        let a = Either::Right(2);
        let b = try_div_by_3_twice(a);
        let c = decorate(b);
        assert_eq!("Left(`div_by_3` failed when 2)", c.show());

        let a = Either::Left("I'm Left!".to_string());
        let b = try_div_by_3_twice(a);
        let c = decorate(b);
        assert_eq!("Left(I'm Left!)", c.show());
    }
}
