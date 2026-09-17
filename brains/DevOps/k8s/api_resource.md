# 쿠버네티스 API 리소스

API 리소스는 쿠버네티스가 관리할 수 있는 오브젝트의 한 종류이다.

기본적으로 pod, configmap, node 등 다양한 종류가 있다. 

이런 리소스들을 객체화(인스턴스화) 시킨 것이 `오브젝트`라고 한다.

현재 쿠버네티스 클러스터가 지원하는 api 리소스 목록들을 전부 출력할 수 있는 명령어이다.

어떤 리소스를 생성, 관리할 수 있는지 한 번에 확인할 수 있다.

```zsh
kubectl api-resources
```

이러한 내용에 대한 스팩, 용도, 목적으로 사용할 수 있는지에 대한 설명을 해주는 명령어는 아래와 같다.

```zsh
kubectl explain pod
```

먼저 미니 쿠배를 실행 시켜보자. 굉장히 아기자기하고 귀여운 이모티콘이 나타나며 실해오디는 것을 확인할 수 있다.

```zsh
minikube start
```

또한 스테이터스 명령어를 실행해보면 클러스터의 상태를 확인할 수 있다.
```zsh
minikube status
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2FbZANSM%2FbtruYSkCVfG%2FgluihlOkkXQvKLKMmlZAs0%2Fimg.png)

처음의 api-resources 명령어를 실행해 보면 아래와 같은 화면이 나타난다. 지금 켑처가 전부가 아닌 엄청 긴 문장으로 나타나니까 참고 바란다. 이름과 버전을 살펴봐야한다.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb1D9hp%2FbtruYTKC9pl%2FmxyzJnU588q6IY9Xfss9P1%2Fimg.png)

아래의 명령어를 입력하면 쿠베ctl로 현재 클러스터의 모든 name으로부터 스테이터스를 확인할 수 있다.

```zsh
kubectl get name --all-namespaces
```

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fb672G8%2FbtruVJ927JT%2FV2pEiHts7cE78aKuXdGGUk%2Fimg.png)

다양한 오브젝트를 관리하기 위해 YAML형식으로 관리를 하고 있다. 루트키는 어떤 그룹에 속하고 어떤 버전인지 확인할 수 있는 apiVersion, 오브젝트가 어떤 리소스인지 확인할 수 있는(타입을 알려주는) kind, 식별하기 위한 정보(이름, 네임스페이스, 레이블 등등) 가 있는 metadata, 마지막으로 오브젝트가 어떤 데이터를 가지고 싶어하는가에 대한 spec가 있다.

특이점은 API리소스에 따라서 spec대신 data(ex, configmap, secret의 경우), rules(Role의 경우), subjects등등 다른 속성을 바탕으로 사용할 수 도 있다.

 

metadata안에서 중요한 내용으로는 Labels과 annotations가 있다. 따로 정의를 하지 않아도 모든 쿠버네티스 오브젝트는 이 정보들을 가질 수 있다. 하지만 Labels은 오브젝트를 식별하기 위한 목적이며(소유자가 누구인지, 어떤 타입인지, 어떤 app인지 등등), 내부 기능에서 Labels Selector기능을 제공하기에 식별을 위해서는 어지간하면 설정해 두는 것이 중요하다. annotations값은 조금 다른데 식별이 아닌, 오브젝트를 어떻게 처리할 것인지를(쿠버네티스 에드온이 읽고) 결정하기 위한 설정 용도로 사용을 한다.