//! 텍스트 유틸: NFC 정규화, 한글 초성, 퍼센트 인코딩/디코딩, 경로 정규화.

use unicode_normalization::UnicodeNormalization;

/// 유니코드 NFC 로 정규화한다. (한글 경로/제목은 항상 NFC 로 고정)
pub fn nfc(s: &str) -> String {
    s.nfc().collect()
}

/// 한글 초성 테이블 (U+AC00 기준 19개).
pub const CHOSEONG: [char; 19] = [
    'ㄱ', 'ㄲ', 'ㄴ', 'ㄷ', 'ㄸ', 'ㄹ', 'ㅁ', 'ㅂ', 'ㅃ', 'ㅅ', 'ㅆ', 'ㅇ', 'ㅈ', 'ㅉ', 'ㅊ', 'ㅋ',
    'ㅌ', 'ㅍ', 'ㅎ',
];

/// 제목 문자열의 한글 초성 문자열.
///
/// 완성형 한글 음절(U+AC00..=U+D7A3)은 `(syllable - 0xAC00) / 588` 로 초성 인덱스를 구하고,
/// 한글이 아닌 문자는 그대로 통과시킨다. 입력은 먼저 NFC 로 정규화한다.
pub fn choseong(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in nfc(s).chars() {
        let cp = c as u32;
        if (0xAC00..=0xD7A3).contains(&cp) {
            let idx = ((cp - 0xAC00) / 588) as usize;
            out.push(CHOSEONG[idx]);
        } else {
            out.push(c);
        }
    }
    out
}

/// `%XX` 시퀀스를 디코딩한다. UTF-8 로 해석하고, 실패하면 손실 허용(lossy)으로 대체한다.
pub fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            let h = hexval(bytes[i + 1]);
            let l = hexval(bytes[i + 2]);
            if let (Some(h), Some(l)) = (h, l) {
                out.push(h * 16 + l);
                i += 3;
                continue;
            }
        }
        out.push(bytes[i]);
        i += 1;
    }
    String::from_utf8_lossy(&out).into_owned()
}

fn hexval(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}

/// 마크다운 링크 목적지로 안전하도록 최소한의 문자만 퍼센트 인코딩한다.
/// 비ASCII(한글 등)는 원문 그대로 둔다 — 브라우저가 요청 시 알아서 인코딩한다.
pub fn encode_path_component(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            ' ' => out.push_str("%20"),
            '%' => out.push_str("%25"),
            '#' => out.push_str("%23"),
            '?' => out.push_str("%3F"),
            '(' => out.push_str("%28"),
            ')' => out.push_str("%29"),
            '<' => out.push_str("%3C"),
            '>' => out.push_str("%3E"),
            '"' => out.push_str("%22"),
            '\'' => out.push_str("%27"),
            _ => out.push(c),
        }
    }
    out
}

/// 순수 어휘적으로 `.`/`..` 를 해소한 상대 경로. 루트를 넘어가는 `..` 는 무시한다.
pub fn normalize_rel_path(p: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for seg in p.split('/') {
        match seg {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            s => stack.push(s),
        }
    }
    stack.join("/")
}

/// 부모 디렉터리 (구분자 없으면 빈 문자열).
pub fn dirname(p: &str) -> &str {
    match p.rfind('/') {
        Some(i) => &p[..i],
        None => "",
    }
}

/// 마지막 구성요소.
pub fn basename(p: &str) -> &str {
    match p.rfind('/') {
        Some(i) => &p[i + 1..],
        None => p,
    }
}

/// 확장자(점 포함 소문자 아님, 원문 그대로). 없으면 빈 문자열.
pub fn extension(p: &str) -> &str {
    let b = basename(p);
    match b.rfind('.') {
        Some(i) if i > 0 => &b[i..],
        _ => "",
    }
}

/// 확장자를 바꾼 경로.
pub fn with_extension(p: &str, ext: &str) -> String {
    let b = basename(p);
    match b.rfind('.') {
        Some(i) if i > 0 => format!(
            "{}{}{}",
            dirname(p),
            if dirname(p).is_empty() { "" } else { "/" },
            format!("{}{}", &b[..i], ext)
        ),
        _ => format!("{}{}", p, ext),
    }
}

/// 경로를 `/` 구성요소로 나눈다.
pub fn components(p: &str) -> Vec<String> {
    p.split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
}

/// 사이드바 미리보기용 발췌 — front matter·코드펜스·제목·마크다운 장식을 걷어내고 공백을 하나로 접는다.
pub fn excerpt(content: &str, max_chars: usize) -> String {
    let body = strip_front_matter(content);
    let mut out = String::new();
    let mut in_fence = false;
    for line in body.lines() {
        let t = line.trim();
        if t.starts_with("```") || t.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence || t.is_empty() || t.starts_with('#') {
            continue;
        }
        if t.starts_with("<br") || t.starts_with("<!--") || t == "---" || t == "<br>" {
            continue;
        }
        let cleaned = clean_inline(t);
        if cleaned.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push(' ');
        }
        out.push_str(&cleaned);
        if out.chars().count() >= max_chars {
            break;
        }
    }
    truncate_chars(&out, max_chars)
}

