//! 관련 노트 엣지 — 본문 토큰으로 계산한 코사인 유사도(명시 링크와 별개).
//!
//! 목적: 색인(README)·문서 내부 링크로만 이어져 있던 그래프에 "내용이 관련 있는 노트끼리"의
//! 이어진 선을 더한다. 결정론이 계약이므로 입출력 순서를 전부 고정한다(토큰 정렬 · 후보 정렬 ·
//! 동점 시 id 오름차순 · 결과 (min,max) 정렬).
use std::collections::{BTreeSet, HashMap};

/// 한 노트에 유지하는 관련 엣지 수(상위 k).
pub const TOP_K: usize = 8;
/// 채택 최소 코사인 유사도.
pub const MIN_SCORE: f32 = 0.04;
/// 문서 빈도 상한 비율 — 이보다 흔한 토큰은 주제 신호가 아니므로 버린다.
const MAX_DF_RATIO: f32 = 0.10;
const MIN_TOKEN_CHARS: usize = 2;
const MAX_TOKEN_CHARS: usize = 24;

/// 한국어/영어 불용어 — 본문에서 자주 나오지만 주제를 구분하지 못하는 말.
const STOPWORDS: &[&str] = &[
    // 영어
    "the",
    "and",
    "for",
    "with",
    "this",
    "that",
    "from",
    "into",
    "are",
    "was",
    "were",
    "will",
    "not",
    "you",
    "your",
    "can",
    "use",
    "used",
    "using",
    "when",
    "then",
    "than",
    "there",
    "here",
    "they",
    "them",
    "his",
    "her",
    "its",
    "our",
    "out",
    "one",
    "two",
    "all",
    "any",
    "how",
    "what",
    "which",
    "who",
    "why",
    "has",
    "have",
    "had",
    "but",
    "also",
    "more",
    "most",
    "some",
    "such",
    "only",
    "like",
    "just",
    "each",
    "other",
    "same",
    "new",
    "get",
    "set",
    "add",
    "run",
    "code",
    // 한국어
    "그리고",
    "하지만",
    "그런데",
    "그래서",
    "이렇게",
    "그렇게",
    "저렇게",
    "때문",
    "경우",
    "대한",
    "대해",
    "통해",
    "위해",
    "위한",
    "있다",
    "없다",
    "있고",
    "없고",
    "하는",
    "되는",
    "하고",
    "되고",
    "하면",
    "되면",
    "해서",
    "에서",
    "으로",
    "로서",
    "로써",
    "부터",
    "까지",
    "보다",
    "처럼",
    "같이",
    "등등",
    "또는",
    "혹은",
    "그것",
    "이것",
    "저것",
    "여기",
    "저기",
    "우리",
    "너희",
    "자신",
    "서로",
    "정도",
    "다음",
    "이후",
    "이전",
    "먼저",
    "나중",
    "다시",
    "아주",
    "매우",
    "정말",
    "너무",
    "조금",
    "이런",
    "그런",
    "저런",
    "어떤",
    "무슨",
    "각각",
    "모두",
    "전부",
    "일부",
    "여러",
    "많은",
    "적은",
];

/// 본문 + 경로에서 주제 토큰을 뽑는다(소문자 · 중복 제거 · 정렬).
pub fn tokens(text: &str, path: &str) -> Vec<String> {
    let mut set: BTreeSet<String> = BTreeSet::new();
    collect(text, &mut set);
    // 경로 조각(디렉터리·파일명)도 주제 토큰으로 — `java`, `spring`, `kafka` 같은 신호.
    for part in path.split('/') {
        let stem = part.strip_suffix(".md").unwrap_or(part);
        collect(stem, &mut set);
    }
    set.into_iter().collect()
}

fn collect(text: &str, set: &mut BTreeSet<String>) {
    let mut cur = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() {
            for low in ch.to_lowercase() {
                cur.push(low);
            }
        } else {
            flush(&mut cur, set);
        }
    }
    flush(&mut cur, set);
}

fn flush(cur: &mut String, set: &mut BTreeSet<String>) {
    if cur.is_empty() {
        return;
    }
    let n = cur.chars().count();
    let all_digit = cur.chars().all(|c| c.is_ascii_digit());
    if n >= MIN_TOKEN_CHARS
        && n <= MAX_TOKEN_CHARS
        && !all_digit
        && !STOPWORDS.contains(&cur.as_str())
    {
        set.insert(cur.clone());
    }
    cur.clear();
}

