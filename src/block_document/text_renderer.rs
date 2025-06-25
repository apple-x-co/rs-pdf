use crate::block_document::geometry::GeoSize;
use crate::block_document::style::{TextOverflow, TextWrap};
use ab_glyph::{Font, FontVec, ScaleFont};
use std::fs::File;
use std::io::Read;
use std::process::exit;

// 折り返し結果を表現するデータ構造
#[derive(Debug, Clone)]
pub struct WrappedLine {
    pub text: String,
    pub size: GeoSize,
}

#[derive(Debug, Clone)]
pub struct WrappedText {
    pub lines: Vec<WrappedLine>,
    pub total_size: GeoSize,
    #[allow(dead_code)] // 将来の機能拡張用
    pub is_truncated: bool, // オーバーフロー時に切り取られたかどうか
}

pub fn measure_text(text: &String, font_size: f32, font_path: &String) -> GeoSize {
    let file = File::open(font_path).map_err(|_| {
        eprintln!("Failed to open font file: {}.", font_path);
    });
    let font = match file {
        Ok(mut file) => {
            let mut font_data = Vec::new();
            file.read_to_end(&mut font_data).expect("Cannot read font");
            FontVec::try_from_vec(font_data).unwrap()
        }
        Err(_) => {
            eprintln!("Failed to open font file: {}.", font_path);
            exit(1);
        }
    };
    let scaled_font = font.as_scaled(font.pt_to_px_scale(font_size).unwrap());

    let mut max_width_px: f32 = 0.0;
    let mut current_width_px: f32 = 0.0;
    let mut prev_glyph = None;

    for c in text.chars() {
        if c == '\n' {
            max_width_px = max_width_px.max(current_width_px);
            current_width_px = 0.0;
            prev_glyph = None; // NOTE: 改行されたのでカーニングをリセット
            continue;
        }

        let glyph_id = scaled_font.glyph_id(c);
        if let Some(prev) = prev_glyph {
            current_width_px += scaled_font.kern(prev, glyph_id); // NOTE: カーニングを加算
        }

        current_width_px += scaled_font.h_advance(glyph_id); // NOTE: 文字の横幅
        prev_glyph = Some(glyph_id);
    }

    max_width_px = max_width_px.max(current_width_px);

    let num_lines = text.lines().count() as f32;
    let height_px: f32 = ((scaled_font.ascent() + scaled_font.descent()) * num_lines)
        + (scaled_font.line_gap() * (num_lines - 1.0));

    // NOTE: 1 px = 0.75 Pt
    let width_pt = max_width_px * 0.75;
    let height_pt = height_px * 0.75;

    // NOTE: 1 Pt = 0.35278 Mm
    let width_mm = width_pt * 0.35278;
    let height_mm = height_pt * 0.35278;

    GeoSize {
        width: width_mm,
        height: height_mm,
    }
}

