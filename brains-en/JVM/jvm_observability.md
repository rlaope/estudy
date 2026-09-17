# Anatomy of JVM Observability Tool Principles

### async-profiler's STW-free CPU Profiling Principle (`perf_events`, `AsyncGetCallTrace`)

Existing JMX or JVMTI's `GetAllStackTraces()` API required the JVM to reach a safepoint to safely collect thread call stacks, which inevitably caused STW.

async-profiler solves this problem by combining OS hardware/software events with non-standard APIs within the JVM.

#### Mechanism Flow: OS Kernel -> JVM

1.  `perf_events` **Interrupt Configuration**: async-profiler uses the Linux kernel's `perf_event_open` system call to configure hardware timer interrupts to occur at a specific frequency (e.g., 99hz, 99 times per second).
2.  **Signal Handler Invocation (OS Level)**: When an interrupt occurs, the Linux kernel suspends the execution of the currently running thread and asynchronously invokes a `SIGPROF` or custom signal handler within that thread's context.
3.  **AsyncGetCallTrace (ASGCT) Invocation (JVM Level)**: async-profiler's signal handler calls AsyncGetCallTrace, an internal API that the HotSpot JVM exports.
4.  **Lock-Free Stack Walking**: ASGCT does not acquire locks or wait for safepoints. Based on the received `ucontext` register state, PC, SP, etc., it checks the current thread's state (`_thread_in_Java`, `_thread_in_native`) and traces back compiled code (JIT, interpreter frames, native frames) by following the Frame Pointer to construct the stack trace.

In Java 21, due to the influence of JEP 436 and others, ASGCT has been improved to correctly trace Virtual Thread stacks.

ASGCT C++ Signature and Data Structures

```cpp
// Implemented internally in Hotspot, e.g., forto/prims/asgct.cpp
typedef struct {
    jint lineno; // Line number or BCI
    jmethodID method_id; // Method ID of the executing method
} ASGCT_CallFrame

typedef struct {
    JNIEnv *env_id;
    jint num_frames;                  // Number of collected frames
    ASGCT_CallFrame *frames;          // Pointer to array of frames
} ASGCT_CallTrace;

// Function pointer called from asynchronous signal handler
void AsyncGetCallTrace(ASGCT_CallTrace *trace, jint depth, void* ucontext);
```

<br>

### NMT (Native Memory Tracking) C++ Level Allocation Hooking Mechanism

Native Memory Tracking tracks native memory areas used by the JVM itself at the C/C++ level, not the Java heap space. This includes metaspace, thread stacks, JIT code cache, GC internal data structures, and more.

#### Memory Allocation Interception Principle

HotSpot JVM code does not directly call standard C library functions like `::malloc` or `::free`. Instead, it is forced to use wrapper functions like `os::malloc` defined in `src/hotspot/share/runtime/os.hpp`. NMT places hooks inside these wrapper functions.

1.  `os::malloc` Call: JVM internal code calls `os::malloc(size, mtClass)` to request memory. Here, `mtClass` is a memory type flag.
2.  **MemTracker** Recording (`MallocTracker`): If NMT is enabled (`-XX:NativeMemoryTracking=summary` or `detail`), the `MemTracker::record_malloc()` function is called inside `os::malloc`.
3.  **Tracking Header Attachment**: More memory is allocated than the actual requested size, specifically for the NMT header (typically 8 or 16 bytes). This header records the allocation size and memory type information.
4.  **Call Stack Capture (Detail Mode)**: In Detail mode, the program counter (PC) is captured from the current C++ call stack, stored in a `NativeCallStack` object, and registered in the NMT memory map registry.

### `os::malloc` Internal Implementation

```cpp
void* os::malloc(size_t size, MEMFLAGS flags, const NativeCallStack& stack) {
  // 1. Calculate actual allocation size including NMT header
  size_t alloc_size = size + NMT_MallocTracker::malloc_header_size();

  // 2. Call libc's malloc
  void* ptr = ::malloc(alloc_size);
  if (ptr == NULL) return NULL;

  // 3. Record allocation details in NMT
  if (MemTracker::tracking_level() >= NMT_summary) {
    return MemTracker::record_malloc(ptr, size, flags, stack);
  }
  return ptr;
}
```

`jcmd` native memory tracking output format example (detail mode)

