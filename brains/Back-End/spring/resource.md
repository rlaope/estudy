# Spring Resource Interface, Resource 추상화와 구현체

## Resource
java.net.URL의 한계 (classpath 내부 접근이나 상대경로 등)를 넘어서기 위해 스프링에서 추가로 구현

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

## Resource 구현체

### UrlResource
URL을 기준으로 리소스를 읽어들임.
  
지원하는 프로토콜은 http, https, ftp, file, jar

### ClassPathResource
지원하는 접두어가 classpath: 일 때, 클래스패스를 기준으로 리소스를 읽어들인다.

### FileSystemResource
파일 시스템을 기준으로 읽는다.

### ServeltContextResource
웹 애플리케이션 루트에서 상대 경로로 리소스를 찾는다.

### InputStreamResource, ByteArrayResource
말그대로 InputStream, ByteArrayInput을 가져오기 위한 구현체

