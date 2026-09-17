# Linux File System

### EXT File System

가장 대표적인 리눅스의 파일 시스템으로 리눅스의 탄생에 많은 영향을 끼쳤던 초기 유닉스 계열 운영체제중 하나인 MINIX에서 사용하던 파일 시스템에서 파일 이름과 크기의 한계를 보완해 설계되었다

- MINIX max filename size 14bytes
- MINIX max file syze 64MB
- EXT max filename size 255bytes
- EXT max file size 2GB

그러나 EXT는 data modification timestamp와 inode 수정을 지원하지 않으며 linked list를 통해 free block과 inode를 추적하기 때문에 성능이 저하된다는 문제점이 존재한다. 이러한 문제를 해결하기위해 등장한게 EXT2다.

> free block: 파일 시스템에서 사용되지 않은 저장공간이다. 이 곳에 새로운 파일이 저장될 수 있으며 효율적인 데이터 저장과 관리를 위해 필수적이다. 남은 저장공간을 추적해 최적의 성능을 유지하기 때문.
>
> inode: 파일 시스템에서 파일의 메타데이터(소유자, 권한, 크기등)을 저장하는 데이터구조로 각 파일에 대한 중요한 정보들을 관리해 파일 접근과 조작을 가능하게 하며 실제 데이터 블록 위치를 참조한다.

### EXT2 파일 시스템

EXT2는 이후 3 4랑도 비슷한 구조로 이어지기에 보다 자세히 살펴봐야한다.

디스크에서 파일 시스템은 파티션 하나당 하나씩 생성된다. 파티션에 ext2를 구축하면 파티션은 다수의 block group으로 나뉜다. 파일 시스템을 블록 그룹으로 분할하면 같은 파일에 대한 inode, data lock이 인접한 실런더에 위치하게 되어 seek time을 줄일 수 있다는 장점이 있다.

ext2는 bootstrap 코드가 존재하는 부트 블록과 여러개의 블록 그룹으로 구성된다. 그리고 블록 그룹은 다시 6가지 영역인 super block, block group descriptor, block bitmap, inode bitmap, inode table, data blocks)으로 구분된다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/0*pKWm_RvuEwbJ6rdr)

### Super Block

Super block은 **파일 시스템 구성**에 대한 모든 정보를 포함한다.

super block은 1024bytes offset에 저장되며 파일시스템을 마운트할때 필수적으로 사용된다. super block의 정보는 매우 중요해 block group descriptor와 함께 모든 block group에 복사본이 존재한다. 이 복사본이 너무 많은 저장공간을 차지하는 대규모 파일시스템의 경우 선택적으로 특정 그룹에만 backup을 보관하기도 한다. ext2의 super block 구조체는 다음과 같다.

ext2_super_block은 fs/ext2/ext2.h에 정의되어 있다.

```h
struct ext2_super_block {
__le32 s_inodes_count; /* Inodes count */
__le32 s_blocks_count; /* Blocks count */
__le32 s_r_blocks_count; /* Reserved blocks count */
__le32 s_free_blocks_count; /* Free blocks count */
__le32 s_free_inodes_count; /* Free inodes count */
__le32 s_first_data_block; /* First Data Block */
__le32 s_log_block_size; /* Block size */
__le32 s_log_frag_size; /* Fragment size */
__le32 s_blocks_per_group; /* # Blocks per group */
__le32 s_frags_per_group; /* # Fragments per group */
__le32 s_inodes_per_group; /* # Inodes per group */
__le32 s_mtime; /* Mount time */
__le32 s_wtime; /* Write time */
        ...
}
```

super block은 inode의 수, 전체 inode, block의 개수, free block, free inode의 수, block size, block group당 inode의 개수 file mount 시간등 정보를 담고있다.

### Inode

ext2_inode도 fs/ext2/ext2.h에 정의되어 있다.

```h
struct ext2_inode {
__le16 i_mode; /* File mode */
__le16 i_uid; /* Low 16 bits of Owner Uid */
__le32 i_size; /* Size in bytes */
__le32 i_atime; /* Access time */
__le32 i_ctime; /* Creation time */
__le32 i_mtime; /* Modification time */
__le32 i_dtime; /* Deletion Time */
__le16 i_gid; /* Low 16 bits of Group Id */
__le16 i_links_count; /* Links count */
__le32 i_blocks; /* Blocks count */
__le32 i_flags; /* File flags */
...
__le32 i_block[EXT2_N_BLOCKS]; /* Pointers to blocks */
...
};
```

- i_mode: inode가 관리하는 파일의 속성 및 접근제어 정보를 나타낸다
- i_uid, i_gid: 파일을 생성한 소유자의 user id, group id를 나타낸다
- i_atime, i_ctime, i_mtime, i_dtime: 각각의 파일 접근 시간, 생성 시간, 수정시간, 삭제시간을 의미한다.
- i_links_count: inode를 가리키고 있는 파일 수 또는 링크 수를 의미한다.
- i_blocks: 파일이 가지고 있는 block 개수를 의미하며 이때 block은 filesystem의 block을 의미하는 것이 아니라 512byte block(sector)을 의미한다.
- i_block[EXT2_N_BLOCKS]: 파일에 속한 block들의 위치를 관리하기 위해 사용되며 12개의 direct block과 3개의 indirect block(single indirect block, indirect block, triple indirect block)으로 구성되어있다.

아래는 i_mode field에 대한 설명이다.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/0*of7YW3mFqCTAqavf)

i_mode 필드는 16bit으로 구성되며 상위 4bit는 파일의 유형을 의미한다.

#### 파일 유형

![](https://miro.medium.com/v2/resize:fit:748/format:webp/1*sRfTNJ1x6B8VpA4e69MLaw.png)

다음 3bits는 특수 권한으로 사용된다
- u 비트는 setuid 비트로 특정 작업을 수행하기 위한 일시적 파일 소유자의 권한을 얻는데 사용된다.
- g 비트는 setgid 비트로 일시적 파일 그룹 권한을 얻게 된다.
- s 비트는 sticky bit로 directory file에 sticky bit를 지정하면 누구나 파일을 생성할 수 있지만 자신의 소유가 아닌 파일은 삭제할 수 없다.

마지막 9bits는 접근 제어 읽기/쓰기/실행을 위해 사용되며 3bits씩 user, group, other(다른 사용자)에 대한 접근 제어를 나타낸다.