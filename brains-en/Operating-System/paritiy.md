# Parity bit, Pararell Paritiy, Hamming Code

### Paritiy bit

A parity bit is a bit added to **check for errors during information transmission.** It can detect a single-bit error. Parity bits are categorized into even or odd parity based on whether the count of '1's in the data (including the parity bit) is even or odd.

![](https://velog.velcdn.com/images/letskuku/post/17091b62-c821-41e2-bc6c-5d73065b4fbe/image.png)

As shown in the image above, when transmitting 8-bit data with a parity bit appended at the end, the data becomes 100101010 for even parity and 100101011 for odd parity. By defining parity this way, the receiver can verify errors by recalculating the parity bit from the total bits of the received data. However, a parity bit can only confirm that an error occurred; it cannot identify which bit is erroneous. Therefore, a retransmission request is necessary. Furthermore, if two or more bits are erroneous, it cannot detect the error.

### pararell paritiy

To compensate for the limitations mentioned above, there's also parallel parity, which applies parity to data blocks structured horizontally and vertically to locate and correct errors. As shown in the table below, parallel parity works by creating a parity bit for each horizontal 1-byte row and each vertical 1-byte column. When transmitted in block units, errors can be located and corrected by checking parity for both rows and columns.

If a single-bit error occurs within a transmitted data block, the parity will not match in specific horizontal and vertical parity sections, indicating an error at their intersection, as shown in the table below.

![](https://mblogthumb-phinf.pstatic.net/MjAxOTA3MTlfMjU4/MDAxNTYzNDk5NTM0MTc4.GSSbKf1ZlkAd9mC9W6_f9QnbN9Cwv9JW6Es4un7ZZnQg.Ct80Bg3NheAYrSDdsSFxf_woXlbeCoN9O2sz4BggORcg.PNG.cni1577/%EC%BA%A1%EC%B2%98.PNG?type=w800)

### Hamming Code

The parity issues mentioned above mean that errors of more than 1 bit cannot be detected, and 1-bit errors cannot be corrected. Hamming Code is a type of Error Correction Code (ECC) that can correct errors of more than 1 bit.

**Parity bits are added to the data as needed, depending on the number of data bits, and error detection and correction are performed by combining these parity bits.** The formula for calculating the number of parity bits required based on the number of data bits is **2^p >= d + p + 1 (p: number of parity bits, d: number of data bits)**.

For example, for 4-bit data transmission, at least 3 parity bits are required, making the Hamming code 7 bits long. In Hamming code, parity bits are inserted at positions corresponding to powers of 2, i.e., positions 1, 2, 4, 8 bits.

In summary, in Hamming code, the bits at positions 1, 2, 4... (which are powers of 2) are parity bits. (Parity bits are inserted at every 2^n position in the data transmission bits to find errors.) The determination is made based on whether the three parity bits starting from these numbers are even or odd.

![](https://img1.daumcdn.net/thumb/R1280x0/?scode=mtistory2&fname=https%3A%2F%2Fblog.kakaocdn.net%2Fdn%2Fbzx20l%2Fbtra345ZkEc%2FPRtCfFcqDtx6LvTCzRtng0%2Fimg.png)
