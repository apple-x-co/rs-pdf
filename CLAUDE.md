# rs-pdf プロジェクトガイド

このドキュメントは、AI アシスタント（Claude）がこのプロジェクトを理解し、効果的に支援するためのガイドです。

## プロジェクト概要

**rs-pdf** は、JSON 設定ファイルから PDF ドキュメントを生成する Rust 製の CLI ツールです。

### 主要な特徴

- JSON による宣言的な PDF レイアウト定義
- 静的ページと動的ページの 2 種類のページタイプをサポート
- 動的ページでは自動ページ分割機能を提供
- Flexbox ライクなレスポンシブレイアウトシステム
- 日本語を含む CJK フォントのサポート
- ミリメートル単位での精密な位置指定

### 技術スタック

- **言語**: Rust (edition 2024)
- **主要ライブラリ**:
  - `printpdf`: PDF 生成のコアライブラリ
  - `ab_glyph`: フォントレンダリング
  - `image`: 画像処理
  - `serde_json`: JSON パース
  - `jsonschema`: スキーマ検証
  - `clap`: CLI インターフェース

## アーキテクチャ

### レイヤー構成

```
┌─────────────────────────────────────┐
│         CLI Layer (main.rs)         │
│  - コマンドライン引数の解析          │
│  - ファイル入出力                    │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     JSON Parse Layer                │
│  - document_json.rs                 │
│  - スキーマ検証                      │
│  - JSON → Document 構造体への変換    │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Document Model Layer            │
│  - document.rs                      │
│  - page.rs (static/dynamic)         │
│  - block.rs (要素の抽象化)           │
│  - geometry.rs (座標・サイズ)        │
│  - style.rs (スタイル定義)           │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Layout Engine Layer             │
│  - flexible_container.rs            │
│  - block_container.rs               │
│  - wrapper.rs (テキスト折り返し)     │
│  - dynamic_page.rs (自動改ページ)    │
└──────────────┬──────────────────────┘
               │
┌──────────────▼──────────────────────┐
│     Rendering Layer                 │
│  - pdf_writer.rs                    │
│  - text_renderer.rs                 │
│  - 各要素のレンダリング (text, image, │
│    rectangle, line)                 │
└─────────────────────────────────────┘
```

### データフロー

1. **入力**: JSON ファイル
2. **検証**: JSON スキーマによる検証 (`schema/schema.json`)
3. **パース**: JSON を Rust の構造体に変換
4. **レイアウト計算**: Flexible コンテナや動的ページのレイアウトを計算
5. **レンダリング**: `printpdf` を使用して PDF を生成
6. **出力**: PDF ファイル

## ディレクトリ構造

```
rs-pdf/
├── src/
│   ├── main.rs                      # エントリーポイント
│   ├── block_document.rs            # モジュールルート
│   └── block_document/
│       ├── block.rs                 # Block 型の定義（要素の抽象化）
│       ├── block_container.rs       # 通常のコンテナ
│       ├── direction.rs             # レイアウト方向（horizontal/vertical）
│       ├── document.rs              # Document 構造体
│       ├── document_json.rs         # JSON パーサー
│       ├── dynamic_page.rs          # 動的ページの実装
│       ├── flexible_container.rs    # Flexible コンテナ（Flexbox ライク）
│       ├── flexible_item.rs         # Flexible アイテム
│       ├── geometry.rs              # Point, Size, Frame の定義
│       ├── image.rs                 # 画像要素
│       ├── line.rs                  # 線要素
│       ├── page.rs                  # Page enum (Static/Dynamic)
│       ├── page_number.rs           # ページ番号機能
│       ├── pdf_writer.rs            # PDF 書き込み
│       ├── rectangle.rs             # 矩形要素
│       ├── static_page.rs           # 静的ページの実装
│       ├── style.rs                 # スタイル定義
│       ├── text.rs                  # テキスト要素
│       ├── text_renderer.rs         # テキストレンダリング
│       └── wrapper.rs               # テキスト折り返し処理
├── demo/
│   ├── static.json                  # 静的ページのサンプル
│   └── dynamic.json                 # 動的ページのサンプル
├── output/                          # 生成された PDF の出力先
├── assets/
│   ├── fonts/                       # フォントファイル
│   └── images/                      # 画像ファイル
├── schema/
│   └── schema.json                  # JSON スキーマ定義
├── Cargo.toml
└── README.md
```

## 主要な概念

### 1. ページタイプ

#### 静的ページ (Static Page)
- 要素を絶対座標または相対座標に配置
- ページの高さは固定
- 用途: 固定レイアウトの文書（カバーページ、固定フォーマットなど）

#### 動的ページ (Dynamic Page)
- 自動ページ分割機能を持つ
- `common`: 全ページ共通の要素（ヘッダー、フッターなど）
- `content`: 動的に配置されるコンテンツ（自動改ページ）
- `continuation`: 2 ページ目以降の共通要素と表示領域の定義
- 用途: コンテンツ量が可変の文書（請求書、納品書など）

### 2. Block 型

すべての要素は `Block` enum で抽象化されています:

```rust
pub enum Block {
    Text(Text),
    Image(Image),
    Rectangle(Rectangle),
    Line(Line),
    Objects(BlockContainer),
    Flexible(FlexibleContainer),
    FlexibleItem(FlexibleItem),
}
```

### 3. レイアウトシステム

#### 通常のコンテナ (`BlockContainer`)
- 子要素を `horizontal` または `vertical` 方向に配置
- 明示的なフレーム（座標とサイズ）を持つ