```
[0x00007f8b90123000] os::malloc(unsigned long, MEMFLAGS, NativeCallStack const&)+0x45
[0x00007f8b90456000] BitMap::allocate_map(unsigned long)+0x23
[0x00007f8b90789000] G1ConcurrentMark::allocate_internal_bitmaps()+0x89
                             (malloc=4096KB type=GC #12)
```

<br>

### JVMTI and ASM/ByteBuddy Low-Level Bytecode Manipulation Flow

This is a core technology that allows APM agents to measure response times or queries without modifying application code. In a Java 21 environment, the process of reassembling bytecode at class load time is as follows.

#### Manipulation Flow at Class Load Time (JVM Perspective)

1.  **Agent Registration**: At startup, `premain` is executed by the `-javaagent:apm.jar` option, and `Instrumentation.addTransformer()` is called.
2.  **JVMTI Event Activation**: Internally, the JVM calls `SetEventNotificationMode` of the JVMTI interface to activate the `JVMTI_EVENT_CLASS_FILE_LOAD_HOOK` event.
3.  **Bytecode Delivery**: Just before the `ClassLoader` reads a class file (e.g., `MyController.class`) from disk and delivers it to the JVM, the JVM passes the original `byte[]` array to the registered Transformer.

`ClassFile` Structure and ASM Reassembly

The ClassFile format defined in the Java Virtual Machine Specification (JVMS) is a strict binary structure, not plain text.

```
ClassFile {
    u4             magic;               // 0xCAFEBABE
    u2             minor_version;
    u2             major_version;       // Java 21 is 65
    u2             constant_pool_count;
    cp_info        constant_pool[constant_pool_count-1];
    u2             access_flags;
    u2             this_class;
    // ... [fields, methods, attributes]
}
```

ASM or ByteBuddy parses this original `byte[]` array and traverses it as a logical tree-like `ClassNode` or an event-driven `ClassVisitor`.

1.  **Constant Pool Expansion**: If new method references like `ApmTracer.startTrace()` or strings are needed for APM code insertion, ASM increases the size of the existing bytecode array's `constant_pool` and adds new constant entries such as `CONSTANT_Methodref_info`.
2.  `Code` Attribute Modification (Method Bytecode Manipulation): It finds the `Code` attribute within the `method_info` of the method to be modified and disassembles the actual JVM instruction opcode array.
    *   Inserts the trace start instruction `INVOKESTATIC` at the method entry point.
    *   Inserts the trace end instruction `INVOKESTATIC` just before `RETURN` or `ATHROW`.
3.  `StackMapTable` Recalculation (Java 21 Requirement): This is the most challenging low-level task. If code containing branch instructions like `IFEQ` or `GOTO` is manipulated, bytecode offsets change, and JVM verifiers since Java 7 require a `StackMapTable` for fast type verification. ASM analyzes the modified instruction array, recalculates the type states of local variables and the operand stack for each frame, and then serializes them into binary.
4.  **New Byte Array Generation**: The manipulated constant pool, modified `max_stack`, `max_locals`, updated `StackMapTable`, and bytecode are then serialized back into a sequential `byte[]` using `ClassWriter.toByteArray()` according to JVMS specifications and returned to the JVM.

```java
import java.lang.instrument.ClassFileTransformer;
import java.security.ProtectionDomain;

public class ApmTransformer implements ClassFileTransformer {
    @Override
    public byte[] transform(ClassLoader loader, String className, 
                            Class<?> classBeingRedefined, 
                            ProtectionDomain protectionDomain, 
                            byte[] classfileBuffer) { // <-- JVM passes the original byte array
        
        if (className.equals("com/example/MyService")) {
            // Parse the original byte array with ASM's ClassReader
            org.objectweb.asm.ClassReader cr = new org.objectweb.asm.ClassReader(classfileBuffer);
            
            // COMPUTE_FRAMES: Automatically recalculates StackMapTable, max_stack, and max_locals
            org.objectweb.asm.ClassWriter cw = new org.objectweb.asm.ClassWriter(cr, org.objectweb.asm.ClassWriter.COMPUTE_FRAMES);
            
            // Insert bytecode instructions at desired locations via a custom ClassVisitor
            ApmClassVisitor cv = new ApmClassVisitor(cw);
            cr.accept(cv, 0);
            
            // Return the reassembled new class binary array
            return cw.toByteArray(); 
        }
        
        // For classes not to be manipulated, return null (keep original)
        return null; 
    }
}
```
