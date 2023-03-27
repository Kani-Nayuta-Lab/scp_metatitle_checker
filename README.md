# scp-metatitle-checker

SCP財団のページをスクレイピングして、SCP番号からメタタイトルを検索できるスクリプトです。

定期的にSCP財団のHPをチェックしている訳ではないので、気がついたら使えなくなっている可能性大。

## Requirement

- Cargo 1.52

## How to Build & Execute

- ビルド方法

```
$ cargo build
```

- scp-metatitle-checkerの実行

```
$ ./target/debug/scp-metatitle-checker
```

## Usage

SCP番号を入力するとSCP本部から指定の番号を検索してくれる

```
===========================
 - SCP Metatitle Checker -
===========================
Please input the item number of the SCP you want to check
If you want to check other branches SCP, Enter ':[BRANCH_NAME]'(Ex. 544:JP) after the item number
>>> 999
SCP-999 - くすぐりオバケ
```

SCP番号の後に":支部コード"を入力するとSCP海外支部から指定の番号を検索してくれる

```
===========================
 - SCP Metatitle Checker -
===========================
Please input the item number of the SCP you want to check
If you want to check other branches SCP, Enter ':[BRANCH_NAME]'(Ex. 544:JP) after the item number
>>> 105:JP
SCP-105-JPとは？私なりに調べてみました！
```