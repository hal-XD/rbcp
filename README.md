# rbcp
back up copy command by Rust

## How to use

- `bcp backup <src>`  
使用したユーザーのホームディレクトリ配下に~/.local/bcp/repositoryを作成し、そこにsrcのバックアップファイルを  
ファイル名_yyyymmdd_commentの形式でバックアップする

- `bcp restore <src> `  
指定したファイルをリストアする。

- `bcp show <src>`  
バックアップされているファイル一覧を表示する。
