# package.json 생성
- 수많은 패키지를 다운받다보면 관리가 어려워진다.
- 그래서 프로젝트마다 package.json 파일을 만들어서 패키지 목록 및 버전을 관리한다.
> 프로젝트를 생성하자마자 package.json을 만들어주고 시작하길 권장한다.

- 명령어 : `npm init`

<br>

1. package name 등의 정보를 입력해준다. 필요 없는 항목은 엔터키로 스킵가능
2. 마지막에 Is this OK? 항목에서 yes 입력한다.

- 정상적으로 파일을 생성했다면 아래의 3가지 파일 및 폴더가 생성될 것이다.
> 단 , 외부 패키지를 전혀 설치하지 않은 초기 상태라면 node_modules 폴더, package-lock.json 파일은 생성되지 않을 수 있다.

<br>

## 1. package.json 파일
- 설치된 패키지를 `dependencies` 항목에서 관리한다.

```json
{
  "name" : "node.package",
  "version" : "1.0.0",
  "description" : "my node package",
  "main" : "index.js",

  "scripts" : {
    "test" : "echo / Error: no test specifed\ && exit 1 "
  },
  "dependencise" : {
    "socket.io" : "^4.1.2"
  }
}
```

<br>

## node-modules 폴더
- 설치한 패키지가 실제로 저장되어 있는 폴더이다.
- 설치한 패키지가 의존하는 다른 패키지도 함께 설치되어야 저장된다.

<br>

## package-lock.json 파일
- 설치한 패키지 + 의존하는 패키지 항목을 모두 관리한다.