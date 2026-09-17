# SocketAddress Class & NetworkInterface Class

### SocketAddress Class & NetworkInterface Class
The SocketAddress class is an abstract class that manages IP addresses and port numbers used by sockets.

- Since the SocketAddress class is an abstract class, you cannot directly create an object from it. Instead, you typically create an instance of the SocketAddress class using its subclass, InetSocketAddress, as shown below:
  
SocketAddress socketAddress = ew InetSocketAddress(host,port);

### SocketAddress Class Features
- Once an instance of the SocketAddress class is created, it cannot be changed.
- If only a port number is provided, such as with `InetSocketAddress(int port)`, the host's IP address becomes an arbitrary IP address. When packets are transmitted, the kernel automatically assigns the default device address.
- If an exception occurs when attempting to connect to an external host using a created SocketAddress object, the Android system marks that object as 'unresolved' and prevents its reuse.

### NetworkInterface Class Features
Information about network devices can be obtained.
- Obtain a list of all network devices existing within the system.
- Find a specific network device.
- Obtain an InetAddress object from the IP address configured within the network device.
- Obtain a list of InterfaceAddress objects from the IP addresses configured within the network device.

<br>

### Example of calling device information using the NetworkInterface class

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
