# 다른사람 Repo 관리

1. 먼저 collaborator로 초대를 받는다.
2. 이메일에서 초대를 승인한다.

<br>

## 다른사람의 Repo를 내 Repo로 가져오기

1. 내 깃허브에서 가져올 리포의 이름과 동일한 새 리포를 생성. 이때, README, gitignore, license는 모두 초기화 하지 않는다.  

2. 가져올 repo를 내 컴퓨터에 다음과 같이 clone 한다.  

```bash
git clone <url>
```

3. clone 한 local repo를 1번에서 만든 새 repo와 연결
```bash
git remote add origin <url>
```

4. 다음과 같이 알잘딱

```bash
 git branch -M main
 git push -u origin master
 ```