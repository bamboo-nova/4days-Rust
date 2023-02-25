// moduleの項目はデフォルトでprivate
// 親と子どもの項目は常に可視化される
// つまり、ある項目がモジュールfooで表示されていれば、fooのすべての子孫で表示されている
mod outer {
    fn private() {
        println!("outer::private");
    }

    pub fn public() {
        println!("outer::public");
    }

    // pubにしないとmoduleがpubにならないのでエラーになる
    // pub(crate)で可視性を設定するのは良くあるパターンで、同一crate内からのアクセスだけを許す
    // あまり一般的ではありませんが、特定のパスに対して可視性を与えることもできる
    // いずれにせよ、祖先モジュール（とその子孫すべて）に対して可視性を付与する必要がある
    pub mod inner {
        fn private() {
            println!("outer::inner::private");
        }

        pub fn public() {
            println!("outer::inner::public");
            super::private();
        }
    }
}

fn main() {
    outer::inner::public();
    outer::public();
}
