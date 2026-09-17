# 자바 Cleaner, PhantomReference class

자바는 과거 finalize() 메서드를 사용해서 가비지 컬렉션에 의해 객체가 회수되기 직전에 실행되는 코드를 작성할 수 있었다. 그러나 finalize()는 실행 시점을 정확히 예측할 수 없으므로 사용하는 것이 권장되지 않아 Java 9부터는 deprecated가 되었다.

그리고 Java 12에서 Cleaner, PhantomReference를 사용한 대안이 제시되었다.

### Cleaner 

Cleaner 클래스는 객체에 도달할 수 없을 때, 그리고 가비지 수집되기 직전에 수행해야 하는 정리작업을 등록하는 데 사용된다. 가비지 수집기에서 개체를 회수하기 전에 실행해야 하는 리소스 정리 또는 기타 작업을 수행하는 방법을 제공한다. Cleaner의 주요 목적은 예측 불가능성과 잠재적인 성능 문제로 인해 권장되지 않는 finalize() 메서드의 사용을 대체하는 것이다.

```java
import java.lang.ref.Cleaner;

public class MyClass implements AutoCloseable {
    private static final Cleaner cleaner = Cleaner.create();

    private final SomeResource resource;

    public MyClass() {
        resource = new SomeResource();
        cleaner.register(this, new MyCleanerAction(resource));
    }

    @Override
    public void close() {
        // Perform any additional cleanup actions before the object is garbage collected
        // This method is called when the object is explicitly closed by the developer
    }

    private static class MyCleanerAction implements Runnable {
        private final SomeResource resource;

        public MyCleanerAction(SomeResource resource) {
            this.resource = resource;
        }

        @Override
        public void run() {
            // Perform cleanup actions when the object is garbage collected
            resource.cleanup();
        }
    }
}
```

위의 예에서 MyClass의 인스턴스에 연결할 수 없게되면 연결된 MyCleanerAction이 실행되어 SomeResource 개체에 대한 정리 작업을 수행한다.

### PhantomReference

PhantomReference 클래스는 SoftReference, WeakRefernce 및 FinalReference와 함께 Java에 사용할 수 있는 4가지 참조 클래스중 하나입니다. PhantomReference는 완료를 위해 대기열에 추가되었지만 아직 가비지 수집기에 의해 회수되지 않은 개체를 추적하는데 유용합니다. 

다른 참조 유형과 달리 PhantomReference는 참조된 객체가 도달할 수 없을 때 가비지 수집되는 것을 막지 않습니다. 대신 개체가 종료를 위해 대기열에 추가되고 회수되려고 할 때 알림을 받을 수 있습니다.

```java
import java.lang.ref.PhantomReference;
import java.lang.ref.Reference;
import java.lang.ref.ReferenceQueue;

public class PhantomReferenceExample {
    public static void main(String[] args) {
        Object referent = new Object();
        ReferenceQueue<Object> referenceQueue = new ReferenceQueue<>();
        PhantomReference<Object> phantomReference = new PhantomReference<>(referent, referenceQueue);

        // At this point, referent is still reachable from the main method

        // Explicitly remove the strong reference to the object
        referent = null;

        // Force garbage collection (for demonstration purposes)
        System.gc();

        // The phantomReference is now enqueued in the referenceQueue
        Reference<?> polledReference = referenceQueue.poll();
        if (polledReference == phantomReference) {
            System.out.println("PhantomReference enqueued for finalization");
        }
    }
}
```

위의 예에서 ReferenceQueue는 선택 사항이며 개체가 완료를 위해 대기열에 추가될 때 알림을 받는 데 사용할 수 있습니다. 이 알림이 필요하지 않은 경우 ReferenceQueue를 제공하지 않고 PhantomReferece를 만들 수 있습니다.

PhantomReference는 일반적으로 사용자가 지정 정리 메커니즘 생성 또는 개체 부활 구현과 같은 고급 시나리오에서 사용된다는 점은 주목할 가치가 있습니다. 대부분의 일반적인 사용 사례에서는 SoftReference, WeakReference 또는 Cleaner로도 충분합니다.