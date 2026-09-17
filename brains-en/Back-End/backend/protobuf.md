# Protocol Buffers

Protocol buffers are a language-neutral, platform-neutral, extensible mechanism for serializing structured data.

Developed and maintained by Google, they are smaller and faster than data formats like JSON.

Protobufs can be generated using `.proto` files, as shown below.

```proto
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

And they can be used as follows.

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

Using protobufs offers the following advantages.

- Compact data storage
- Fast parsing speed
- Usable across multiple programming languages
- Optimized functionality through auto-generated classes

These compact and fast protobufs are used in environments like gRPC, Google Cloud, and Envoy Proxy.

<br>

However, are protobufs always good? Let's also look at situations where they might not be ideal.

Protobufs can be a poor choice for **features involving large messages**. This is because protobufs tend to load and use entire messages from memory. Therefore, for data exceeding megabytes, it's recommended to look for other solutions.

**Ensuring equality in serialized data is difficult.** When a file written with protobuf is serialized, binary serializations cannot compare two messages without parsing them.

Messages are not compressed. While messages can be compressed like other files or using gzip, special-purpose compression algorithms, such as those used for JPEG or PNG, generate much smaller files for the appropriate data types.

For many features involving large-scale operations, they do not achieve maximum efficiency in terms of both size and speed. This means they are not useful for tasks like large-volume batch processing.

Furthermore, protobufs do not support object-orientation.

Protobuf buffer messages are not inherently self-describing, but they possess a reflective schema to implement their own internal description, and cannot be fully interpreted without access to the corresponding `.proto` file. This is a characteristic of protobufs that allows messages to be self-describing.

<br>

### Protobuf Usage

Code generated from protobufs, whether from files or streams, provides various utility methods.

Such as extracting individual values, checking for data existence, or serializing back into file or stream-type data.

Define it via a proto file as follows,

```proto
message Person {
  optional string name = 1;
  optional int32 id = 2;
  optional string email = 3;
}
```

then create it in Java code using a builder,

```java
Person john = Person.newBuilder()
    .setId(1234)
    .setName("John Doe")
    .setEmail("jdoe@example.com")
    .build();
output = new FileOutputStream(args[0]);
john.writeTo(output);
```

And you can also use it in C++ as shown below. This means it's compatible across multiple languages!

```cpp
Person john;
fstream input(argv[1], ios::in | ios::binary);
john.ParseFromIstream(&input);
int id = john.id();
std::string name = john.name();
std::string email = john.email();
```

The reference material is the [official protocol buffers documentation](https://protobuf.dev/overview/#syntax).
