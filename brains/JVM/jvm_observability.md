# JVM Observability 도구별 원리 해부학

### async-profiler의 stw없는 cpu 프로파일링 원리 (`perf_events`, `AsyncGetCallTrace`)

기존의 JMX나 JVMTI의 `GetAllStackTraces()` api는 스레드의 콜스택을 안전하게 수집하기 위해 jvm safepoint에 도달하게 만들어야 했으며, 이는 필연적으로 stw를 유발했다.

async-profiler는 os의 하드웨어/소프트웨어 이벤트와 jvm 내의 비표준 api를 결합하여 이문제를 해결한다.

#### 메커니즘 동작 흐름 os kernel -> jvm

1. `perf_events` **인터럽트 설정**: async-profiler는 linux 커널의 `perf_event_open` 시스템 콜을 사용하여 특정 주기 (예: 99hz, 1초에 99번)로 하드웨어 타이머 인터럽트를 발생시키도록 설정한다.
2. **시그널 핸들러 호출 os level**: 인터럽트가 발생하면 리눅스 커널은 현재 실행중인 스레드의 실행을 일시 중단하고 해당 스레드 컨텍스트 내에서 `SIGPROF` 또는 커스텀 시그널 핸들러를 비동기적으로 호출한다.
3. **AsyncGetCallTrace(ASGCT) 호출 JVM 레벨:**: async-profiler의 시그널 핸들러는 hotspot jvm이 외부로 읷스포트해둔 내부 api인 AsyncGetCallTrace를 호출한다
4. **Lock-Free 스택 워킹:**: ASGCT는 락을 획득하거나 safepoint를 기다리지 않는다. 전달받은 `ucontext` 레지스터 상태, pc, sp등을 바탕으로 현재 스레드의 상태 `_thread_in_Java`, `_thread_in_native` 등을 확인하고 Frame Pointer를 따라가며 컴파일된 코드 JIT, 인터프리터 프레임, 네이티브 프레임을 역추적하여 스택트레이스를 구성한다.

Java 21에서는 JEP 436등의 영향으로 ASGCT가 Virtual Thread의 스택도 정상적으로 추적할 수 있도록 개선되었다.

ASGCT C++ 시그니처 및 데이터 구조

```cpp
// Hotspot 내부 forto/prims/asgct.cpp 등에 구현됨
typedef struct {
    jint lineno; // 라인 번호 또는 BCI
    jmethodID method_id; // 실행중인 메서드 ID
} ASGCT_CallFrame

typedef struct {
    JNIEnv *env_id;
    jint num_frames;                  // 수집된 프레임 수
    ASGCT_CallFrame *frames;          // 프레임 배열 포인터
} ASGCT_CallTrace;

// 비동기 시그널 핸들러에서 호출되는 함수 포인터
void AsyncGetCallTrace(ASGCT_CallTrace *trace, jint depth, void* ucontext);
```

<br>

### NMT(Native Memory Tracking)의 cpp 레벨 할당 후킹 메커니즘

native memory tracking은 java 힙 공간이 아닌 jvm 자체가 c/cpp 레벨에서 사용하는 네이티브 메모리 영역. 메타스페이스, 스레드 스택, JIT 코드 캐시, gc 내부 구조체 등을 추적한다.

#### 메모리 할당 인터셉트 원리

HotSpot JVM 코드는 표준 c 라이브러리 `::malloc` 이나 `::free`를 직접 호출하지 않는다 대신 `src/hotspot/share/runtime/os.hpp`에 정의된 wrapper 함수인 `os::malloc` 등을 사용하도록 강제되어있다. NMT는 이 래퍼 함수 내부에 훅을 배치한다.

1. `os::malloc` 호출: JVM 내부 코드에서 `os::malloc(size, mtClass)`를 호출하여 메모리를 요청한다. 여기서 mtClass는 메모리 타입 플래그다
2. **MemTracker** 기록(`MallocTracker`): NMT가 켜져있으면 (`-XX:NativeMemoryTracking=summary` or `detail`), `os::malloc` 내부에서 `MemTracker::record_malloc()` 함수가 호출된다.
3. **Tracking Header 부착**: 실제 요청한 크기보다 NMT 헤더(일반적으로 8 아니면 16바이트)만큼 메모리를 더 할당한다. 이 헤더에는 할당 크기와 메모리 타입 정보가 기록된다.
4. **Call Stack 캡처 (Detail 모드)**: Detail 모드일 경우 현재 cpp 콜스택에 program counter(pc)를 캡쳐하여 `NativeCallStack` 객체에 저장하고 이를 NMT 메모리 맵 레지스트리에 등록한다.

### `os::malloc` 내부 구현

```cpp
void* os::malloc(size_t size, MEMFLAGS flags, const NativeCallStack& stack) {
  // 1. NMT 헤더를 포함한 실제 할당 크기 계산
  size_t alloc_size = size + NMT_MallocTracker::malloc_header_size();

  // 2. 실제 libc의 malloc 호출
  void* ptr = ::malloc(alloc_size);
  if (ptr == NULL) return NULL;

  // 3. NMT에 할당 내역 기록
  if (MemTracker::tracking_level() >= NMT_summary) {
    return MemTracker::record_malloc(ptr, size, flags, stack);
  }
  return ptr;
}
```

