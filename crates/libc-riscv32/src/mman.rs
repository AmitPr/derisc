pub const MAP_FAILED: i32 = -1;

pub const MAP_SHARED: u32 = 0x01;
pub const MAP_PRIVATE: u32 = 0x02;
pub const MAP_SHARED_VALIDATE: u32 = 0x03;
pub const MAP_TYPE: u32 = 0x0f;
pub const MAP_FIXED: u32 = 0x10;
pub const MAP_ANON: u32 = 0x20;
pub const MAP_ANONYMOUS: u32 = MAP_ANON;
pub const MAP_NORESERVE: u32 = 0x4000;
pub const MAP_GROWSDOWN: u32 = 0x0100;
pub const MAP_DENYWRITE: u32 = 0x0800;
pub const MAP_EXECUTABLE: u32 = 0x1000;
pub const MAP_LOCKED: u32 = 0x2000;
pub const MAP_POPULATE: u32 = 0x8000;
pub const MAP_NONBLOCK: u32 = 0x10000;
pub const MAP_STACK: u32 = 0x20000;
pub const MAP_HUGETLB: u32 = 0x40000;
pub const MAP_SYNC: u32 = 0x80000;
pub const MAP_FIXED_NOREPLACE: u32 = 0x100000;
pub const MAP_FILE: u32 = 0;

pub const PROT_NONE: u32 = 0;
pub const PROT_READ: u32 = 1;
pub const PROT_WRITE: u32 = 2;
pub const PROT_EXEC: u32 = 4;
