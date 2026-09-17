# Linux File System

### EXT File System

The most representative Linux file system, EXT, was designed to address the limitations of file names and sizes in the file system used by MINIX, one of the early Unix-like operating systems that significantly influenced the birth of Linux.

- MINIX max filename size 14bytes
- MINIX max file syze 64MB
- EXT max filename size 255bytes
- EXT max file size 2GB

However, EXT does not support data modification timestamps or inode modification. It also suffers from performance degradation because it tracks free blocks and inodes using a linked list. EXT2 emerged to solve these problems.

> free block: Unused storage space in a file system. New files can be stored here, and it is essential for efficient data storage and management. It tracks remaining storage space to maintain optimal performance.
>
> inode: A data structure in a file system that stores file metadata (owner, permissions, size, etc.). It manages crucial information for each file, enabling file access and manipulation, and references the actual data block locations.

### EXT2 File System

EXT2's structure is similar to EXT3 and EXT4, so it's worth examining in more detail.

In a disk, one file system is created per partition. When ext2 is built on a partition, the partition is divided into multiple block groups. Dividing the file system into block groups has the advantage of reducing seek time by placing inodes and data blocks for the same file on adjacent cylinders.

ext2 consists of a boot block containing bootstrap code and several block groups. Each block group is further divided into six areas: super block, block group descriptor, block bitmap, inode bitmap, inode table, and data blocks.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/0*pKWm_RvuEwbJ6rdr)

### Super Block

The Super block contains all information about the **file system configuration**.

The super block is stored at a 1024-byte offset and is essential when mounting the file system. The information in the super block is so critical that a copy exists in every block group, along with the block group descriptor. For large file systems where these copies would consume too much storage, backups may optionally be kept only in specific groups. The ext2 super block structure is as follows:

ext2_super_block is defined in fs/ext2/ext2.h.

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

The super block contains information such as the number of inodes, total inodes, number of blocks, number of free blocks, number of free inodes, block size, number of inodes per block group, and file mount time.

### Inode

ext2_inode is also defined in fs/ext2/ext2.h.

```c
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

- i_mode: Indicates the file's attributes and access control information managed by the inode.
- i_uid, i_gid: Represent the user ID and group ID of the owner who created the file.
- i_atime, i_ctime, i_mtime, i_dtime: Respectively represent the file access time, creation time, modification time, and deletion time.
- i_links_count: Represents the number of files or links pointing to this inode.
- i_blocks: Represents the number of blocks held by the file. Here, a "block" refers to a 512-byte block (sector), not a file system block.
- i_block[EXT2_N_BLOCKS]: Used to manage the locations of blocks belonging to the file, consisting of 12 direct blocks and 3 indirect blocks (single indirect block, double indirect block, and triple indirect block).

Below is an explanation of the i_mode field.

![](https://miro.medium.com/v2/resize:fit:1400/format:webp/0*of7YW3mFqCTAqavf)

The i_mode field is 16 bits, with the upper 4 bits indicating the file type.

#### File Type

![](https://miro.medium.com/v2/resize:fit:748/format:webp/1*sRfTNJ1x6B8VpA4e69MLaw.png)

The next 3 bits are used for special permissions:
- The `u` bit is the setuid bit, used to temporarily gain the file owner's permissions to perform specific tasks.
- The `g` bit is the setgid bit, which temporarily grants file group permissions.
- The `s` bit is the sticky bit. If the sticky bit is set on a directory file, anyone can create files, but they cannot delete files they do not own.

The last 9 bits are used for access control (read/write/execute), with 3 bits each representing access control for the user, group, and others (other users).
