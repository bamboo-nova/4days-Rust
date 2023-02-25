// 概要
// Option and Result types: optional valueやエラーハンドリング
// String: 所有してるデータで使われるデフォルトの文字列型
// Vec: 標準の拡張可能なベクトル
// HashMap: 設定可能なハッシュアルゴリズムによるハッシュマップ
// Box: ヒープで確保されたデータのための所有ポインタ
// Rc: ヒープで確保されたデータのための共有された参照カウントのポインタ

// 実際は、Rustは標準ライブラリのいくつかのレイヤーを持っている: core, alloc, and std.
// coreは最も基本的なtypeと関数が含まれていて、libcやallocator, OSに依存しない
// allocはグローバルなheap allocatorを必要とするタイプを含む(Vec, Box, Arc)
// よくcoreのみを使うが、allocも使う


// Option and Result
// Option<&T>は&Tと比較して、空間のオーバーヘッドがゼロ
// Resultはエラー処理に必須
fn main() {
    let numbers = vec![10, 20, 30];
    let first: Option<&i8> = numbers.first();
    // Someで出されるので、print文では:?にする必要があるので注意
    println!("first: {first:?}");

    // binary_searchはResult<usize, usize>を返す。
    // もし見つかったらResult::Okはその要素が見つかったインデックスを保持する
    // 見つからなかったらResult::Errが挿入されるべきインデックスを保持する
    let idx: Result<usize, usize> = numbers.binary_search(&10);
    println!("idx: {idx:?}");  // Ok(0)

    let idx: Result<usize, usize> = numbers.binary_search(&-10);
    println!("idx: {idx:?}");  // Err(0)
}
