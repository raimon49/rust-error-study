use std::error::Error;
use std::io::{Write, stderr};

fn print_error(mut err: &dyn Error) {
    let _ = writeln!(stderr(), "error: {}", err);
    while let Some(cause) = err.source() {
        let _ = writeln!(stderr(), "caused by: {}", cause);
        err = cause;
    }
}

fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
fn main() {
    let _crew_get_items = pirate_share(100, 10);

    // thread 'main' panicked at 'attempt to divide by zero' が起きる
    // 実行時に「RUST_BACKTRACE=1 cargo run」と環境変数をセットするとスタックトレースが表示される
    // JavaのRuntimeExceptionに近いエラー
    let result = std::panic::catch_unwind(|| {
        // catch_unwind()でpanicを補足できるが、すべてのpanicをcatchできる訳ではない
        // try-catchのように使うことは推奨されない
        let _will_error_items = pirate_share(100, 0);
    });

    let _err = result.as_ref().err(); // Result<T, E>から成功値/エラー値を借用しResult<&T, &E>として処理 ここではOption<&E>を取り出している
    assert!(result.is_err());

    // すべてのエラーと結果を表現する型を定義
    type GenError = Box<dyn std::error::Error>;
    type GenResult<T> = Result<T, GenError>;

    let io_error = std::io::Error::new(
        std::io::ErrorKind::Other, "timed out");
    let _my_error = GenError::from(io_error); // Fromトレイトのfrom()メソッドでエラーを変換
}
