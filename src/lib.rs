// [Rustで(Either)モナドを作る](https://qiita.com/morikuni/items/8bdb39119379d60f628c)
// を実装してみた。

mod show;    // モジュールshow, monad, eitherの読み込み。
mod monad;   //   lib.rsでモジュールを読み込んでいないと
mod either;  //   同じディレクトリの他ソースコードでそのモジュールを使えない。

#[cfg(test)]
mod tests {
    use show::Show;
    use monad::*;
    use either::*;

    #[test]
    fn test_show(){
        let r: Either<String, i32> = Either::Right(100);
        println!("{}", r.show());

        let l: Either<String, i32> = Either::Left("Hello".to_string());
        println!("{}", l.show());
    }

    #[test]
    fn test_either(){
        let r: Either<String, i32> = Either::Right(100);
        let r2 = r.bind(|i|
            Monad::<i32, i32, Either<String, i32>>::return_(i*2i32)
        );
        println!("{}", r2.show());
        // Right(200)

        let l: Either<String, i32> = Either::Left("Hello".to_string());
        let l2 = l.bind(|i|
            Monad::<i32, i32, Either<String, i32>>::return_(i*2i32)
        );
        println!("{}", l2.show());
        // Left(Hello)
    }
}
