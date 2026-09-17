# SocketAdress 클래스 & NetworkInterface 클래스

### SocketAdress 클래스 & NetworkInterface 클래스
SocketAddress 클래스는 소켓에서 사용하는 IP주소와 포트번호를 관리하는 추상 클래스이다.

- SocketAddress 클래스는 추상 클래스이기 때문에 직접 객체를 생성하지 않으며, 보통 하위 InetSockentAdress 클래스를 사용하여 아래와 같이 SockentAddress 클래스의 인스턴스를 생성한다.
  
SocketAddress socketAddress = ew InetSocketAddress(host,port);

### SocketAddress 클래스 특징
- SocketAddress 클래스의 인스턴스는 한번 생성하게 되면 변경이 불가능하다.
- InetSocketAddress(int port)와 같이 포트번호만 주면, 호스트의 IP주소는 임의의 IP주소가 된다. 패킷이 전송할 때 커널에서 알아서 디폴트 디바이스 주소를 부여한다.
- 생성한 SocketAddress 객체를 사용하여 외부 호스트와 연결을 시도할 때 예외가 발생하면, 안드로이드 시스템은 해당 객체에 unresolved로 표기하고 재 사용할 수 없게 만든다.


### NetworkInterface 클래스 특징
네트워크 디바이스의 정보를 얻을 수 있다.
- 시스템 내 존재하는 모든 네트워크 디바이스의 리스트 획득
- 특정 네트워크 디바이스 찾기
- 네트워크 디바이스 내 설정된 IP주소로부터 InetAddress 객체 얻기
- 네트워크 디바이스 내 설정된 IP주소로부터 InterfaceAddress 객체로 구성된 리스트를 얻는다.

<br>

### NetworkInterface 클래스를 이용해 디바이스 정보를 호출하는 예제

```java
import java.net.InterfaceAddress;
import java.net.NetworkInterface;
import java.net.SocketException;
import java.util.Enumeration;
import java.util.Iterator;
import java.util.List;

public class NetworkParameter {
    public static void main(String args[]) throws Exception{
        Enumeration<NetworkInterface> en = NetworkInterface.getNetworkInterfaces();

        while(en.hasMoreElements()){
            NetworkInterface ni = en.nextElement();
            printParameter(ni);
        }
    }

    public static void printParameter(NetworkInterface ni) throws SocketException{
        System.out.println("Name = " +ni.getName());
        System.out.println("Display Name = " + ni.getDisplayName());
        System.out.println("Is up = " + ni.isUp());
        System.out.println("Support multicast =" + ni.supportsMulticast());
        System.out.println("Is loopback" + ni.isLoopback());
        System.out.println("Is virtual = " + ni.isVirtual());
        System.out.println("is point to point = " + ni.isPointToPoint());
        System.out.println("Hardware address = " + ni.getHardwareAddress());
        System.out.println("MTU" + ni.getMTU());
        System.out.println("\nList of Interface Address:");
        
        List<InterfaceAddress> list = ni.getInterfaceAddresses();
        Iterator<InterfaceAddress> it = list.iterator();

        while(it.hasNext()){
            InterfaceAddress ia = it.next();
            System.out.println("Address = " + ia.getAddress());
            System.out.println("Broadcast = " + ia.getBroadcast());
            System.out.println("Network prefix length = " + ia.getNetworkPrefixLength());
            System.out.println("");
        }
    }
}
```

```
Name = utun0

        Display Name = utun0
        Is up = true
        Support multicast =true
        Is loopbackfalse
        Is virtual = false
        is point to point = true
        Hardware address = null
        MTU2000

        List of Interface Address:
        Address = /fe80:0:0:0:2bd7:7f96:b318:7dd1%utun0
        Broadcast = null
        Network prefix length = 64

        Name = awdl0
        Display Name = awdl0
        Is up = true
        Support multicast =true
        Is loopbackfalse
        Is virtual = false
        is point to point = false
        Hardware address = [B@64616ca2
        MTU1484

        List of Interface Address:
        Address = /fe80:0:0:0:5495:d7ff:fe6f:7913%awdl0
        Broadcast = null
        Network prefix length = 64

        Name = en1
        Display Name = en1
        Is up = true
        Support multicast =true
        Is loopbackfalse
        Is virtual = false
        is point to point = false
        Hardware address = [B@13fee20c
        MTU1500

        List of Interface Address:
        Address = /fe80:0:0:0:1026:3310:cf18:a758%en1
        Broadcast = null
        Network prefix length = 64

        Address = /192.168.0.117
        Broadcast = /192.168.0.255
        Network prefix length = 24

        Name = lo0
        Display Name = lo0
        Is up = true
        Support multicast =true
        Is loopbacktrue
        Is virtual = false
        is point to point = false
        Hardware address = null
        MTU16384

        List of Interface Address:
        Address = /fe80:0:0:0:0:0:0:1%lo0
        Broadcast = null
        Network prefix length = 64

        Address = /0:0:0:0:0:0:0:1%lo0
        Broadcast = null
        Network prefix length = 128

        Address = /127.0.0.1
        Broadcast = null
        Network prefix length = 8


        Process finished with exit code 0
```