# Npm(Node.js) - 기초 명령어 사용법 정리

## npm(node.js) - 명령어 기본 설명
1. npm - node.js를 설치하면 자동으로 함께 설치 된다.
2. 실제 프로젝트에서 npm 기반 모듈 설치 & 제거 및 버전 관리 사용법은 정말 중요하다
3. 프로젝트 초기에 각 모듈별 버전 관리를 철저학 ㅔ해야 나중에 꼬이지 않는다.

<br>

### npm - 도움말 옵션(-h)
```bash
# npm 커맨드 확인
npm -h

# npm 커맨드 세부사항 확인
npm 커맨드 -h
```

<br>

### npm - list
```bash
# 현재 프로젝트 설치된 모듈 확인
npm list

# npm 전역에 설치 모듈 확인
npm list -g

# depth 옵션
npm list -g --depth=0
```

<br>

### npm - view

```bash
# 모듈의 최신버전 확인
npm view 모듈명 version
```


