# 원격 저장소

## pull, 원격 저장소의 데이터를 로컬 저장소에 가져와 병합하기

- `pull`을 실행하면 원격 저장소의 변경도니 데이터를 가져올 수 있습니다.
- `git pull`은 Remote Repository의 커밋을 가져온 다음 (git fetch) 현재 Working Directory에 Merge하는 동작을 하는 명령어이다.

> git pull = git fetch + get merge

### pull
- 원격 저장소로 부터 필요한 파일을 다운 + 병합
- 지역 브랜치와, 원격 저장소 origin / master 가 같은 위치를 가르킨다

### fetch

- 원격 저장소로부터 필요한 파일을 다운 (병합은 따로해야함)
- 지역 브랜치는 원래 가지고 있던 지역 저장소의 최근 커밋 위치를 가리키고, 원격 저장소 origin/master는 가져온 최신 커밋을 가리킨다.
- 신중할 때 사용한다

- 사용 이유
> 원래 내용과 바뀐 내용의 차이를 알 수 있다.
> commit이 얼마나 됐는지 알 수 있다.
> 이런 세부 내용 확인 후 git merge origin/ master 하면 git pull 상태와 같아진다.


## pull과 fetch의 차이

git pull
- git remote 명령을 통해 서로 연결된 원격 저장소의 최신 내용을 로컬 저장소로 *가져오면서 병합*한다.

git fetch
- 로컬 저장소와 원격 저장소의 변경 사항이 다를 때 이를 비교 대조하고 git merge 명령어와 함께 최신 데이터를 반영하거나 충돌 문제 등을 해결한다.

<br>


## Git Push 에러 해결하기

```
! [rejected] master -> master (fetch first)
error: failed to push some refs to 'https://github.com/dalso~~'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing
hint: to the same ref. You may want to first integrate the remote changes
hint: (e.g., 'git pull …') before pushing again.
hint: See the 'Note about fast-forwards' in 'git push --help' for details.
```

- 위의 에러 원인은 깃의 원격 저장소와 현재 로컬 저장소가 동기화 되어있지 않기 때문입니다. 해결방법도 동기화 시켜주면 간단하게 해결합니다.

```bash
git pull --rebase 원격저장소별칭 master
```

위 명령어를 통해 정상적으로 동기화 시켜주면 다시 push하는데 이상 없을것이다.