fn pirate_share(total: u64, crew_size: usize) -> u64 {
    let half = total / 2;
    half / crew_size as u64
}
fn main() {
    let _crew_get_items = pirate_share(100, 10);

    // thread 'main' panicked at 'attempt to divide by zero' が起きる
    // 実行時に「RUST_BACKTRACE=1 cargo run」と環境変数をセットするとスタックトレースが表示される
    // JavaのRuntimeExceptionに近いエラー
    let _will_error_items = pirate_share(100, 0);
}
