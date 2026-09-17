# [Kotlin] kapt란?

![](image/kapt_0.jpg)

### kapt

kapt는  Kotlin언어를 사용해 **annotaion processor를 실행하기 위한 Gradle 플러그인이다.**

### annotation processor

annotation processor는 컴파일 타임에 소스 코드에 있는 주석을 읽어들이고, 그에 따라 추가적인 코드를 생성하거나 변형할 수 있다. 이를 통해서 자동화된 코드 생성이나 런타임에서 퍼포먼스 향상 등 다양한 이점을 얻을 수 있다.

kapt 플러그인은 kotlin-gradle-plugin 플러그인을 사용하는 코틀린 프로젝트에서만 사용할 수 있으며, kapt를 사용하기 위해서는 먼저 kotlin-kapt 라이브러리를 프로젝트 의존성이 추가해야한다.

```
plugins {
    id("org.jetbrains.kotlin.jvm") version "1.5.31"
    id("org.jetbrains.kotlin.kapt") version "1.5.31"
}

dependencies {
    implementation("com.example:some-library:1.0")
    kapt("com.example:some-library-processor:1.0")
}
```

위 예제에서는 kotlin-kapt 플러그인을 추가하고 com.example:some-library 라이브러리를 사용해 해당 라이브러리 대한 annotaion processor를 com.example:some-library-processor 라이브러리로 지정한다.

### kapt tesk

kapt 플러그인에서 컴파일 타임에 annotation processor를 실행하기 위해 다음과 같은 tesk를 생성한다.

* **kaptKoltin** : 코틀린 소스 코드와 annotation processor를 사용해 추가 코드를 생성한다
* **compileKotlin**: kaptKotlin tesk를 의존하며, 생성된 추가 코드를 포함해 kotlin 소스 코드를 컴파일한다.
* **compileJava**: kaptKotlin tesk를 의존하며, annotation processor를 사용해 생성된 java 소스 코드를 컴파일한다.

### kapt, anntation processor 차이와 장점

kapt와 annotation processor는 서로 밀접하게 관련된 개념이다.

***annotation processor**는 자바에서 제공하는 기능으로, 컴파일 타임에서 소스 코드에 있는 주석을 읽어들이고, 그에 따라 추가적인 코드를 생성하거나 변형할 수 있다. 이를 통해 자동화된 코드 생성이나 런타임에서 퍼포먼스를 향상 등 다양한 이점을 얻을 수 있다.*

**kapt**는 코틀린에서 annotation processor를 실행하기 위한 gradle 플러그인이라고 했다 즉, kapt는 코틀린에서 annotation processor를 실행하기 위한 플러그인인 것이다.

그냥 kotlin에서도 *annotation processor를 사용할 수 있긴하다*. 하지만, kapt를 사용하면 gradle에 annotation processor를 관리할 수 있으므로 더 편리하다. kapt를 **사용하면 자동으로 annotation processor를 실행하고, 생성된 코드를 컴파일할 수 있다.** 또한, kapt를 사용하면 kotlin 전용 annotation processor를 사용할 수 있으며, annotation processor의 **코드 생성 경로나 클래스 패스 설정 등을 간편하게 할 수 있다.**

따라서, kotlin을 사용하는 경우 kapt를 사용하는 것이 좀 더 편리하다. 그러나 annotation processor가 기존 java 라이브러리를 사용할 때도 필요한 경우가 있으므로 필요한 경우에는 직접 annotation processor를 사용할 수 있다.
