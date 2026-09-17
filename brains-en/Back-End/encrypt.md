# Encryption Algorithm AES256

- Encryption: Transforming plaintext into ciphertext.
- Decryption: Transforming ciphertext into plaintext.
- One-way encryption: Plaintext, once encrypted, cannot be decrypted back. e.g., SHA-256
- Two-way encryption: Can be decrypted back. e.g., AES256
- Symmetric key: Refers to using the same key for both encryption and decryption.
- Asymmetric key: Different keys are used for encryption and decryption, respectively.

### AES(Advnaced Encryption Standard)

It is a symmetric-key algorithm that uses the same key for encryption and decryption, and is a representative two-way algorithm.

Encrypted data cannot be decrypted without a valid secret key. It is the most common security algorithm worldwide for various purposes such as wireless communication, financial transactions, and encrypted data storage, enabling stable and secure data transmission.

However, each block uses a similar type of encryption with very simple algebraic formulas, and implementation can be challenging.

- AES-128: 10 rounds of encryption processing
- AES-192: 12 rounds of encryption processing
- AES-256: 14 rounds of encryption processing

> The number appended at the end indicates the key length: 128 (16 bytes), 192 (24 bytes), 256 (32 bytes).

- Encryption principle: plain text -> plain bytes -> encrypt -> encrypted bytes -> encrypted base64 text
- Decryption principle: encrypted base64 text -> encrypted bytes -> decrypt -> plain bytes -> plain text

The components include plaintext, a secret value called a key used for encryption and decryption, and AES256 uses a 256-bit key.
Initialization Vector (IV) is a secret value used to perform block encryption during the encryption process. It is also used during decryption.
Ciphertext: The result of encrypting plaintext is called ciphertext.

AES256 uses the Advanced Encryption Standard (AES) encryption algorithm. AES is a block cipher algorithm that processes data in 128-bit blocks.

Padding is the process of adding padding to the last block to make its length a multiple of the block size, if the data block length is not already a multiple of the block size.

Operation Mode is a method for processing data blocks. AES256 uses the CTR (Counter) operating mode. CTR operating mode encrypts data by using a block cipher function like a stream cipher function.

### Block Cipher

It is a symmetric-key encryption technique that divides data into fixed-length blocks and encrypts them.

It is called block encryption because it uses encryption in block units, with DES, AES, and Blowfish being representative examples.

Block ciphers divide plaintext into blocks, perform encryption, and then concatenate them to generate the complete ciphertext.
Each block is encrypted independently, and the ciphertext of the previous block is not used.
By performing encryption in block units this way, it is possible to encrypt data more securely than encrypting the entire data at once.

The biggest advantage is security. Because encryption is performed in block units, it is more secure than encrypting the entire data at once, and
even if only a part of the data is modified, not only the modified part but the entire data is re-encrypted, which helps protect integrity.

### Operation Mode

When utilizing block ciphers in an encryption algorithm, it is a method of **dividing plaintext into small blocks** and performing encryption block by block.

This method offers advantages in terms of performance improvement and security by dividing the entire plaintext into blocks and encrypting them in small block units, rather than encrypting the whole plaintext at once.

Representative modes of operation include ECB, CBC, CTR, OFB, and CFB.

### AES Operation Process

**Encryption**
1. Divide the input plaintext into 128-bit blocks and perform block encryption using a 256-bit key along with the IV.
2. Encrypt the next block using the ciphertext generated in the previous step.
3. After encrypting all blocks, return the ciphertext of the last block as output.

**Decryption**
1. Divide the input ciphertext into 128-bit blocks.
2. Decrypt the block using a 256-bit key along with the IV.
3. Use the decrypted text generated in the previous step for the decryption of the next block.
4. Once all blocks are decrypted, return the decrypted text of the last block as output.

If the input plaintext is smaller than 128 bits, padding is used to match the block size. (Representative examples include PKCS#7 and PKCS#5)

Padding is a method of filling empty spaces so that the length of the input plaintext becomes a multiple of the block size.

These padding schemes calculate the number of bytes to add if the block size is 16 bytes, and then add that many padding bytes.

For example, if the plaintext is 10 bytes, 6 bytes of padding are added, and each of these bytes will contain the value 06.

Conversely, if the plaintext is larger than 128 bits, the larger plaintext is divided into multiple blocks for processing.

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