`jcmd` native memory tracking 출력 포맷 예시 detail 기준

```
[0x00007f8b90123000] os::malloc(unsigned long, MEMFLAGS, NativeCallStack const&)+0x45
[0x00007f8b90456000] BitMap::allocate_map(unsigned long)+0x23
[0x00007f8b90789000] G1ConcurrentMark::allocate_internal_bitmaps()+0x89
                             (malloc=4096KB type=GC #12)
```

<br>

### JVMTI와 ASM/ByteBuddy의 로우레벨 바이트코드 조작 흐름

APM 에이전트가 애플리케이션 코드를 수정하지 않고 응답 시간이나 쿼리를 측정할 수 있는 핵심기술이다 java 21 환경에서 클래스 로드 시점에 바이트 코드를 재조립하는 과정은 다음과 같다.

#### 클래스 로드 시점의 조작 흐름 JVM 관점

1. **Agent 등록**: 시작시 `-javaagent:apm.jar` 옵션에 의해 `premain`이 실행되고 `Instrumentation.addTransformer()` 가 호출된다.
2. **JVMTI 이벤트 활성화**: 내부적으로 JVM은 JVMTI 인터페이스의 `SetEventNotificationMode`를 호출하여 `JVMTI_EVENT_CLASS_FILE_LOAD_HOOK` 이벤트를 활성화한다.
3. **바이트코드 전달**: `ClassLoader`가 클래스파일(예: `MyController.class`)를 디스크에 읽어 JVM 전달하기 직전, JVM은 등록된 Transformer에게 원본 byte[] 배열을 넘겨준다.

`ClassFile` 구조와 ASM 재조립

Java Virtual Machine Specification JVMS에 정의된 ClassFile 포맷은 단순 텍스트가 아닌 엄격한 바이너리 구조체다.

```
ClassFile {
    u4             magic;               // 0xCAFEBABE
    u2             minor_version;
    u2             major_version;       // Java 21은 65
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    // ... [fields, methods, attributes]
}
```

ASM이나 ByteBuddy는 이 원본 `byte[]` 배열을 파싱하여 논리적은 트리형태 classNode 또는 이벤트 드리븐 방식 ClassVisitor으로 순회한다.

1. **Constant Pool 확장**: APM 코드를 삽입하기 위해 새로운 메서드 참조 `ApmTracer.startTrace()`나 문자열에 필요하면, ASM은 기존 바이트배열의 `constant_pool` 크기를 늘리고 새로운 상수 엔트리 `CONSTANT_Methodref_info`등을 추가한다.
2. `Code` Attribute 수정 (메서드 바이트코드 조작): 수정하려는 메서드의 method_info 내부에있는 Code 속성을 찾아서 실제 jvm 명령어 opcode 배열을 분해한다.
   1. 메서드 진입부 추적 시작 명령어 INVOKESTATIC을 삽입한다
   2. RETURN or ATHROW 직전에 추적 종료 명령어 INVOKESTATIC을 삽입한다
3. `StackMapTable` 재계산 (java 21 필수 요소): 가장 까다로운 로우레벨 작업이다 분기문 IFEQ, GOTO 등이 포함된 코드가 조작되면 바이트코드 오프셋이 변경되고 java 7이후 jvm verifier는 빠른 타입 검증을 위해 `StackMapTable`을 요구한다 ASM은 변경된 명령어 배열을 분석하여 프레임별 로컬변수와 오퍼랜드 스택의 타입상태를 다시 계산하여 바이너리로 직렬화한다.
4. **새로운 바이트 배열 생성**: 조작된 상수 풀, 변경된 max_stack, max_locals 갱신된 StackMapTable및 바이트코드를 JVMS 규격에 맞게 다시 순차적인 byte[] 로 직렬화 ClassWriter.toByteArray()하여 JVM에 반환한다.


```java
import java.lang.instrument.ClassFileTransformer;
import java.security.ProtectionDomain;

public class ApmTransformer implements ClassFileTransformer {
    @Override
    public byte[] transform(ClassLoader loader, String className, 
                            Class<?> classBeingRedefined, 
                            ProtectionDomain protectionDomain, 
                            byte[] classfileBuffer) { // <-- JVM이 원본 바이트 배열을 전달
        
        if (className.equals("com/example/MyService")) {
            // ASM의 ClassReader로 원본 바이트 배열 파싱
            org.objectweb.asm.ClassReader cr = new org.objectweb.asm.ClassReader(classfileBuffer);
            
            // COMPUTE_FRAMES: StackMapTable과 max_stack, max_locals를 자동으로 재계산
            org.objectweb.asm.ClassWriter cw = new org.objectweb.asm.ClassWriter(cr, org.objectweb.asm.ClassWriter.COMPUTE_FRAMES);
            
            // 커스텀 ClassVisitor를 통해 원하는 위치에 바이트코드 명령어 삽입
            ApmClassVisitor cv = new ApmClassVisitor(cw);
            cr.accept(cv, 0);
            
            // 재조립된 새로운 클래스 바이너리 배열 반환
            return cw.toByteArray(); 
        }
        
        // 조작하지 않을 클래스는 null 반환 (원본 유지)
        return null; 
    }
}
```