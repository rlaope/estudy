# 브랜치 Brnach


### 브랜치란 ?
- 브랜치란 독립적으로 어떤 작업을 진행하기 위한 개념입니다.
- 필요에 의해 만들어지는 각각의 브랜치는 다른 브랜치의 영향을 받지 않기 때문에, 여러 작업을 동시에 진행할 수 있습니다.

### master 브랜치

- 저장소를 처음 만들면, Git은 바로 `master`라는 이름의 브랜치를 만들어 둡니다.
- 'master'가 아닌 또 다른 새로운 브랜치를 만들어서 '이제부터 이 브랜치를 사용할거야!'라고 선언(체크아웃, checkout)하지 않는 이상, 이 때의 모든 작업은 `master` 브랜치에서 이루어 집니다.

## 브랜치 만들기 , 전환하기
1. huemang이라는 브랜치를 만들어 보겠습니다 ` git branch <username> ` 이렇게 작성하시면 됩니다  

```bash
git branch huemang
```

git branch라고 입력하시면 huemang이라는 브랜치가 생성된 것을 볼 수 있습니다.

2. 이제 master 브랜치에서 huemang이라는 브랜치로 이동해보겠습니다 그때 사용하는 명령어가 `checkout`입니다.
` git checkout <username> `으로 입력합니다.

```bash
git checkout huemang
```
이렇게 입력해주시면 브랜치는 huemang으로 바뀌었습니다

<br>
  
## 브랜치 병합하기

- huemang이라는 branch와 master라는 branch를 한번 병합해보겠습니다 그때 사용하는 명령어는 `merge` 입니다.
`git merge <commit> `

- 'master' 브랜치에 'huemang'를 넣기 위해서는 우선 'master' 브랜치에 'HEAD'가 위치하게 만들어야 합니다. 이 때에는 checkout 명령어를 이용하여 현재 사용중인 브랜치를 'master'로 전환합니다.

```bash
git checkout master
```

- 그리고 나서 merge를 사용해서 병합해주시면 됩니다
```bash
git merge huemang
```
master라는 브랜치에 huemang이라는 브랜치를 병합

<br>

## 브랜치 삭제하기

- huemang 브랜치의 내용이 모두 'master'에 통합 되었기 때문에 이제 더 이상 huemang 브랜치가 필요없게 되었습니다.

- 브랜치를 삭제하려면 branch 명령에 -d 옵션을 지정하여 실행하면 됩니다.

```bash
$ git branch -d huemang
```

이렇게 하면 huemang이라는 브랜치는 삭제가 되었습니다.




