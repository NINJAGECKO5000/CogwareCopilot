pub struct V3DRegisters;

#[allow(dead_code, non_upper_case_globals)]
impl V3DRegisters {
    /// V3D Identification 0 (V3D block identity)
    pub const Ident0: u32 = 0x000;
    /// V3D Identification 1 (V3D Configuration A)
    pub const Ident1: u32 = 0x004;
    /// V3D Identification 1 (V3D Configuration B)
    pub const Ident2: u32 = 0x008;

    /// Scratch Register
    pub const Scratch: u32 = 0x010;

    /// 2 Cache Control
    pub const L2CacheCtrl: u32 = 0x020;
    /// Slices Cache Control
    pub const SliceCacheCtrl: u32 = 0x024;

    /// Interrupt Control
    pub const InterruptCtrl: u32 = 0x030;
    /// Interrupt Enables
    pub const InterruptEnable: u32 = 0x034;
    /// Interrupt Disables
    pub const InterruptDisable: u32 = 0x038;

    /// Control List Executor Thread 0 Control and Status.
    pub const ControlList0CS: u32 = 0x100;
    /// Control List Executor Thread 1 Control and Status.
    pub const ControlList1CS: u32 = 0x104;
    /// Control List Executor Thread 0 End Address.
    pub const ControlList0EA: u32 = 0x108;
    /// Control List Executor Thread 1 End Address.
    pub const ControlList1EA: u32 = 0x10c;
    /// Control List Executor Thread 0 Current Address.
    pub const ControlList0CA: u32 = 0x110;
    /// Control List Executor Thread 1 Current Address.
    pub const ControlList1CA: u32 = 0x114;
    /// Control List Executor Thread 0 Return Address.
    pub const ControlList00RA0: u32 = 0x118;
    /// Control List Executor Thread 1 Return Address.
    pub const ControlList01RA0: u32 = 0x11c;
    /// Control List Executor Thread 0 List Counter
    pub const ControlList0LC: u32 = 0x120;
    /// Control List Executor Thread 1 List Counter
    pub const ControlList1LC: u32 = 0x124;
    /// Control List Executor Thread 0 Primitive List Counter
    pub const ControlList0PC: u32 = 0x128;
    /// Control List Executor Thread 1 Primitive List Counter
    pub const ControlList1PC: u32 = 0x12c;

    /// V3D Pipeline Control and Status
    pub const PipelineCS: u32 = 0x130;
    /// Binning Mode Flush Count
    pub const BinningFlushCnt: u32 = 0x134;
    /// Rendering Mode Frame Count
    pub const RenderFrameCnt: u32 = 0x138;

    /// Current Address of Binning Memory Pool
    pub const BinningMemPool: u32 = 0x300;
    /// Remaining Size of Binning Memory Pool
    pub const FreeBinningMemPool: u32 = 0x304;
    /// Address of Overspill Binning Memory Block
    pub const BinningOverspill: u32 = 0x308;
    /// Size of Overspill Binning Memory Block
    pub const BinningOverspillSize: u32 = 0x30c;
    /// Binner Debug
    pub const BinnerDebug: u32 = 0x310;

    /// Reserve QPUs 0-7
    pub const ReserveQpuBank0: u32 = 0x410;
    /// Reserve QPUs 8-15
    pub const ReserveQpuBank1: u32 = 0x414;
    /// QPU Scheduler Control
    pub const QpuSchedCtrl: u32 = 0x418;

    // these are awful and should be probably broken out into their own enum to keep their names
    // from being a novel
    /// QPU User Program Request Program Address
    pub const QpuUserProgReqProgAddr: u32 = 0x430;
    /// QPU User Program Request Uniforms Address
    pub const QpuUserProgReqUniformsAddr: u32 = 0x434;
    /// QPU User Program Request Uniforms Length
    pub const QpuUserProgReqUniformsLen: u32 = 0x438;
    /// QPU User Program Request Control and Status
    pub const QpuUserProgReqCS: u32 = 0x43c;

    /// VPM Allocator Control
    pub const VpmAllocCtrl: u32 = 0x500;
    /// VPM base (user) memory reservation
    pub const VpmBase: u32 = 0x504;