/// 관련 엣지 계산. 반환은 `(min, max)` 쌍을 정렬한 목록(무방향 1회 저장).
pub fn build(
    note_count: usize,
    note_tokens: &[Vec<String>],
    explicit: &[(u32, u32)],
    top_k: usize,
    min_score: f32,
) -> Vec<(u32, u32)> {
    if note_count == 0 {
        return Vec::new();
    }
    let max_df = ((note_count as f32) * MAX_DF_RATIO).ceil() as usize;

    // 문서 빈도 → idf 가중치
    let mut df: HashMap<&str, u32> = HashMap::new();
    for toks in note_tokens {
        for t in toks {
            *df.entry(t.as_str()).or_insert(0) += 1;
        }
    }
    let idf = |t: &str| -> f32 { (1.0 + note_count as f32 / df[t] as f32).ln() };

    // 노트 벡터 노름
    let norms: Vec<f32> = note_tokens
        .iter()
        .map(|toks| {
            let s: f32 = toks.iter().map(|t| idf(t) * idf(t)).sum();
            s.sqrt()
        })
        .collect();

    // 역색인(상한 이하 토큰만)
    let mut postings: HashMap<&str, Vec<u32>> = HashMap::new();
    for (i, toks) in note_tokens.iter().enumerate() {
        for t in toks {
            let d = df[t.as_str()] as usize;
            if d <= max_df && d > 1 {
                postings.entry(t.as_str()).or_default().push(i as u32);
            }
        }
    }

    let explicit_set: BTreeSet<(u32, u32)> = explicit
        .iter()
        .map(|&(a, b)| if a <= b { (a, b) } else { (b, a) })
        .collect();

    let mut result: BTreeSet<(u32, u32)> = BTreeSet::new();
    let mut acc: HashMap<u32, f32> = HashMap::new();
    for (i, toks) in note_tokens.iter().enumerate() {
        acc.clear();
        for t in toks {
            let d = df[t.as_str()] as usize;
            if d > max_df || d <= 1 {
                continue;
            }
            let w = idf(t);
            if let Some(list) = postings.get(t.as_str()) {
                for &j in list {
                    if j as usize != i {
                        *acc.entry(j).or_insert(0.0) += w * w;
                    }
                }
            }
        }
        let ni = norms[i];
        if ni <= 0.0 {
            continue;
        }
        let mut scored: Vec<(f32, u32)> = acc
            .iter()
            .filter(|(_, s)| **s > 0.0)
            .map(|(&j, &s)| (s / (ni * norms[j as usize]), j))
            .collect();
        // 점수 내림차순, 동점이면 id 오름차순(결정론)
        scored.sort_by(|a, b| {
            b.0.partial_cmp(&a.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.1.cmp(&b.1))
        });
        let mut kept = 0;
        for (score, j) in scored {
            if kept >= top_k {
                break;
            }
            if score < min_score {
                break;
            }
            let pair = if (i as u32) <= j {
                (i as u32, j)
            } else {
                (j, i as u32)
            };
            if explicit_set.contains(&pair) {
                continue; // 이미 명시 링크가 있는 쌍은 중복해서 잇지 않는다
            }
            result.insert(pair);
            kept += 1;
        }
    }
    result.into_iter().collect()
}

/// `related.bin` 본문: `u32` LE 쌍 평탄 배열 `[u0, v0, u1, v1, ...]`.
pub fn to_le_bytes(pairs: &[(u32, u32)]) -> Vec<u8> {
    let mut out = Vec::with_capacity(pairs.len() * 8);
    for &(u, v) in pairs {
        out.extend_from_slice(&u.to_le_bytes());
        out.extend_from_slice(&v.to_le_bytes());
    }
    out
}

/// 노트별 관련 노트 목록(사이드바 · 강조용).
pub fn adjacency(note_count: usize, pairs: &[(u32, u32)]) -> Vec<Vec<u32>> {
    let mut adj = vec![Vec::new(); note_count];
    for &(u, v) in pairs {
        adj[u as usize].push(v);
        adj[v as usize].push(u);
    }
    for a in adj.iter_mut() {
        a.sort_unstable();
        a.dedup();
    }
    adj
}
