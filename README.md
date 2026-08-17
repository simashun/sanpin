Windows 向け beep 音つき ping ラッパー

## 概要
- Windows 向けOS標準 `ping` のラッパーです。
- デフォルトで ping 成功/失敗で異なるビープ音が鳴ります（`--nonbeep` で無効化可能）。


## 要件
- Windows (Beep を利用するため)
- Rust toolchain（ソースからビルドする場合）

## ビルド（ソースから）
```powershell
cargo build --release
# バイナリは target\release\sanpin.exe に生成されます
```

## 使い方
zip 解凍後、sanpin.exe をDOS窓から実行します。

基本構文:
```
sanpin [OPTIONS] <TARGET> 
```

主なオプション:
- `-c, --count <COUNT>`: 送信回数（連続は `-t/--continuous`）
- `-w, --timeout <MILLISECONDS>`: タイムアウト（ミリ秒）
- `-n, --nonbeep`: ビープを無効化（デフォルトでビープ有効）
- `-t, --continuous`: 継続的に ping を実行

例:
```powershell
# 127.0.0.1 に 5 回 ping を送り、到達で短いビープ（デフォルト）
> sanpin.exe -c 5 127.0.0.1

# 継続モード（Ctrl+C で停止）
> sanpin.exe -t 192.0.2.1 
> ^C  (Ctrl+cで停止)
>
```


## ライセンス
このプロジェクトはデュアルライセンスで配布されています: `MIT` または `Apache-2.0`（"MIT OR Apache-2.0"）。
どちらか一方のライセンスを選んで使用できます。詳細な条文はリポジトリルートの `LICENSE-MIT` と `LICENSE-APACHE`、および簡易説明の `LICENSE` を参照してください。

---
小さな変更や改善、バグ報告は Issue/PR で歓迎します。
