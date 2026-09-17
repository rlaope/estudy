# Block IO (Buffer Head, BIO Struct, Scheduler)


### Buffer Head

Block devices refer to hardware devices like flash memory that allow random access to fixed-size data. The minimum unit that a block device physically accesses is a sector (512 bytes), and the logical access unit is a block, which is a multiple of the sector size.

For a block on disk to appear in memory, a buffer acting as an object is required. The kernel stores and represents control information about which block device and which block a buffer corresponds to using the `buffer_head` structure, defined in `<linux/buffer_head.h>`.

```c
// https://github.com/torvalds/linux/blob/f2e8a57ee9036c7d5443382b6c3c09b51a92ec7e/include/linux/buffer_head.h#L59
/*
* Historically, a buffer_head was used to map a single block
* within a page, and of course as the unit of I/O through the
* filesystem and block layers.  Nowadays the basic I/O unit
* is the bio, and buffer_heads are used for extracting block
* mappings (via a get_block_t call), for tracking state within
* a page (via a page_mapping) and for wrapping bio submission
* for backward compatibility reasons (e.g. submit_bh).
*/
struct buffer_head {
  unsigned long b_state;		/* Represents the current state of the buffer and holds one of several flag values. (see above) */
  struct buffer_head *b_this_page;/* circular list of page's buffers */
  union {
    struct page *b_page;	/* the page this bh is mapped to */
    struct folio *b_folio;	/* the folio this bh is mapped to */
  };

  sector_t b_blocknr;		/* start block number */
  size_t b_size;			/* Length (size) of the block */
  char *b_data;			/* The block pointed to by the buffer */

  struct block_device *b_bdev;
  bh_end_io_t *b_end_io;		/* I/O completion */
  void *b_private;		/* reserved for b_end_io */
  struct list_head b_assoc_buffers; /* associated with another mapping */
  struct address_space *b_assoc_map;	/* mapping this buffer is
              associated with */
  atomic_t b_count;		/* Represents the usage count of the buffer. It is incremented/decremented by get_bh() and put_bh() functions. */
  spinlock_t b_uptodate_lock;	/* Used by the first bh in a page, to
          * serialise IO completion of other
          * buffers in the page */
};
```

Before kernel version 2.6, the buffer head was much larger and played a more significant role, responsible for all block I/O operations. However, performing block I/O with buffer heads was a large and difficult task, and performing I/O from a page perspective proved simpler and offered better performance. Furthermore, using a large buffer head data structure to describe buffers smaller than a page was inefficient.

<br>

### bio Structure

As the functionality of the massive buffer head was simplified, the `bio` structure in `<linux/bio.h>` took over the block I/O operations.

```c
// https://litux.nl/mirror/kerneldevelopment/0672327201/ch13lev1sec3.html
struct bio {
        sector_t             bi_sector;         /* associated sector on disk */
        struct bio           *bi_next;          /* list of requests */
        struct block_device  *bi_bdev;          /* associated block device */
        unsigned long        bi_flags;          /* status and command flags */
        unsigned long        bi_rw;             /* read or write? */
        unsigned short       bi_vcnt;           /* number of bio_vecs off */
        unsigned short       bi_idx;            /* current index in bi_io_vec */
        unsigned short       bi_phys_segments;  /* number of segments after coalescing */
        unsigned short       bi_hw_segments;    /* number of segments after remapping */
        unsigned int         bi_size;           /* I/O count */
        unsigned int         bi_hw_front_size;  /* size of the first mergeable segment */
        unsigned int         bi_hw_back_size;   /* size of the last mergeable segment */
        unsigned int         bi_max_vecs;       /* maximum bio_vecs possible */
        struct bio_vec       *bi_io_vec;        /* bio_vec list */
        bio_end_io_t         *bi_end_io;        /* I/O completion method */
        atomic_t             bi_cnt;            /* usage counter */
        void                 *bi_private;       /* owner-private method */
        bio_destructor_t     *bi_destructor;    /* destructor method */
};
```

The `bio` structure uses a scatter-gather I/O method, allowing individual buffers to be represented as a linked list of segments called `bio_io_vec`, even if they are not contiguous in memory.

<br>

### Request Queue

Block devices store pending block I/O requests in a request queue. The request queue is represented using the `request_queue` structure in `<linux/blkdev.h>`.

