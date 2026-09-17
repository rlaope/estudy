# JVM Heap Object Headers on Internals

When examining an object's memory layout, object data exists in addition to instance data.

As seen in the HotSpot JVM object memory structure, all Java objects have memory space for headers and alignment padding, in addition to instance data (payload).

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FcaT8fy%2FbtsQ1XlvM5O%2FAAAAAAAAAAAAAAAAAAAAABJkufEpLzAlOU9P_3DliY3pMc0dYRg9qMWNVvYJBSub%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3D6l%252BJukuTTSTlgAODct%252F6iwbPgAU%253D)

The information above comes together to form an object, so let's explore object headers here.

<br>

## Mark Word

The Mark Word is not a pointer but a bit field that holds data itself, storing metadata uniquely managed for each object.

Typically, the hash code points to the object's memory address, is uniquely allocated per object, and is assigned to the Mark Word.

Additionally, there are marking bits for generational GC age and whether an object has been collected by GC.
- Lock information
- GC age-bit
- Hashcode
- Unused: unused region

The above information is stored in the Mark Word, encoded in sequential bit units. The hash code initially exists as 0 when the object is created and is allocated when `hashCode` is first called.

<br>

## Class Word

The Class Word is a pointer that points to the memory address where the class metadata for that object resides.

It is class-dependent, and since identical metadata is managed as a single entity for all objects, it is also referred to as **shared class metadata**.

Typically, class information, `kclass`, is one of the pieces of information obtained via the Class Word, indicating which class an object is an instance of.
- Class information: indicates which class the object belongs to (`kclass`)
- VTable: address of the virtual method table (space to support polymorphism)
- Field information: field offset, type information
- Static variables: static field

For Java developers, the `Class` type is provided to represent class information, offering functionalities such as obtaining desired information about a class and creating objects.

However, the `klass` managed internally by the JVM is a different concept from the `Class` familiar to developers.

Internally, when the JVM **builds and loads a class, it requires metadata about the class**, and the class information extracted from the class file is precisely `kclass`.

Based on the obtained `kclass` information, the JVM creates a mirrored replica for Java, which is then returned to Java as a `Class` object.

Since the `kclass` word contains metadata shared by all classes, it is stored and managed in the JVM's Metaspace region.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FbnKgRq%2FbtsQ4QdMYHC%2FAAAAAAAAAAAAAAAAAAAAAM3k0VGsHl7cYAkuBC12bBzNYmeofrFN0s7MbbonsOxI%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DyApkjuUajdOnfsSMk6XnqZvpnQs%253D)

A simplified representation of how these object header parts are structured in memory is as follows.

The Mark Word exists as metadata within the heap, but the Class Word is a pointer to class information, ultimately pointing to the Metaspace region.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FcGUnd2%2FbtsQ2SqpEXc%2FAAAAAAAAAAAAAAAAAAAAAJHjNJPE_FEK1KML4-4wKS-QX2eDQiXuZWFZF22cGn2C%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3D6T%252FIHHzbkjMwmRjaYiuyouP6Xds%253D)

<br>

## Array Length

For Java arrays, the array length is also stored in the header. The object type stored in the header corresponds to the type of elements contained in the array.

Therefore, to accurately calculate the memory size occupied by an array object, the array length must also be known.

Array length information does not exist if the object is not an array. Thus, the length of the object header can vary depending on whether it is an array.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdna%2FzpqcN%2FbtsQ4XjDBRt%2FAAAAAAAAAAAAAAAAAAAAAE2504q3Zrdm264RhXJqA7T50yHn_zrfR3XubwSMUxFv%2Fimg.png%3Fcredential%3DyqXZFxpELC7KVnFOS48ylbz2pIh7yKj8%26expires%3D1761922799%26allow_ip%3D%26allow_referer%3D%26signature%3DaCeaNjC%252BMUQuoIzOg3yg4nLLpYM%253D)

<br>

## Instance Data, Alignment Padding

The second part of the object layout, instance data, is the actual information an object holds.

For example, various types of fields defined in program code, the presence of a parent class, and all fields defined in the parent class are recorded in this section.

If the `+XX:CompactFields` parameter is set to `true` (default), shorter fields from subclasses are reportedly inserted between superclass variables, saving some space.

The alignment padding section may not exist and serves only to reserve space without special meaning.

In HotSpot's automatic memory management system, an object's starting address must be an 8-byte integer.

In other words, the size of all objects must be a multiple of 8 bytes, so if the instance data does not meet this condition, it is **filled with padding**.
