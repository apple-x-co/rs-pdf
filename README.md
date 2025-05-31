# rs-pdf

JSONからPDFを生成するCLIツール

## 概要

rs-pdfは、JSON設定ファイルからPDFドキュメントを生成するRustで書かれたコマンドラインツールです。テキスト、画像、図形、線などの要素を組み合わせて、複雑なレイアウトのPDFを作成できます。

## 特徴

📄 **JSON設定による柔軟なレイアウト** - JSON形式の設定ファイルでPDFのレイアウトを定義  
🎨 **豊富な要素サポート** - テキスト、画像、矩形、線を組み合わせた文書作成  
📐 **レスポンシブレイアウト** - FlexibleコンテナによるFlexboxライクなレイアウト  
🎯 **精密な位置指定** - ミリメートル単位での正確な要素配置  
🌈 **スタイルカスタマイズ** - 色、ボーダー、フォントなど詳細なスタイル設定  
🔤 **日本語フォント対応** - CJKフォントを含む外部フォントの読み込み  
📱 **マルチページ対応** - 複数ページの文書生成

## 使用方法

```bash
rs-pdf -i input.json -o output.pdf
```

### オプション

- `-i, --input-path <PATH>` : 入力JSONファイルのパス
- `-o, --output-path <PATH>` : 出力PDFファイルのパス
- `-d, --debug` : デバッグモード（グリッド表示）
- `-a, --allow-override` : 既存ファイルの上書きを許可

## JSON設定ファイルの構造

### 基本構造

```json5
{
  "$schema": "https://github.com/apple-x-co/rs-pdf/blob/master/schema/schema.json",
  "document": {
    "title": "文書タイトル",
    "width": 210.0,
    "height": 297.0,
    "font_path": "fonts/NotoSansCJKjp-Thin.ttf",
    "pages": [
      {
        "objects": [
          // ページ内の要素
        ]
      }
    ]
  }
}
```

### サポートされる要素

#### テキスト要素

```json
{
  "type": "text",
  "text": "表示するテキスト",
  "font_size": 24.0,
  "bounds": {
    "point": { "x": 10.0, "y": 10.0 },
    "size": { "width": 100.0, "height": 30.0 }
  },
  "style": {
    "text_fill_color": { "red": 0, "green": 0, "blue": 255 },
    "border_color": { "red": 200, "green": 200, "blue": 200 }
  }
}
```

#### 画像要素

```json
{
  "type": "image",
  "path": "assets/images/sample.png",
  "bounds": {
    "point": { "x": 50.0, "y": 50.0 },
    "size": null
  }
}
```

#### 矩形要素

```json
{
  "type": "rectangle",
  "bounds": {
    "point": { "x": 0.0, "y": 0.0 },
    "size": { "width": 100.0, "height": 50.0 }
  },
  "style": {
    "background_color": { "red": 200, "green": 255, "blue": 200 },
    "border_width": { "width": 1.0 }
  }
}
```

#### 線要素

```json
{
  "type": "line",
  "bounds": {
    "point": { "x": 0.0, "y": 0.0 },
    "size": { "width": 100.0, "height": 0.0 }
  },
  "style": {
    "border_style": { "line_style": "dash", "dash_1": 2 }
  }
}
```

### コンテナ要素

#### 通常のコンテナ

```json
{
  "type": "objects",
  "direction": "horizontal",
  "bounds": {
    "point": { "x": 10.0, "y": 10.0 },
    "size": { "width": 200.0, "height": 100.0 }
  },
  "objects": [
    // 子要素
  ]
}
```

#### Flexibleコンテナ

```json5
{
  "type": "flexible",
  "direction": "horizontal",
  "objects": [
    // 子要素（自動的にサイズが調整される）
  ]
}
```

### スタイル設定

#### 色設定
- `background_color` : 背景色
- `border_color` : ボーダー色
- `text_fill_color` : テキスト塗りつぶし色
- `text_outline_color` : テキストアウトライン色

#### ボーダー設定
- `border_width` : ボーダー幅
- `border_style` : ボーダースタイル（solid/dash）

#### テキストスタイル
- `text_style` : テキスト描画モード（fill/stroke/fill_stroke）
- `text_outline_style` : アウトラインスタイル

### 座標系とサイズ

- **単位**: ミリメートル (mm)
- **座標系**: 左上原点
- **ページサイズ**: A4サイズの場合 width: 210.0, height: 297.0

## 設定例

完全な設定例は `schema/demo.json` を参照してください。

## 推奨フォント

日本語表示には以下のフォントを推奨します：

https://github.com/minoryorg/Noto-Sans-CJK-JP

## サポートされていない機能

- 圧縮
- パスワード保護

## スキーマ検証

JSON設定ファイルは `schema/schema.json` のスキーマに基づいて検証されます。不正な設定の場合はエラーメッセージが表示されます。