```c
// https://github.com/torvalds/linux/blob/f2e8a57ee9036c7d5443382b6c3c09b51a92ec7e/include/linux/blkdev.h#L378
struct request_queue {
	struct request		*last_merge;
	struct elevator_queue	*elevator;

	struct percpu_ref	q_usage_counter;

	struct blk_queue_stats	*stats;
	struct rq_qos		*rq_qos;
	struct mutex		rq_qos_mutex;

	const struct blk_mq_ops	*mq_ops;

	/* sw queues */
	struct blk_mq_ctx __percpu	*queue_ctx;

	unsigned int		queue_depth;

	/* hw dispatch queues */
	struct xarray		hctx_table;
	unsigned int		nr_hw_queues;

	/*
	 * The queue owner gets to use this for whatever they like.
	 * ll_rw_blk doesn't touch it.
	 */
	void			*queuedata;

	/*
	 * various queue flags, see QUEUE_* below
	 */
	unsigned long		queue_flags;
	/*
	 * Number of contexts that have called blk_set_pm_only(). If this
	 * counter is above zero then only RQF_PM requests are processed.
	 */
	atomic_t		pm_only;

	/*
	 * ida allocated id for this queue.  Used to index queues from
	 * ioctx.
	 */
	int			id;

	spinlock_t		queue_lock;

	struct gendisk		*disk;

	refcount_t		refs;

	/*
	 * mq queue kobject
	 */
	struct kobject *mq_kobj;

#ifdef  CONFIG_BLK_DEV_INTEGRITY
	struct blk_integrity integrity;
#endif	/* CONFIG_BLK_DEV_INTEGRITY */

#ifdef CONFIG_PM
	struct device		*dev;
	enum rpm_status		rpm_status;
#endif

	/*
	 * queue settings
	 */
	unsigned long		nr_requests;	/* Max # of requests */

	unsigned int		dma_pad_mask;

#ifdef CONFIG_BLK_INLINE_ENCRYPTION
	struct blk_crypto_profile *crypto_profile;
	struct kobject *crypto_kobject;
#endif

	unsigned int		rq_timeout;

	struct timer_list	timeout;
	struct work_struct	timeout_work;

	atomic_t		nr_active_requests_shared_tags;

	struct blk_mq_tags	*sched_shared_tags;

	struct list_head	icq_list;
#ifdef CONFIG_BLK_CGROUP
	DECLARE_BITMAP		(blkcg_pols, BLKCG_MAX_POLS);
	struct blkcg_gq		*root_blkg;
	struct list_head	blkg_list;
	struct mutex		blkcg_mutex;
#endif

	struct queue_limits	limits;

	unsigned int		required_elevator_features;

	int			node;
#ifdef CONFIG_BLK_DEV_IO_TRACE
	struct blk_trace __rcu	*blk_trace;
#endif
	/*
	 * for flush operations
	 */
	struct blk_flush_queue	*fq;
	struct list_head	flush_list;

	struct list_head	requeue_list;
	spinlock_t		requeue_lock;
	struct delayed_work	requeue_work;

	struct mutex		sysfs_lock;
	struct mutex		sysfs_dir_lock;

	/*
	 * for reusing dead hctx instance in case of updating
	 * nr_hw_queues
	 */
	struct list_head	unused_hctx_list;
	spinlock_t		unused_hctx_lock;

	int			mq_freeze_depth;

#ifdef CONFIG_BLK_DEV_THROTTLING
	/* Throttle data */
	struct throtl_data *td;
#endif
	struct rcu_head		rcu_head;
	wait_queue_head_t	mq_freeze_wq;
	/*
	 * Protect concurrent access to q_usage_counter by
	 * percpu_ref_kill() and percpu_ref_reinit().
	 */
	struct mutex		mq_freeze_lock;

	int			quiesce_depth;

	struct blk_mq_tag_set	*tag_set;
	struct list_head	tag_set_list;

	struct dentry		*debugfs_dir;
	struct dentry		*sched_debugfs_dir;
	struct dentry		*rqos_debugfs_dir;
	/*
	 * Serializes all debugfs metadata operations using the above dentries.
	 */
	struct mutex		debugfs_mutex;

	bool			mq_sysfs_init_done;
};
```

The `request_queue` structure contains multiple `request` structures, which represent block I/O requests. Each `request` contains one or more `BIO` structures as members, and each `BIO` structure points to a `bio_vec` array, which can contain multiple segments.

<br>

### I/O Scheduler

The kernel does not immediately send block I/O requests to the queue upon receiving them. Instead, it uses an I/O scheduler to merge and sort pending block I/O requests that can be combined, thereby minimizing disk seek time and significantly improving system performance.

- If the sectors that request A and request B intend to access are adjacent, it is efficient to combine them into a single I/O request. This allows multiple requests to be processed with a single command.
- When there are no requests that can be merged, it is more efficient to sort and add a request near other requests that access physically close sectors, rather than simply appending it to the end of the request queue.

> I/O scheduler algorithms are quite similar to the **elevator algorithm** in our daily lives, so much so that they were actually called the Linux Elevator until kernel version 2.4.

- **Deadline**
    - Most users are more sensitive to read performance than write performance. If a single read request remains unprocessed, the overall system latency will increase tremendously.
    - As the name suggests, it sets an expiration time for each request. Read requests have a 0.5-second deadline, and write requests have a 5-second deadline.
    - The Deadline scheduler uses FIFO queues for requests. When a request at the head of either the write FIFO or read FIFO queue expires, that request is processed with the highest priority.

- **Anticipatory**
    - While the Deadline scheduler ensures excellent read performance, this comes at the cost of overall performance degradation.
    - The Anticipatory scheduler operates heuristically based on the Deadline scheduler.
    - When a read request occurs, the scheduler typically processes it and then immediately moves to another request. However, with the Anticipatory scheduler, it does nothing for several milliseconds, during which other read requests from the user continue to arrive, get merged, and sorted.
    - After the waiting period, the scheduler resumes processing the previous requests.
    - This strategy of doing nothing for several milliseconds, anticipating that more requests will arrive to process many read requests at once, surprisingly significantly increases performance.
    - In particular, the scheduler uses various I/O pattern statistics and heuristics to predict the behavior of application file systems, processing read requests and significantly reducing wasted seek time.
    - This scheduler is ideal for servers.

- **Completely Fair Queueing**
    - It is currently the default I/O scheduler in Linux, showing the best and highest performance under various load conditions.
    - It stores, merges, and sorts I/O requests in queues allocated per process.
    - Between each request queue, a pre-configured number of requests (default 4) are dequeued and processed in a round-robin fashion.

- **Noop**
    - This scheduler is for block devices that allow perfectly random access, such as flash memory.
    - It only merges requests and performs no sorting or other operations to save seek time.
