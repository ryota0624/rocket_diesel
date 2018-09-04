cargo install diesel_cli

cargo install watch
cargo watch -x "run"

https://qiita.com/mochizukikotaro/items/84204d5c46b67c9b74f4

http://diesel.rs/guides/getting-started/

https://keens.github.io/blog/2017/12/16/dieselshounetashuu/


curl -H "Accept: application/json" -H "Content-type: application/json" -X POST -d "{\"title\": \"feaf\", \"body\": \"bbofa\"}" http://localhost:8000/posts

https://qnighy.hatenablog.com/entry/2017/06/01/070000

## つまったところ

### modelsのderiveは注意
belongs_to を使うときは所持、所持先のどちらも`Identifiable`は持つ必要がある

### schema.rs
option<Text>など、いくつかの定義が使えない。
```` ^^^^ the trait `diesel::deserialize::FromSql<diesel::sql_types::Nullable<diesel::sql_types::Text>, _>` is not implemented for `*const str````

### 作成系モデル
- 作成系のstructの定義場所にはそのテーブル用のschema.rsを見るようにインポートしないとx

### db使う箇所
- schemaとmodelをインポートする