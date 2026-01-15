# ByteBuf, Memory Management

자바의 `ByteBuffer` vs Netty의 `ByteBuf` 차이를 알아보겠다.

자바 기본 ByteBuffer는 쓰기 불편하고 성능 제약이 많은데, (읽기/쓰기 모드를 flip() 해줘야 한다거나)

Netty의 ByteBuf는 이를 개선해 별도의 인덱스 reader index, writer index를 가진다.

reader index는 어디까지 읽었는지 writer index는 어디까지 썼는지를 기록해 놔 flip()없이 그냥 쓰고 읽을 수 있다.


### Memory Allocation (Direct Buffer vs Heap Buffer)

zero copy와 운영(gc)에 핵심이라고도 볼 수 있는데

Heap Buffer(Unpooled.buffer())은 jvm heap 메모리 안이 위치고 단점으로는 소켓이 데이터를 보낼때 os는 jvm heap있는 데이터를 직접 못가져간다.

heap 버퍼가 os가 접근 가능한 direct 영역으로 복사 -> 소켓 전송 과정을 거치는데 이 복사 비용이 낭비가 된다.

그러나 **Direct Buffer**는 jvm heap 바깥의 native memory 영역에 바로 쓸수있어 os가 메모리 주소로 바로 접근해 쏴버린다 소켓에 그래서 복사 과정이 없다.

대신에 메모리 할당/해제 비용이 매우 비싸다 os 시스템 콜이 필요하므로 그래서 netty는 pooling이란것을 한다.

### Pooling과 reference counting

netty는 비싼 direct buffer를 매번 만들지 않고 pool에 받아놓고 빌려준다. 여기선 memory leak이 있을 수 있다.

gc가 관리하지 않는 영역 direct memory 이기 때문에 개발자가 수동으로 카운트 관리를 해야한다.

- `refCnt`: 참조 카운트로 객체 생성시 1로 시작한다
- `retain()`: 카운트 + 1 
- `release()`: 카운트 -1
- 소멸: refCnt가 0이되면 다시 풀로 돌아간다.

```java
// 당신의 코드
try {
    // ... 로직 ...
} finally {
    m.release(); // [매우 잘함] 다 썼으니 반납. 이거 없으면 메모리 릭 발생.
}
```

이런식으로 마지막이 릴리즈를 해서 direct memory 영역 공간을 회수해야한다.

이걸 깜빡하면 direct memory oom이 날수있거나 관리를 잘못해서 릴리즈한걸 참조하려다가 IllegaReferenceCountException (이미 relase()해서 0이 된걸 또 쓰려고함)이 발생할 가능성도있다.

### Slice, CompositeByteBuf

Netty는 데이터를 합치거나 자를 때, 절대로 바이트 복사를 하지 않는다.

그냥 view만 새로 만드는데, Slice는 거대한 데이터에서 헤더만 읽고 싶을때 데이터를 복사해서 새 배열로 만드는게 아니라 포인터만 가진 얇은 객체를 만든다.

`ByteBuf header = msg.slice(0, 10);` 이런식을로 원분 msg와 메모리를 공유한다. 복사비용이 0이라는 뜻임. (0~10이 헤더라고 가정)

`CompositeByteBuf`는 합치기이ㅣㄴ데, header body를 합쳐서 보내야할때 ,새 배열을 만들고 둘 다 복사해 넣는 방식이 아니고 두 개의 버퍼를 논리적으로만 이어붙인다.

```java
CompositeByteBuf compBuf = Unpooled.compositeBuffer();
compBuf.addComponents(true, headerBuf, bodyBuf);
// 실제 메모리 복사는 일어나지 않음.
```

```java
package kr.honestfund.nice.proxy.server;

import io.netty.buffer.ByteBuf;
import io.netty.channel.ChannelHandlerContext;
import io.netty.handler.codec.ReplayingDecoder;
import lombok.extern.slf4j.Slf4j;
import java.nio.charset.Charset;
import java.util.List;

// 상태 정의: 헤더를 읽을 차례냐, 바디를 읽을 차례냐
enum NiceState {
    READ_HEADER,
    READ_BODY
}

@Slf4j
public class NiceMessageDecoder extends ReplayingDecoder<NiceState> {

    private final Charset charset = Charset.forName("EUC-KR");
    private int bodyLength = 0;

    public NiceMessageDecoder() {
        super(NiceState.READ_HEADER); // 초기 상태 설정
    }

    @Override
    protected void decode(ChannelHandlerContext ctx, ByteBuf in, List<Object> out) {
        switch (state()) {
            case READ_HEADER:
                // [핵심] in.readableBytes() 체크할 필요 없음.
                // ReplayingDecoder가 데이터 부족하면 알아서 여기서 멈추고,
                // 데이터 더 들어오면 다시 실행해줌 (마치 Blocking처럼 코딩 가능)
                
                // 10바이트 읽기 (String 객체 생성 최소화하려면 byte[]로 읽어서 파싱 추천)
                CharSequence headerStr = in.readCharSequence(10, charset);
                try {
                    // "0000000100" -> 100 (Header 10바이트 포함 여부는 프로토콜에 따라 조정)
                    // 작성하신 코드 기준: headerStr 자체가 body 길이라고 가정
                    bodyLength = Integer.parseInt(headerStr.toString());
                    
                    // [상태 전환] 이제 바디 읽으러 가자
                    checkpoint(NiceState.READ_BODY); 
                } catch (NumberFormatException e) {
                    log.error("Invalid Header: {}", headerStr);
                    ctx.close();
                    return;
                }
                break;

            case READ_BODY:
                // 바디 길이만큼 읽기
                // 역시 데이터가 bodyLength보다 적으면 ReplayingDecoder가 알아서 대기함
                String body = in.readCharSequence(bodyLength, charset).toString();
                
                // [완성] 전체 전문(Header + Body)을 뒤쪽 핸들러(NiceProxyHandler)로 넘김
                // 필요하다면 Header도 합쳐서 넘길 수 있음
                out.add(headerStr + body); 
                
                // [초기화] 다음 패킷을 위해 다시 헤더 상태로 복귀
                checkpoint(NiceState.READ_HEADER);
                break;
        }
    }
}
```