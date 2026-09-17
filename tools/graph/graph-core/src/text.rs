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
}
