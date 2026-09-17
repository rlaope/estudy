# 패키지 설치 / 삭제 명령어

- 외부 모듈인 패키지를 설치/삭제하는 명령어이다.

## 1. 패키지 설치
- 명령어 : n`pm install 패키지명`
- 패키지를 현재 프로젝트의 node_modules 폴더에 설치한다
- package.json 파일의 dependencies 항목에도 자동으로 추가 된다.
  > 과거에는 dependencies에 패키지를 추가하기 위해 npm install --save 패키지명 으로 설치했으나, npm@5 버전 이후부터는 디폴트로 적용되므로 --save를 붙이지 않아도 된다.
- `npm i 패키지명` 으로 축약가능하다
- `npm install 패키지명@버전` 으로 버전을 지정 가능
- `npm install 주소`로 패키지 설치 주소를 지정하여 설치가능

<br>

## 2. 패키지 여러개 설치
- 명령어 : `npm install 패키지1 2 3..`
- 여러개의 패키지를 동시에 설치한다.
  
<br>

## 3. 전역 설치
- 명령어 : `npm install -global 패키지명`
- npm 자체가 설치되어 있는 폴더에 패키지를 설치하여 어디서나 참조할 수 있게 한다.
> 맥, 리눅스의 경우 , sudo를 붙여서 sudo npm i --global 패키지명 으로 설치해야 할 수 있다.

> `npm i -g 패키지명` 으로 축약가능

<br>

## 4. 개발용 설치
- 명령어 : `npm install --save-dev 패키지명`
- package.json 파일의 devDependencies 항목에 추가되도록 설치한다
> `npm i -D` 로 축약 가능


<br>

## 5. package.json 파일을 사용한 설치
- 명령어 `npm install`
- package.json 파일이 저장된 폴더에서 명령어를 실행하면 , dependencies 항목에 기록된 패키지들을 모두 자동으로 설치해 준다.

<br>

## 6. 패키지 삭제
- 명령어 : `npm uninstall 패키지명`
- node_modules 폴더에 설치된 패키지를 삭제한다. package.json의 dependencies 항목에서도 제거 된다.
> npm rm 패키지명 으로 축약가능
