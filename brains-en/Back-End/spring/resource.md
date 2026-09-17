# Spring Resource Interface, Resource Abstraction and Implementations

## Resource
Spring provides additional implementations to overcome the limitations of `java.net.URL` (e.g., accessing within the classpath or relative paths).

### Resource Interface
```java
public interface Resource extends InutStreamSource {
	boolean exists();
	boolean isReadable();
	boolean isOpen();
	boolean isFile();

	URL getURL() throws IOException;
	URI getURI() throws IOEXception;
	File getFile() throws IOException;
	ReadableByteChannel readableChannel() throws IOException;
	
	long contentLength() throws IOException;
	long lastModified() throws IOException;
	
	Resouce createRelative(String relativaPath) throws IOException;
	String getFilename();
	String getDescription();
}
```

## Resource Implementations

### UrlResource
Reads resources based on a URL.
  
Supported protocols include http, https, ftp, file, jar.

### ClassPathResource
When the supported prefix is `classpath:`, it reads resources based on the classpath.

### FileSystemResource
Reads based on the file system.

### ServeltContextResource
Finds resources by relative path from the web application root.

### InputStreamResource, ByteArrayResource
As the name suggests, these are implementations for obtaining `InputStream` and `ByteArrayInput`.
