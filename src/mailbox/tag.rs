pub struct MailboxTag;

#[allow(dead_code, non_upper_case_globals)]
impl MailboxTag {
    /// Get firmware revision
    pub const GetVersion: u32 = 0x00000001;

    /* Hardware info commands */
    /// Get board model
    pub const GetBoardModel: u32 = 0x00010001;
    /// Get board revision
    pub const GetBoardRevision: u32 = 0x00010002;
    /// Get board MAC address
    pub const GetBoardMacAddress: u32 = 0x00010003;
    /// Get board serial
    pub const GetBoardSerial: u32 = 0x00010004;
    /// Get ARM memory
    pub const GetArmMemory: u32 = 0x00010005;
    /// Get VC memory
    pub const GetVcMemory: u32 = 0x00010006;
    /// Get clocks
    pub const GetClocks: u32 = 0x00010007;

    /* Power commands */
    /// Get power state
    pub const GetPowerState: u32 = 0x00020001;
    /// Get timing
    pub const GetTiming: u32 = 0x00020002;
    /// Set power state
    pub const SetPowerState: u32 = 0x00028001;

    /* GPIO commands */
    /// Get GPIO state
    pub const GetGetGpioState: u32 = 0x00030041;
    /// Set GPIO state
    pub const SetGpioState: u32 = 0x00038041;

    /* Clock commands */
    /// Get clock state
    pub const GetClockState: u32 = 0x00030001;
    /// Get clock rate
    pub const GetClockRate: u32 = 0x00030002;
    /// Get max clock rate
    pub const GetMaxClockRate: u32 = 0x00030004;
    /// Get min clock rate
    pub const GetMinClockRate: u32 = 0x00030007;
    /// Get turbo
    pub const GetTurbo: u32 = 0x00030009;

    /// Set clock state
    pub const SetClockState: u32 = 0x00038001;
    /// Set clock rate
    pub const SetClockRate: u32 = 0x00038002;
    /// Set turbo
    pub const SetTurbo: u32 = 0x00038009;

    /* Voltage commands */
    /// Get voltage
    pub const GetVoltage: u32 = 0x00030003;
    /// Get max voltage
    pub const GetMaxVoltage: u32 = 0x00030005;
    /// Get min voltage
    pub const GetMinVoltage: u32 = 0x00030008;

    /// Set voltage
    pub const SetVoltage: u32 = 0x00038003;

    /* Temperature commands */
    /// Get temperature
    pub const GetTemperature: u32 = 0x00030006;
    /// Get max temperature
    pub const GetMaxTemperature: u32 = 0x0003000A;

    /* Memory commands */
    /// Allocate Memory
    pub const AllocateMemory: u32 = 0x0003000C;
    /// Lock memory
    pub const LockMemory: u32 = 0x0003000D;
    /// Unlock memory
    pub const UnlockMemory: u32 = 0x0003000E;
    /// Release Memory
    pub const ReleaseMemory: u32 = 0x0003000F;

    /// Execute code
    pub const ExecuteCode: u32 = 0x00030010;

    /* QPU control commands */
    /// Execute code on QPU
    pub const ExecuteQpu: u32 = 0x00030011;
    /// QPU enable
    pub const EnableQpu: u32 = 0x00030012;

    /* Displaymax commands */
    /// Get displaymax handle
    pub const GetDispmanxHandle: u32 = 0x00030014;
    /// Get HDMI EDID block
    pub const GetEdidBlock: u32 = 0x00030020;

    /* SD Card commands */
    /// Get SD Card EMCC clock
    pub const MailboxGetSdhostClock: u32 = 0x00030042;
    /// Set SD Card EMCC clock
    pub const MailboxSetSdhostClock: u32 = 0x00038042;

    /* Framebuffer commands */
    /// Allocate Framebuffer address
    pub const AllocateFramebuffer: u32 = 0x00040001;
    /// Blank screen
    pub const BlankScreen: u32 = 0x00040002;
    /// Get physical screen width/height
    pub const GetPhysicalWidthHeight: u32 = 0x00040003;
    /// Get virtual screen width/height
    pub const GetVirtualWidthHeight: u32 = 0x00040004;
    /// Get screen colour depth
    pub const GetColourDepth: u32 = 0x00040005;
    /// Get screen pixel order
    pub const GetPixelOrder: u32 = 0x00040006;
    /// Get screen alpha mode
    pub const GetAlphaMode: u32 = 0x00040007;
    /// Get screen line to line pitch
    pub const GetPitch: u32 = 0x00040008;
    /// Get screen virtual offset
    pub const GetVirtualOffset: u32 = 0x00040009;
    /// Get screen overscan value
    pub const GetOverscan: u32 = 0x0004000A;
    /// Get screen palette
    pub const GetPalette: u32 = 0x0004000B;

    /// Release Framebuffer address
    pub const ReleaseFramebuffer: u32 = 0x00048001;
    /// Set physical screen width/heigh
    pub const SetPhysicalWidthHeight: u32 = 0x00048003;
    /// Set virtual screen width/height
    pub const SetVirtualWidthHeight: u32 = 0x00048004;
    /// Set screen colour depth
    pub const SetColourDepth: u32 = 0x00048005;
    /// Set screen pixel order
    pub const SetPixelOrder: u32 = 0x00048006;
    /// Set screen alpha mode
    pub const SetAlphaMode: u32 = 0x00048007;
    /// Set screen virtual offset
    pub const SetVirtualOffset: u32 = 0x00048009;
    /// Set screen overscan value
    pub const SetOverscan: u32 = 0x0004800A;
    /// Set screen palette
    pub const SetPalette: u32 = 0x0004800B;
    /// Set screen VSync
    pub const SetVsync: u32 = 0x0004800E;
    /// Set screen backlight
    pub const SetBacklight: u32 = 0x0004800F;

    /* VCHIQ commands */
    /// Enable VCHIQ
    pub const VchiqInit: u32 = 0x00048010;

    /* Config commands */
    /// Get command line
    pub const GetCommandLine: u32 = 0x00050001;

    /* Shared resource management commands */
    /// Get DMA channels
    pub const GetDmaChannels: u32 = 0x00060001;

    /* Cursor commands */
    /// Set cursor info
    pub const SetCursorInfo: u32 = 0x00008010;
    /// Set cursor state
    pub const SetCursorState: u32 = 0x00008011;
}