//! 최소 JSON 직렬화기.
//!
//! 외부 크레이트 없이 결정론적으로 같은 바이트를 만든다.
//! UTF-8 은 그대로 두고(JSON 은 UTF-8 허용), 제어문자/따옴표/역슬래시만 이스케이프한다.

/// JSON 문자열 리터럴로 이스케이프한다.
pub fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0c}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// 문자열을 JSON 리터럴(따옴표 포함)로 만든다.
pub fn string(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    out.push('"');
    out.push_str(&escape(s));
    out.push('"');
    out
}

/// JSON 배열/객체를 조립하기 위한 간단한 빌더.
pub struct Arr {
    items: Vec<String>,
}

impl Arr {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
    pub fn push(&mut self, raw_json: String) {
        self.items.push(raw_json);
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn finish(self) -> String {
        let mut s = String::from("[");
        s.push_str(&self.items.join(","));
        s.push(']');
        s
    }
    /// 여러 줄로 펼친 배열(사람이 읽는 meta.json 용).
    pub fn finish_pretty(self, indent: &str) -> String {
        if self.items.is_empty() {
            return "[]".to_string();
        }
        let mut s = String::from("[\n");
        for (i, it) in self.items.iter().enumerate() {
            s.push_str(indent);
            s.push_str(it);
            if i + 1 != self.items.len() {
                s.push(',');
            }
            s.push('\n');
        }
        s.push_str(&" ".repeat(indent.len().saturating_sub(2)));
        s.push(']');
        s
    }
}

impl Default for Arr {
    fn default() -> Self {
        Self::new()
    }
}
