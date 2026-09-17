//! 그래프 모델: 노트 + 섹션 허브 + 루트 허브, 그리고 CSR 인접 행렬.

use std::collections::BTreeSet;

use crate::links::{Kind, LinkRecord};

#[derive(Debug, Default, Clone)]
pub struct EdgeBreakdown {
    /// 노트 ↔ 소속 섹션 허브
    pub section_membership: usize,
    /// 루트 허브 ↔ 섹션 허브
    pub root_to_section: usize,
    /// 노트 → 노트 (본문 내 `.md` 링크)
    pub intra_note: usize,
    /// README → 노트 (색인 직접 링크)
    pub readme_direct: usize,
    /// README 중첩 목록의 부모 → 자식
    pub readme_parent_child: usize,
    /// README → 섹션 허브 (디렉터리 타깃)
    pub readme_to_section: usize,
    /// 중복 제거 후 무방향 엣지 총수
    pub total_unique: usize,
}

#[derive(Debug, Clone)]
pub struct Graph {
    pub node_count: usize,
    pub note_count: usize,
    /// (섹션 이름, 노트 수) — 이름 오름차순
    pub sections: Vec<(String, usize)>,
    /// (섹션 이름, 허브 노드 id)
    pub section_hubs: Vec<(String, usize)>,
    pub root_hub: usize,
    /// 무방향 엣지 (u < v), 중복 제거, 정렬
    pub edges: Vec<(u32, u32)>,
    /// CSR 오프셋 (N+1)
    pub offsets: Vec<u32>,
    /// CSR 타깃 (무방향이므로 각 엣지가 양방향으로 2번 들어간다 → 길이 = 2 * |edges|)
    pub targets: Vec<u32>,
    pub degrees: Vec<u32>,
    pub breakdown: EdgeBreakdown,
}

impl Graph {
    /// CSR 을 u32 little-endian 바이트로 직렬화한다: [offsets N+1][targets E]
    pub fn csr_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity((self.offsets.len() + self.targets.len()) * 4);
        for v in &self.offsets {
            out.extend_from_slice(&v.to_le_bytes());
        }
        for v in &self.targets {
            out.extend_from_slice(&v.to_le_bytes());
        }
        out
    }
}

/// 노트 경로의 톱레벨 섹션 이름 (루트 노트는 빈 문자열).
pub fn top_section(rel: &str) -> &str {
    match rel.find('/') {
        Some(i) => &rel[..i],
        None => "",
    }
}

