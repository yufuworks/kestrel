# Kestrel

Raspberry Pi の状態をリアルタイムで監視する macOS デスクトップアプリ。

> **このプロジェクトは習作です。**  
> Rust・Tauri・SSH・React+TypeScript を実際に手を動かして学ぶことが目的です。

## 概要

- SSH 経由で Pi のメトリクス（CPU・メモリ・温度・ディスク・稼働時間）を取得
- Tauri（Rust バックエンド + React フロントエンド）で構築

## 学習テーマ

Claude Code を活用して開発しています。
コード生成より解説・議論を中心に教師役として使い、実装は自分で行うサポートモードを基本としています。
生成したコードは必ず内容を理解してからマージする方針です。

| 技術 | 内容 |
|------|------|
| Rust | 所有権・エラーハンドリング・非同期 |
| Tauri | Rust ↔ WebView IPC |
| SSH | ssh2 クレートによるコマンド実行 |
| React + TypeScript | UI 構築 |

## 開発環境のセットアップ

（開発手順を後で記載）

## フェーズ

- **Phase 1（現在）**：SSH 経由の監視アプリ
- **Phase 2**：Pi 側エージェント（kestreld）
- **Phase 3**：SNS アイデンティティハブ

---
authorised this readme by claude code