fn strip_front_matter(s: &str) -> &str {
    let t = s.trim_start_matches('\u{feff}');
    if let Some(rest) = t.strip_prefix("---") {
        if let Some(idx) = rest.find("\n---") {
            return rest[idx + 4..].trim_start_matches(['\r', '\n']);
        }
    }
    t
}

fn find_close(chars: &[char], start: usize, open: char, close: char) -> Option<usize> {
    let mut depth = 0i32;
    for (i, &c) in chars.iter().enumerate().skip(start) {
        if c == open {
            depth += 1;
        } else if c == close {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
    }
    None
}

/// `![alt](url)` 같은 이미지 링크를 건너뛴 다음 위치.
fn skip_image(chars: &[char], i: usize) -> Option<usize> {
    let close = find_close(chars, i, '[', ']')?;
    if close + 1 < chars.len() && chars[close + 1] == '(' {
        return Some(find_close(chars, close + 1, '(', ')')? + 1);
    }
    None
}

/// `[text](url)` / `[text][ref]` 에서 보이는 글자만.
fn link_text(chars: &[char], i: usize) -> Option<(String, usize)> {
    let close = find_close(chars, i, '[', ']')?;
    let text: String = chars[i + 1..close].iter().collect();
    if close + 1 < chars.len() && chars[close + 1] == '(' {
        return Some((text, find_close(chars, close + 1, '(', ')')? + 1));
    }
    if close + 1 < chars.len() && chars[close + 1] == '[' {
        return Some((text, find_close(chars, close + 1, '[', ']')? + 1));
    }
    None
}

fn clean_inline(line: &str) -> String {
    let chars: Vec<char> = line.chars().collect();
    let mut out = String::new();
    let mut i = 0usize;
    while i < chars.len() {
        let c = chars[i];
        if c == '!' && i + 1 < chars.len() && chars[i + 1] == '[' {
            if let Some(next) = skip_image(&chars, i + 1) {
                i = next;
                continue;
            }
        }
        if c == '[' {
            if let Some((text, next)) = link_text(&chars, i) {
                out.push_str(&text);
                i = next;
                continue;
            }
        }
        if c == '<' {
            if let Some(close) = chars[i..].iter().position(|&x| x == '>') {
                i += close + 1;
                continue;
            }
        }
        if matches!(c, '*' | '_' | '`' | '~' | '>' | '|') {
            i += 1;
            continue;
        }
        out.push(c);
        i += 1;
    }
    // 공백 하나로 접기
    let mut s = String::new();
    let mut prev_space = false;
    for ch in out.chars() {
        let is_ws = ch == ' ' || ch == '\t';
        if is_ws {
            if !prev_space && !s.is_empty() {
                s.push(' ');
            }
            prev_space = true;
        } else {
            s.push(ch);
            prev_space = false;
        }
    }
    s.trim().trim_start_matches(['-', '•', '·']).trim().to_string()
}

fn truncate_chars(s: &str, max: usize) -> String {
    let count = s.chars().count();
    if count <= max {
        return s.to_string();
    }
    let cut: String = s.chars().take(max).collect();
    format!("{}…", cut.trim_end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn choseong_basic() {
        assert_eq!(choseong("전문검색인덱스"), "ㅈㅁㄱㅅㅇㄷㅅ");
        assert_eq!(choseong("Redis 분산락"), "Redis ㅂㅅㄹ");
    }

    #[test]
    fn normalize_paths() {
        assert_eq!(normalize_rel_path("a/./b/../c.md"), "a/c.md");
        assert_eq!(normalize_rel_path("../x/foo.md"), "x/foo.md");
        assert_eq!(dirname("a/b/c.md"), "a/b");
        assert_eq!(with_extension("a/b/c.m", ".md"), "a/b/c.md");
    }

    #[test]
    fn percent() {
        assert_eq!(percent_decode("a%20b%2Fc.md"), "a b/c.md");
        assert_eq!(encode_path_component("성능 비교.png"), "성능%20비교.png");
    }

    #[test]
    fn excerpt_strips_markdown() {
        let src = "---\ntitle: x\n---\n\n# 제목 줄\n\n본문 **강조** [링크](http://x/y) ![그림](image/a.png) 끝.\n\n```\n코드 블록\n```\n\n- 목록 항목\n";
        let e = excerpt(src, 60);
        assert!(e.starts_with("본문 강조 링크 끝."), "got: {e}");
        assert!(!e.contains("코드"), "코드펜스가 남았다: {e}");
        assert!(!e.contains("image/"), "이미지 경로가 남았다: {e}");
        assert!(!e.contains('#'), "제목이 남았다: {e}");
    }

    #[test]
    fn excerpt_truncates() {
        let e = excerpt("가나다라마바사", 4);
        assert_eq!(e, "가나다라…");
        assert_eq!(excerpt("짧다", 10), "짧다");
    }
}
