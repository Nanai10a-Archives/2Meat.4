# 2Meat-4th

[![codecov](https://codecov.io/gh/Nanai10a/2Meat-4th/branch/master/graph/badge.svg?token=TQFPUYJE0E)](https://codecov.io/gh/Nanai10a/2Meat-4th)

:sparkles: ***master status*** :sparkles:  
![build](https://github.com/Nanai10a/2Meat-4th/workflows/build/badge.svg?branch=master)
![grcov](https://github.com/Nanai10a/2Meat-4th/workflows/grcov/badge.svg?branch=master)
![md_lint](https://github.com/Nanai10a/2Meat-4th/workflows/md_lint/badge.svg?branch=master)

:hammer_and_wrench: ***development status*** :hammer_and_wrench:  
![build](https://github.com/Nanai10a/2Meat-4th/workflows/build/badge.svg?branch=development)
![grcov](https://github.com/Nanai10a/2Meat-4th/workflows/grcov/badge.svg?branch=development)
![md_lint](https://github.com/Nanai10a/2Meat-4th/workflows/md_lint/badge.svg?branch=development)


誰が4世代目まで書くと予想したでしょう, そう私は予想していませんでした, 以上.

### 概要

2MeatのRust-rewriteです.  
実はこの4thにも既にarchiveされたbranchが存在しますが気にしてはいけません.

## 規約

- in working
    - `cargo check`をある程度の頻度でかけよう
- before commit
    - `cargo fmt`をかけろ
    - `cargo build`で最終確認して
- on commit
    - [gitmoji](https://gitmoji.carloscuesta.me/)をprefixに使用する
        - commitは1 mojiで表せる単位で分割する
    - prefixとコメント本文の間にはspaceをはさみましょう
        - ex) `:tada: こめんと`  
          preview) :tada: こめんと
    - 末尾には句点をつけましょう.
    - signatureはして.
- others
    - 日本語の句読点を以下のように置き換える.
        - `、`→`, ` (spaceを入れることに注意)
        - `。`→`.` (末尾spaceは消す)
    - ファイル末尾改行を忘れずに.
    - 基本的に日本語でコメントを書きましょう.
    - have fun!

## こま胡麻things

#### 意識的依存crate

- selenity
    - Discord API
- tokio
    - 非同期処理
- mongodb
    - DB
- dotenv
    - env::var

---

その他仕様変更等は頻繁に起こります.  
ご承知おきください.
