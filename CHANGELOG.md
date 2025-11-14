# CHANGELOG

## v0.1.1 (2025-11-14)

- Fix: Ping 出力の UTF-8 デコードエラーを解消（バイト読み取り + Shift-JIS デコードに変更）
- Feature: リアルタイム出力モードを追加（出力を逐次表示しつつ `full_output` を蓄積）
- Feature: 成功／失敗を通知するビープ音を追加（`--beep` オプション）
- Fix: `ping -t`（継続）オプションの実装と表示改善（`-t/--continuous`）
- Fix: 空の stderr で意味のないエラーメッセージになる問題を修正（`full_output` にフォールバック）
- Fix: 「一般エラー。」など日本語の失敗メッセージを失敗扱いにしてビープを鳴らすようにした
- UX: Ctrl-C のハンドリング改善（親プロセスの余分な終了診断を抑制）
- Build: release ビルドと Windows 用パッケージを作成

----

今後の予定:

- Git タグ作成とコミット、GitHub リリース作成
- CHANGELOG の詳細追記（必要に応じて）