// NOTE: 文字単位でテキストを折り返す
pub fn wrap_text_by_character(
    text: &String,
    font_size: f32,
    font_path: &String,
    available_width: f32,
    available_height: Option<f32>,
    text_wrap: &TextWrap,
) -> WrappedText {
    // NOTE: フォントの読み込み
    let file = File::open(font_path).map_err(|_| {
        eprintln!("Failed to open font file: {}.", font_path);
    });
    let font = match file {
        Ok(mut file) => {
            let mut font_data = Vec::new();
            file.read_to_end(&mut font_data).expect("Cannot read font");
            FontVec::try_from_vec(font_data).unwrap()
        }
        Err(_) => {
            eprintln!("Failed to open font file: {}.", font_path);
            exit(1);
        }
    };
    let scaled_font = font.as_scaled(font.pt_to_px_scale(font_size).unwrap());

    // NOTE: 行の高さを計算
    let line_height_px = scaled_font.ascent() + scaled_font.descent() + scaled_font.line_gap();
    let line_height_pt = line_height_px * 0.75;
    let line_height_mm = line_height_pt * 0.35278;

    let mut lines = Vec::new();
    let mut current_line = String::new();
    let mut current_width_px = 0.0;
    let mut prev_glyph = None;
    let mut is_truncated = false;

    // NOTE: Step 1: まず通常の折り返し処理（高さ制限は無視）
    for line_text in text.lines() {
        for c in line_text.chars() {
            let glyph_id = scaled_font.glyph_id(c);
            let mut char_width_px = scaled_font.h_advance(glyph_id);

            // NOTE: カーニングを考慮
            if let Some(prev) = prev_glyph {
                char_width_px += scaled_font.kern(prev, glyph_id);
            }

            // NOTE: 文字を追加した場合の幅をチェック
            let new_width_px = current_width_px + char_width_px;
            let new_width_pt = new_width_px * 0.75;
            let new_width_mm = new_width_pt * 0.35278;

            // NOTE: 幅制限をチェック
            if new_width_mm > available_width && !current_line.is_empty() {
                // NOTE: 現在の行を確定
                let line_size = measure_text(&current_line, font_size, font_path);
                lines.push(WrappedLine {
                    text: current_line.clone(),
                    size: line_size,
                });

                // NOTE: 新しい行を開始
                current_line = c.to_string();
                current_width_px = char_width_px;
                prev_glyph = Some(glyph_id);
            } else {
                // NOTE: 文字を現在の行に追加
                current_line.push(c);
                current_width_px = new_width_px;
                prev_glyph = Some(glyph_id);
            }
        }

        // NOTE: 明示的な改行があった場合、現在の行を確定
        if !current_line.is_empty() {
            let line_size = measure_text(&current_line, font_size, font_path);
            lines.push(WrappedLine {
                text: current_line.clone(),
                size: line_size,
            });

            current_line.clear();
            current_width_px = 0.0;
            prev_glyph = None;
        }
    }

    // NOTE: 最後の行が残っている場合
    if !current_line.is_empty() {
        let line_size = measure_text(&current_line, font_size, font_path);
        lines.push(WrappedLine {
            text: current_line,
            size: line_size,
        });
    }

    // NOTE: Step 2: 高さ制限のチェックと省略記号処理（必要な場合のみ）
    if let Some(max_height) = available_height {
        let max_lines = (max_height / line_height_mm).floor() as usize;

        if lines.len() > max_lines && max_lines > 0 {
            // NOTE: 高さオーバーフロー発生
            is_truncated = true;

            // NOTE: 表示可能な行数まで切り詰め
            lines.truncate(max_lines);

            // NOTE: 最後の行に省略記号処理（ellipsisの場合のみ）
            if matches!(text_wrap.overflow, TextOverflow::Ellipsis) && !lines.is_empty() {
                if let Some(last_line) = lines.last_mut() {
                    last_line.text = truncate_with_ellipsis(
                        &last_line.text,
                        font_size,
                        font_path,
                        available_width,
                    );
                    last_line.size = measure_text(&last_line.text, font_size, font_path);
                }
            }
        }
    }

    // NOTE: 全体のサイズを計算
    let max_width = lines.iter().map(|l| l.size.width).fold(0.0, f32::max);
    let total_height = lines.len() as f32 * line_height_mm;

    WrappedText {
        lines,
        total_size: GeoSize {
            width: max_width,
            height: total_height,
        },
        is_truncated,
    }
}

// NOTE: 省略記号付きで文字列を切り詰める
fn truncate_with_ellipsis(
    text: &String,
    font_size: f32,
    font_path: &String,
    available_width: f32,
) -> String {
    let ellipsis = "...";
    let ellipsis_size = measure_text(&ellipsis.to_string(), font_size, font_path);

    // NOTE: 省略記号だけで枠を超える場合は省略記号のみ返す
    if ellipsis_size.width >= available_width {
        return ellipsis.to_string();
    }

    let available_for_text = available_width - ellipsis_size.width;

    // NOTE: テキスト全体が利用可能幅に収まる場合は、省略記号を付けて返す
    let full_size = measure_text(text, font_size, font_path);
    if full_size.width <= available_for_text {
        return format!("{}{}", text, ellipsis);
    }

    // NOTE: 文字を1つずつ削除して収まるサイズを見つける
    let chars: Vec<char> = text.chars().collect();

    // NOTE: 最低1文字は残すようにする
    for i in (1..chars.len()).rev() {
        let truncated: String = chars[..i].iter().collect();
        let size = measure_text(&truncated, font_size, font_path);
        if size.width <= available_for_text {
            return format!("{}{}", truncated, ellipsis);
        }
    }

    // NOTE: 1文字だけを試す
    if !chars.is_empty() {
        let single_char: String = chars[..1].iter().collect();
        let size = measure_text(&single_char, font_size, font_path);
        if size.width <= available_for_text {
            return format!("{}{}", single_char, ellipsis);
        }
    }

    // NOTE: 1文字も入らない場合は省略記号のみ
    ellipsis.to_string()
}