2015年ではmod.rsを名前ディレクトリの下に作る方式だったが、2018年から下記のように直接「名前.rs」にするように変更になっている。

```
src/
├── main.rs
├── top_module.rs
└── top_module/
    └── sub_module.rs
```

これは、例えば、あるモジュールのテストを、Goの規約と同じように、some_module_test.rsという名前のファイルに置きたい場合に便利(mod.rsだとIDEが判別困難だったり色々な弊害があった)