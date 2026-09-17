# Encryption Algorithm AES256

- 암호화: 평문을 암호화된 문구로. 
- 복호화: 암호화된 문구를 평문으로.
- 단방향 암호화: 평문을 암호화했을때 다시 복호화 할 수 없음 ex. SHA-256
- 양방향 암호화: 다시 복호화할 수 있음. ex. AES256
- 대칭키: 암호화하고 복호화하는데 사용되는 키가 동일한 것을 의미함
- 비대칭키: 암호화 복호화 각각 다른키가 사용됨

### AES(Advnaced Encryption Standard)

암호화, 복호화에 동일한 키를 사용하는 대칭키 알고리즘 대표적으로 양방향 알고리즘이다.

암호화된 데이터는 유효한 비밀 키 없이는 복호화할 수 없고, 무선 통신, 금융거래, 암호화된 데이터 저장과 같은 다양한 목적으로 전세계적으로 가장 일반적인 보안 알고리즘이며 데이터가 깨지지 않고 안정하게 전송이 가능함.

그러나 매우 간단한 대수 공식에 각 블록은 유사한 종류의 암호화를 사용하고 구현 어려움이 있음.

- AES-128: 10번의 암호화 처리
- AES-192: 12번의 암호화 처리
- AES-256: 14번의 암호화 처리

> 뒤의 붙은 숫자는 키의 길이를 의미함 128 16byte, 192 24byte, 256 32byte

- 암호화 원리 plain text -> pain bytes -> encrypt -> encrypted bytes -> encrypted base64 test
- 복호화 원리 encrypted base64 text -> encrypted bytes -> decrypt -> plain bytes -> plain text

구성요소는 plain text(평문), key라고 부르는 암호화 및 복호화를 수행하는데 사용되는 비밀 값이 존재하며 AES256에서는 256비트의 키를 사용함  
Initialization Vector(IV) 라는 초기화 벡터를 통해 암호화 과정에서 블록 암호화를 수행하는데 사용되는 비밀 값이다. 복호화할때도 사용된다.  
CipherText(암호문) 평문을 암호화한 결과를 cipher text라고 한다.

AES256 에서는 Advanced Encryption Standard(AES) 암호화 알고리즘을 사용하고 AES는 블록 암호화 알고리즘으로 128비트 블록 단위로 데이터를 처리한다.

Padding이라는 데이터 블록의 길이가 블록 크기의 배수가 아닐 경우, 마지막 블록에 패딩을 추가해 블록 크기의 배수로 맞추는 작업이 있다.

Operation Mode는 데이터 블록을 처리하는 방법으로 AES256 에서는 CTR(Counter) 운영 모드를 사용한다. CTR 운영 모드는 블록 암호화 함수를 스트림 암호화 함수처럼 사용해 데이터를 암호화한다.

<br>

### 블록 암호화

고정된 길의의 블록 단위로 데이터를 나누어 암호화하는 대칭키 암호화 기법이다.

브록 단위로 암호화를 사용하기 때문에 블록 암호화라고 부르며 대표적으로 DES, AES, Blowfish등이 있음.

블록 암호화는 평문을 블록 단위로 나누어 암호화를 수행하고, 이를 다시 이어붙여 전체 암호문을 생성한다.  
이때 각 블록은 독립적으로 암호화되며, 이전 블록의 암호문은 사용하지 않는다.  
이렇게 블록 단위로 암호화를 수행함으로써, 전체 데이터를 한번에 암호화하는 것 보다 더 안전하게 암호화가 가능하다.

가장 큰 장점은 역시 보안성이다. 블록 단위로 암호화를 하기 때문에 전체 데이터를 한 번에 암호화 하는 것보다 더욱 안전하게 암호화가 가능하며
데이터 일부만 수정되더라도 수정된 부분 뿐만 아니라 전체 데이터의 암호화가 새롭게 수행되며 무결성을 보호하는데 도움이 된다.

<br>

### Operation Mode

암호화 알고리즘에서 블록 암호를 활용할때 **평문을 작은 블록으로 분할**하고, 블록 단위로 암호화를 수행하는 방법이다.

이 방법으로 전체 평분을 한번에 암호화할필요 없이 블록으로 분할해 작은 블록 단위로 암호화해 성능향상과 보안 측면에서 이점이 있다.

대표적인 mode of operation으로는 ECB, CBC, CTR, OFB, CFB 등이 있다.

<br>

### AES 동작과정

**암호화**
1. 입력 평문을 128비트 블록으로 나누고 IV와 함께 256비트 키를 활용해 블록 암호화한다.
2. 이전 단계에서 생성된 암호문을 다음 블록을 암호화함
3. 모든 블록을 암호화한 후 마지막 블록의 암호문을 출력으로 반환한다.

**복호화**
1. 입력 암호문을 128비트 블록으로 나눈다.
2. IV와 함께 256비트 키를 사용해 블록을 복호화한다.
3. 이전 단계에서 생성된 복호문을 다음 블록의 복호화에 사용한다.
4. 모든 블록을 복호화하면, 마지막 블록의 복호문을 출력으로 반환한다.

만약 입력 평문이 128비트보다 작으면 padding을 사용해 블록 크기를 맞추어준다. (대표적으로 PKCS#7 PKCS#5등이 있음)

패딩은 입력 평문의 길이가 블록 크기의 배수가 되도록 빈 공간을 채워주는 방식이고

이 패딩 방식들은 블록 크기가 16바이트라면 추가해야하는 바이트를 구한 뒤 그 수만큼 패딩을 추가한다. 

ex 10byte 평문일시 6바이트 패딩 값을 추가하고 이 때 각 바이트에는 06이라는 값이 들어감.

반대로, 평문이 128보다 크다면 큰 평문을 여러 블록으로 나누어 처리한다. 

```java
public class AES256 {  

	public static String alg = "AES/CTR/NoPadding";  

	public static String encrypt(String text, String key, String iv) throws Exception {  
		byte[] key_byte = Base64.getDecoder().decode(key);  
		byte[] iv_byte = Base64.getDecoder().decode(iv);  
		Cipher cipher = Cipher.getInstance(alg);  
		SecretKeySpec keySpec = new SecretKeySpec(key_byte, "AES");  
		IvParameterSpec ivParamSpec = new IvParameterSpec(iv_byte);  
		cipher.init(Cipher.ENCRYPT_MODE, keySpec, ivParamSpec);
		byte[] encrypted = cipher.doFinal(text.getBytes("UTF-8"));  

		return Base64.getEncoder().encodeToString(encrypted);  
	  
	}  

	public static String decrypt(String cipherText, String key, String iv) throws Exception {  
		byte[] key_byte = Base64.getDecoder().decode(key);  
		byte[] iv_byte = Base64.getDecoder().decode(iv);  
		  
		Cipher cipher = Cipher.getInstance(alg);  
		SecretKeySpec keySpec = new SecretKeySpec(key_byte, "AES");  
		IvParameterSpec ivParamSpec = new IvParameterSpec(iv_byte);  
		cipher.init(Cipher.DECRYPT_MODE, keySpec, ivParamSpec);  
		  
		byte[] decodedBytes = Base64.getDecoder().decode(cipherText);  
		byte[] decrypted = cipher.doFinal(decodedBytes);  
		  
		return new String(decrypted, "UTF-8");  
	}  
}
```