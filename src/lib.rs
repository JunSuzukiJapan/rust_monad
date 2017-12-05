mod show;    // モジュールshowの読み込み
mod either;  // モジュールeitherの読み込み

#[cfg(test)]
mod tests {
    use show::Show;
    use either::*;

    #[test]
    fn test_show(){
        let r: Either<String, i32> = Either::Right(100);
        println!("{}", r.show());

        let l: Either<String, i32> = Either::Left("Hello".to_string());
        println!("{}", l.show());
    }
}
