pub struct MemAllocFlags;

// macro_rules! const_flags {
//     ($($n:ident = $v:expr),+) => {
//         $(pub const $n: u32 = $v;)+
//     };
// }

#[allow(dead_code, non_upper_case_globals)]
impl MemAllocFlags {
    /// Can be resized to 0 at any time. Use for cached data
    pub const Discardable: u32 = (1 << 0);
    /// Normal allocating alias. Don't use from ARM
    pub const Normal: u32 = (0 << 2);
    /// 0xC alias uncached
    pub const Direct: u32 = (1 << 2);
    /// 0x8 alias. Non-allocating in L2 but coherent
    pub const Coherent: u32 = (2 << 2);
    /// Allocating in L2
    pub const L1_NonAllocating: u32 = (Self::Direct | Self::Coherent);
    /// Initialize buffer to all zeroes
    pub const Zero: u32 = (1 << 4);
    /// Don't initialize (default is to initialize to all ones)
    pub const NoInit: u32 = (1 << 5);
    /// Likely to be locked for long periods of time
    pub const HintPermalock: u32 = (1 << 6);
}