#### Flexible コンテナ (`FlexibleContainer`)
- Flexbox ライクなレイアウト
- 子要素（`FlexibleItem`）の `basis` に基づいてサイズを自動計算
- 親の幅/高さを子要素間で分配

### 4. 座標系

- **単位**: ミリメートル (mm)
- **原点**: 左上 (0, 0)
- **X 軸**: 右方向が正
- **Y 軸**: 下方向が正
- **A4 サイズ**: width: 210.0mm, height: 297.0mm

### 5. スタイルシステム

`Style` 構造体で以下を定義:
- 背景色、ボーダー色、テキスト色
- ボーダー幅、ボーダースタイル (solid/dash)
- テキストスタイル (fill/stroke/fill_stroke)
- テキスト配置 (horizontal/vertical alignment)
- テキスト折り返し設定
- 余白 (space: top/left/right/bottom)

## コーディング規約とベストプラクティス

### 1. コミットメッセージ

**Conventional Commits 形式を使用**:
- プレフィックス: `feat`, `fix`, `docs`, `refactor`, `chore`, `test`, `style`, `perf`
- プレフィックスは英語、本文は日本語
- 1 行で完結（72 文字以内推奨）
- 行末にブランチ名を記載

例:
```
feat: バージョン 0.2.3 にアップデートし `continuation` 機能を追加 master
```

### 2. コードスタイル

- **Rust 標準**: `rustfmt` に従う
- **命名規則**:
  - 構造体・Enum: `PascalCase`
  - 関数・変数: `snake_case`
  - 定数: `SCREAMING_SNAKE_CASE`
- **エラーハンドリング**: `Result` 型を使用し、適切なエラーメッセージを提供
- **ドキュメント**: 公開 API には doc コメント (`///`) を記述

### 3. モジュール構成

- 各要素型（Text, Image など）は独立したファイルに分離
- `block_document.rs` がモジュールのルート
- 共通の型（`Point`, `Size`, `Frame`）は `geometry.rs` に集約
- スタイル関連は `style.rs` に集約

### 4. JSON スキーマとの整合性

- JSON スキーマ (`schema/schema.json`) と Rust の構造体は常に同期
- 新機能追加時は両方を更新
- デモファイル (`demo/*.json`) で動作確認

### 5. パフォーマンス最適化

- リリースビルドでは最大限の最適化を有効化 (`Cargo.toml` の `[profile.release]` 参照)
- LTO (Link Time Optimization) を使用
- バイナリサイズの削減のため `strip = "symbols"` を設定

## 開発時の注意点

### 1. 動的ページの自動改ページ

- `dynamic_page.rs` の `split_content_into_pages` 関数が改ページロジックを実装
- コンテンツがフレームに収まらない場合、自動的に次ページに分割
- `continuation` 設定により、2 ページ目以降の表示領域と共通要素を変更可能

### 2. Flexible レイアウト

- `flexible_container.rs` が Flexbox ライクな計算を実装
- 子要素の `basis` の合計で親の幅/高さを分配
- `direction` (horizontal/vertical) に基づいて主軸を決定

### 3. テキスト折り返し

- `wrapper.rs` が文字単位の折り返しを実装
- `text_wrap.mode` が `character` の場合、文字単位で折り返し
- `break_anywhere: true` で任意の位置での改行を許可

### 4. フォント処理

- 外部フォントファイル (`document.font_path`) を読み込み
- `ab_glyph` でフォントメトリクスを計算
- CJK フォント（日本語など）の正しいレンダリングをサポート

### 5. デバッグモード

- `-d` または `--debug` フラグでグリッド表示を有効化
- レイアウトの確認やデバッグに使用

## よくある変更パターン

### 新しい要素型を追加する場合

1. `src/block_document/new_element.rs` を作成
2. `Block` enum に新しいバリアントを追加
3. JSON スキーマ (`schema/schema.json`) を更新
4. `document_json.rs` でパース処理を追加
5. `pdf_writer.rs` でレンダリング処理を追加
6. デモファイルで動作確認

### 新しいスタイルプロパティを追加する場合

1. `style.rs` の `Style` 構造体を更新
2. JSON スキーマを更新
3. `document_json.rs` でパース処理を追加
4. 各要素のレンダリング処理で新しいプロパティを適用

### バージョンアップ時

1. `Cargo.toml` の `version` を更新
2. `Cargo.lock` を更新（`cargo build` で自動更新）
3. `README.md` に変更内容を反映（必要に応じて）
4. コミットメッセージに変更内容を明記

## テストとデバッグ

### デモファイルの実行

```bash
# 静的ページのデモ
cargo run -- -i demo/static.json -o output/static.pdf -a

# 動的ページのデモ
cargo run -- -i demo/dynamic.json -o output/dynamic.pdf -a

# デバッグモード（グリッド表示）
cargo run -- -i demo/dynamic.json -o output/dynamic.pdf -a -d
```

### ビルド

```bash
# デバッグビルド
cargo build

# リリースビルド（最適化あり）
cargo build --release
```

## 参考リソース

- **README.md**: ユーザー向けドキュメント
- **schema/schema.json**: JSON スキーマ定義
- **demo/*.json**: サンプル設定ファイル
- **printpdf documentation**: https://docs.rs/printpdf/

## 現在のバージョン: 0.2.3

### 最新の変更（v0.2.3）

- `continuation` 機能を追加
- 2 ページ目以降の共通要素と表示領域を個別に制御可能に
- デモファイルを実用的な商品リスト形式に更新
