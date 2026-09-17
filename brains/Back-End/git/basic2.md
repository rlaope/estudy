# git 기본 명령어 정리 2

- `pwd` : 현재폴더  
- `cd` : directory 이동  
- `cd..` : 상위폴더 이동   

`자동완성 : tab키로 입력 가능`

```bash
ls -al
git init
git remote add <원격저장소이름>
```

- `ls -al` 숨김 폴더, 숨김 파일 보기

- `git init` 로컬 저장소 생성
- `git remote add <주소>` 내 컴퓨터에 master폴더에 github 저장소 주소를 알려준다


* master : branch 이름
* origin : 원격 저장소 이름

<br>

### git log - 이력 확인

```bash
git log [option] [revision range] [[--] <path>..]
```

- `git log`는 다양한 옵션을 조합하여 원하는 형태의 로그를 출력할 수 있는 기능입니다.

<br>

### git reset - 이전 상태로 (이력 제거)

```bash
git reset [<commit>] [--soft | --mixed [-N] | --hard | --merge | --keep]
```

- 특정 커밋까지 이력을 초기화합니다. 바로 전, 또는 n번 전까지 작업했던 내용을 취소할 수 있습니다.

- ` 이력이 지워지기 때문에 조심히 사용해야 합니다 `

- `git reset`은 다양한 옵션이 존재하는데 여기선 `--hard` 옵션을 사용합니다

#### 작업
1. `git log`로 2번 커밋 ID 조회
2. 2번 커밋까지 이력 초기화

```bash
git log
git reset {v2 커밋 아이디} -- hard # 커밋 아이디 예) 27a00b7
```

- result :
```
HEAD is now at 27a00b7 v2 commit
```
  
- 2번 커밋까지 이력 초기화 -> 결론적으로 3번 이력 삭제 확인

<br>

현재 Git 저장소 이력  

![저장소이력](image/basic2_history.png)