    /// Performance Counter Clear
    pub const PerfCntrClr: u32 = 0x670;
    /// Performance Counter Enables
    pub const PerfCntrEnable: u32 = 0x674;

    /// Performance Counter Count 0
    pub const PerfCntrCnt0: u32 = 0x680;
    /// Performance Counter Mapping 0
    pub const PerfCntrMap0: u32 = 0x684;
    /// Performance Counter Count 1
    pub const PerfCntrCnt1: u32 = 0x688;
    /// Performance Counter Mapping 1
    pub const PerfCntrMap1: u32 = 0x68c;
    /// Performance Counter Count 2
    pub const PerfCntrCnt2: u32 = 0x690;
    /// Performance Counter Mapping 2
    pub const PerfCntrMap2: u32 = 0x694;
    /// Performance Counter Count 3
    pub const PerfCntrCnt3: u32 = 0x698;
    /// Performance Counter Mapping 3
    pub const PerfCntrMap3: u32 = 0x69c;
    /// Performance Counter Count 4
    pub const PerfCntrCnt4: u32 = 0x6a0;
    /// Performance Counter Mapping 4
    pub const PerfCntrMap4: u32 = 0x6a4;
    /// Performance Counter Count 5
    pub const PerfCntrCnt5: u32 = 0x6a8;
    /// Performance Counter Mapping 5
    pub const PerfCntrMap5: u32 = 0x6ac;
    /// Performance Counter Count 6
    pub const PerfCntrCnt6: u32 = 0x6b0;
    /// Performance Counter Mapping 6
    pub const PerfCntrMap6: u32 = 0x6b4;
    /// Performance Counter Count 7
    pub const PerfCntrCnt7: u32 = 0x6b8;
    /// Performance Counter Mapping 7
    pub const PerfCntrMap7: u32 = 0x6bc;
    /// Performance Counter Count 8
    pub const PerfCntrCnt8: u32 = 0x6c0;
    /// Performance Counter Mapping 8
    pub const PerfCntrMap8: u32 = 0x6c4;
    /// Performance Counter Count 9
    pub const PerfCntrCnt9: u32 = 0x6c8;
    /// Performance Counter Mapping 9
    pub const PerfCntrMap9: u32 = 0x6cc;
    /// Performance Counter Count 10
    pub const PerfCntrCnt10: u32 = 0x6d0;
    /// Performance Counter Mapping 10
    pub const PerfCntrMap10: u32 = 0x6d4;
    /// Performance Counter Count 11
    pub const PerfCntrCnt11: u32 = 0x6d8;
    /// Performance Counter Mapping 11
    pub const PerfCntrMap11: u32 = 0x6dc;
    /// Performance Counter Count 12
    pub const PerfCntrCnt12: u32 = 0x6e0;
    /// Performance Counter Mapping 12
    pub const PerfCntrMap12: u32 = 0x6e4;
    /// Performance Counter Count 13
    pub const PerfCntrCnt13: u32 = 0x6e8;
    /// Performance Counter Mapping 13
    pub const PerfCntrMap13: u32 = 0x6ec;
    /// Performance Counter Count 14
    pub const PerfCntrCnt14: u32 = 0x6f0;
    /// Performance Counter Mapping 14
    pub const PerfCntrMap14: u32 = 0x6f4;
    /// Performance Counter Count 15
    pub const PerfCntrCnt15: u32 = 0x6f8;
    /// Performance Counter Mapping 15
    pub const PerfCntrMap15: u32 = 0x6fc;

    /// PSE Error Signals
    pub const PseErrors: u32 = 0xf00;
    /// FEP Overrun Error Signals
    pub const FepOverrunErrors: u32 = 0xf04;
    /// FEP Interface Ready and Stall Signals; FEP Busy Signals
    pub const FepInterfaceStatus: u32 = 0xf08;
    /// FEP Internal Ready Signals
    pub const FepInternalReadySignals: u32 = 0xf0c;
    /// FEP Internal Stall Input Signals
    pub const FepInternalStallSignals: u32 = 0xf10;

    /// Miscellaneous Error Signals: u32 = VPM; VDW, VCD, VCM, L2C)
    pub const MiscErrors: u32 = 0xf20;
}
