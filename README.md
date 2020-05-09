## ssfd-rs

ローカル環境内で静的ファイル用HTTPサーバーを立てるためのRust製のCLIツールです。

## インストール

```
$ cargo install --git https://github.com/didy-kpn/ssfd-rs.git
```

## 使い方

### サーバー起動

以下のコマンドを実行すると、実行時のディレクトリがルートとして設定されて、サーバーが立ち上げます。

デフォルトアドレスは 0.0.0.0:8080 です。

```
$ ssfd
```

### オプションの説明

ホスト名やルートディレクトリはオプションを指定する事で変更できます。

オプションについては以下のコマンドを実行してください。

```
$ ssfd --help
```

#### オプション使用例

```
$ ssfd -h localhost -p 8080 -d ./static_index -b base-path
```

## ライセンス

このプロジェクトは MIT ライセンスの元にライセンスされています。

詳細は[LICENSE](https://github.com/didy-kpn/ssfd/blob/master/LICENSE)をご覧ください。
