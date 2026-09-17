# Inode (Index Node)

If a File Descriptor is a number tag that a running process uses to handle files, then an inode feels like the actual identity card of a file stored on disk.

### Inode

We recognize files by their filenames. However, the operating system kernel manages files by their inode numbers.

- **Abstraction**: In a file system, the filename is merely an alias for humans, and all of the file's actual metadata (size, permissions, creation time, etc.) and data block location information are stored in the inode.
- **Uniqueness**: Each inode within a file system has a unique number.

#### inode metadata

- **File Mode**: Permissions (read, write, execute) and file type (regular file, directory, socket, etc.)
- **Link Count**: The number of hard links pointing to this inode.
- **Owner Info**: Owner UID, GID (group UID)
- **File Size**: The size of the file (in bytes).
- **Timestamps**:
  - `atime`: Last access time
  - `mtime`: Last content modification time
  - `ctime`: Last attribute (metadata) change time
- **Data Block Pointers**: Address values indicating where the actual data is stored on disk.

Filenames are not stored in the inode. Filenames are mapped and stored within the data blocks of directory files in the format of **filename : inode number**.

### Inode Structure: Data Block Pointers

File sizes can vary from a few KB to several TB.

How can this be pointed to within a limited-size inode? We can look at the Multi-Level Indexing structure of the traditional Unix File System (UFS).

1.  **Direct Blocks**: The first approximately 12 pointers directly point to actual data blocks. Small files end here, making it very fast.
2.  **Single Indirect**: Instead of a data block, it points to an index block containing other pointers.
3.  **Double/Triple Indirect**: Points to an index of an index, supporting large files up to terabytes.

Modern file systems like Ext4 and XFS reportedly use a method called extents instead of this approach, recording and managing contiguous blocks from a start to an end point, thereby increasing efficiency.

### Hard Link, Soft (Symbolic) Link

**Hard links** are a mechanism that shares the existing inode number as is, and the inode number is identical to the original.

Data is retained if the original is deleted (until the link count reaches 0), and the file size is also the same as the original.

**Soft links (symbolic links)** create a new inode that contains the path name of the original.

It is different from the original, and the link is broken if the original is deleted. The file size is very small, only as large as the path string.

If the original is continuously referenced without knowing when it will be used, it leads to unnecessary inefficiency. So, it's kept as a soft link and then converted to a hard link for optimization when needed.

### Inode Exhaustion

File descriptor exhaustion can occur, but inode exhaustion can also occur.

This is the case where disk space (`df -h`) is sufficient, but attempting to create a new file results in a 'No space left on device' error.

-   When too many very small files, such as millions of session files or log files, are created, the total number of inodes is fixed at file system creation, so they can become exhausted.
-   You can check this with `df -i`.
-   File creation becomes impossible (e.g., `touch` or `mkdir` fails).
-   Service systems or database write operations may fail.

### FD, Inode

1.  The user calls `open("a.txt")`.
2.  The kernel finds the inode number corresponding to `a.txt` in the directory entry.
3.  It reads the corresponding inode and loads it into memory.
4.  The kernel creates an open file description pointed to by this inode and assigns an FD number to the process's FD table.

In conclusion, an FD is the entry point currently used by a process, and an inode is the actual room existing on disk.