pub fn build(notes: &[String], records: &[Vec<LinkRecord>]) -> Graph {
    let note_count = notes.len();

    // 섹션 목록 (톱레벨 디렉터리) — 이름 오름차순, 노트 수 포함
    let mut section_names: Vec<String> = Vec::new();
    for n in notes {
        let s = top_section(n);
        if !s.is_empty() && !section_names.iter().any(|x| x == s) {
            section_names.push(s.to_string());
        }
    }
    section_names.sort();
    let mut sections: Vec<(String, usize)> = section_names
        .iter()
        .map(|name| {
            let cnt = notes
                .iter()
                .filter(|n| top_section(n) == name.as_str())
                .count();
            (name.clone(), cnt)
        })
        .collect();
    sections.sort();

    let hub_id = |name: &str| -> Option<usize> {
        section_names
            .binary_search_by(|s| s.as_str().cmp(name))
            .ok()
            .map(|k| note_count + k)
    };
    let section_hubs: Vec<(String, usize)> = section_names
        .iter()
        .enumerate()
        .map(|(k, s)| (s.clone(), note_count + k))
        .collect();
    let root_hub = note_count + section_names.len();
    let node_count = root_hub + 1;

    let mut bd = EdgeBreakdown::default();
    let mut set: BTreeSet<(u32, u32)> = BTreeSet::new();
    let add =
        |a: usize, b: usize, which: u8, bd: &mut EdgeBreakdown, set: &mut BTreeSet<(u32, u32)>| {
            if a == b {
                return;
            }
            let key = if a < b {
                (a as u32, b as u32)
            } else {
                (b as u32, a as u32)
            };
            let fresh = set.insert(key);
            if !fresh {
                return;
            }
            match which {
                1 => bd.section_membership += 1,
                2 => bd.root_to_section += 1,
                3 => bd.intra_note += 1,
                4 => bd.readme_direct += 1,
                5 => bd.readme_parent_child += 1,
                6 => bd.readme_to_section += 1,
                _ => {}
            }
        };

    // (1) 섹션 소속: 노트 ↔ 자기 톱레벨 섹션 허브 (루트 노트는 루트 허브)
    for (i, n) in notes.iter().enumerate() {
        let s = top_section(n);
        let hub = if s.is_empty() {
            root_hub
        } else {
            hub_id(s).unwrap_or(root_hub)
        };
        add(i, hub, 1, &mut bd, &mut set);
    }
    // (2) 루트 허브 ↔ 섹션 허브
    for (_, hub) in &section_hubs {
        add(root_hub, *hub, 2, &mut bd, &mut set);
    }

    // (3) 본문 내 링크 (노트 → 노트) — README 직접 링크도 여기에 포함된다
    for (i, recs) in records.iter().enumerate() {
        for r in recs {
            if let Kind::Note(j) = r.kind {
                add(i, j, 3, &mut bd, &mut set);
            }
        }
    }

    // (4) README 색인 계층: 직접 링크 + 중첩 목록의 부모→자식
    // README 파일명을 하드코딩하지 않는다 — 루트의 `README*.md` 중 대표 하나를 고른다
    // (`README.md` 가 있으면 그것, 없으면 정렬상 첫 번째, 예: `README.ko.md`).
    let readme = {
        let root_readmes: Vec<usize> = notes
            .iter()
            .enumerate()
            .filter(|(_, n)| !n.contains('/') && {
                let lower = n.to_lowercase();
                lower.ends_with(".md") && lower.starts_with("readme")
            })
            .map(|(i, _)| i)
            .collect();
        root_readmes
            .iter()
            .copied()
            .find(|&i| notes[i].to_lowercase() == "readme.md")
            .or_else(|| root_readmes.first().copied())
    };
    if let Some(readme) = readme {
        for r in &records[readme] {
            match &r.kind {
                Kind::Note(j) => add(readme, *j, 4, &mut bd, &mut set),
                Kind::Section(name) => {
                    if let Some(h) = hub_id(name) {
                        add(readme, h, 6, &mut bd, &mut set);
                    }
                }
                _ => {}
            }
        }

        let mut stack: Vec<(usize, Option<usize>)> = Vec::new();
        for r in &records[readme] {
            if !r.bullet {
                continue;
            }
            let target: Option<usize> = match &r.kind {
                Kind::Note(j) => Some(*j),
                Kind::Section(name) => hub_id(name),
                _ => None,
            };
            while let Some((ind, _)) = stack.last() {
                if *ind >= r.indent {
                    stack.pop();
                } else {
                    break;
                }
            }
            let parent = stack.last().and_then(|(_, t)| *t);
            if let (Some(p), Some(t)) = (parent, target) {
                add(p, t, 5, &mut bd, &mut set);
            }
            let inherit = target.or(parent);
            stack.push((r.indent, inherit));
        }
    }

    let edges: Vec<(u32, u32)> = set.into_iter().collect();
    bd.total_unique = edges.len();

    // CSR (무방향 → 양방향 저장)
    let mut adj: Vec<Vec<u32>> = vec![Vec::new(); node_count];
    for (a, b) in &edges {
        adj[*a as usize].push(*b);
        adj[*b as usize].push(*a);
    }
    let mut offsets: Vec<u32> = Vec::with_capacity(node_count + 1);
    let mut targets: Vec<u32> = Vec::new();
    let mut degrees: Vec<u32> = Vec::with_capacity(node_count);
    offsets.push(0);
    for v in adj.iter_mut() {
        v.sort_unstable();
        v.dedup();
        degrees.push(v.len() as u32);
        targets.extend_from_slice(v);
        offsets.push(targets.len() as u32);
    }

    Graph {
        node_count,
        note_count,
        sections,
        section_hubs,
        root_hub,
        edges,
        offsets,
        targets,
        degrees,
        breakdown: bd,
    }
}
