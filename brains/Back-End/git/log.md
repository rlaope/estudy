# git log - 이력확인
- git log는 다양한 옵션을 조합하여 원하는 형태의 로그를 출력할 수 있는 강력한 기능입니다. 이번 실습에선, 추가 옵션 없이 `git log`만 사용합니다.

### 작업
1. 전체 로그 확인

```bash
git log
```
  
결과

```bash
commit 306b9474de0af37367ff90e5c1367588413f81bf (HEAD -> main)
Author: subicura <subicura@subicura.com>
Date:   Sat Jul 17 00:55:36 2021 +0900

    v3 commit

commit 27a00b73cf7ab2e70e8dd5e5235bf7f94e9ddd84
Author: subicura <subicura@subicura.com>
Date:   Sat Jul 17 00:53:50 2021 +0900

    v2 commit

commit 1ac5146ad27c5277996d54c08ec4ccded0edd4e3
Author: subicura <subicura@subicura.com>
Date:   Sat Jul 17 00:50:30 2021 +0900

    v1 commit 
```
- 전체 커밋 로그 확인.