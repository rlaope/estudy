//! 스테이지 트리 준비/쓰기.

use std::path::{Path, PathBuf};

/// `.`/`..` 를 어휘적으로 해소한 절대경로 문자열 (심볼릭 링크 해석 없음).
pub fn lexical_absolute(p: &Path) -> PathBuf {
    let joined = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(p)
    };
    let mut out = PathBuf::new();
    for c in joined.components() {
        use std::path::Component::*;
        match c {
            RootDir | Prefix(_) => out.push(c.as_os_str()),
            CurDir => {}
            ParentDir => {
                out.pop();
            }
            Normal(s) => out.push(s),
        }
    }
    out
}

/// 스테이지 디렉터리를 비우고 다시 만든다. 위험한 대상은 거부한다.
pub fn prepare_stage(stage_abs: &Path, root_abs: &Path) -> Result<(), String> {
    if stage_abs.as_os_str().is_empty() || stage_abs == Path::new("/") {
        return Err("스테이지 경로가 비었거나 루트(/) 이다".to_string());
    }
    if stage_abs == root_abs {
        return Err(format!(
            "스테이지 경로가 리포 루트와 같다 — 원본을 지울 수 있어 거부한다: {}",
            stage_abs.display()
        ));
    }
    if root_abs.starts_with(stage_abs) {
        return Err(format!(
            "스테이지가 리포 루트의 조상이다 — 거부한다: stage={} root={}",
            stage_abs.display(),
            root_abs.display()
        ));
    }
    if stage_abs.exists() {
        std::fs::remove_dir_all(stage_abs)
            .map_err(|e| format!("스테이지 초기화 실패 {}: {}", stage_abs.display(), e))?;
    }
    std::fs::create_dir_all(stage_abs)
        .map_err(|e| format!("스테이지 생성 실패 {}: {}", stage_abs.display(), e))?;
    Ok(())
}

/// 스테이지 파일 경로를 만든다 (부모 디렉터리 생성 포함).
pub fn stage_path(stage_abs: &Path, rel: &str) -> Result<PathBuf, String> {
    let mut p = stage_abs.to_path_buf();
    for seg in rel.split('/') {
        p.push(seg);
    }
    if let Some(parent) = p.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("디렉터리 생성 실패 {}: {}", parent.display(), e))?;
    }
    Ok(p)
}

/// 파일을 바이트 그대로 복사한다.
pub fn copy_file(src: &Path, dst: &Path) -> Result<(), String> {
    std::fs::copy(src, dst)
        .map(|_| ())
        .map_err(|e| format!("복사 실패 {} → {}: {}", src.display(), dst.display(), e))
}

/// 텍스트 파일을 쓴다 (디렉터리 생성 포함).
pub fn write_bytes(dst: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(parent) = dst.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("디렉터리 생성 실패 {}: {}", parent.display(), e))?;
    }
    std::fs::write(dst, bytes).map_err(|e| format!("쓰기 실패 {}: {}", dst.display(), e))
}
