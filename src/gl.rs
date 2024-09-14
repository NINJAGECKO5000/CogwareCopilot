const VERTEX_SHADER: [u32; 18] = [
    0x958e0dbf, 0xd1724823, /* mov r0, vary; mov r3.8d, 1.0 */
    0x818e7176, 0x40024821, /* fadd r0, r0, r5; mov r1, vary */
    0x818e7376, 0x10024862, /* fadd r1, r1, r5; mov r2, vary */
    0x819e7540, 0x114248a3, /* fadd r2, r2, r5; mov r3.8a, r0 */
    0x809e7009, 0x115049e3, /* nop; mov r3.8b, r1 */
    0x809e7012, 0x116049e3, /* nop; mov r3.8c, r2 */
    0x159e76c0, 0x30020ba7, /* mov tlbc, r3; nop; thrend */
    0x009e7000, 0x100009e7, /* nop; nop; nop */
    0x009e7000, 0x500009e7, /* nop; nop; sbdone */
];

const FILL_SHADER: [u32; 12] = [
    0x009E7000, 0x100009E7, // nop; nop; nop
    0xFFFFFFFF, 0xE0020BA7, // ldi tlbc, RGBA White
    0x009E7000, 0x500009E7, // nop; nop; sbdone
    0x009E7000, 0x300009E7, // nop; nop; thrend
    0x009E7000, 0x100009E7, // nop; nop; nop
    0x009E7000, 0x100009E7, // nop; nop; nop
];

pub type Vc4Addr = u32;
pub type GpuHandle = u32;

pub struct Render {
    /* This is the current load position */
    loadpos: Vc4Addr, // Physical load address as ARM address

    /* These are all the same thing just handle and two different address GPU and ARM */
    renderer_handle: GpuHandle, // Renderer memory handle
    renderer_data_vc4: Vc4Addr, // Renderer data VC4 locked address

    render_wth: u16, // Render width
    render_ht: u16,  // Render height

    shader_start: Vc4Addr,          // VC4 address shader code starts
    frag_shader_rec_start: Vc4Addr, // VC4 start address for fragment shader record

    bin_wth: u32, // Bin width
    bin_ht: u32,  // Bin height

    render_control_vc4: Vc4Addr,     // VC4 render control start address
    render_control_end_vc4: Vc4Addr, // VC4 render control end address

    vertex_vc4: Vc4Addr, // VC4 address to vertex data
    num_verts: u32,      // number of vertices

    index_vertex_vc4: Vc4Addr, // VC4 address to start of index vertex data
    index_vertex_ct: u32,      // Index vertex count
    max_index_vertex: u32,     // Maximum Index vertex referenced

    /* TILE DATA MEMORY ... HAS TO BE 4K ALIGN */
    tile_handle: GpuHandle,        // Tile memory handle
    tile_mem_size: u32,            // Tiel memory size;
    tile_state_data_vc4: Vc4Addr,  // Tile data VC4 locked address
    tile_data_buffer_vc4: Vc4Addr, // Tile data buffer VC4 locked address

    /* BINNING DATA MEMORY ... HAS TO BE 4K ALIGN */
    binning_handle: GpuHandle, // Binning memory handle
    binning_data_vc4: Vc4Addr, // Binning data VC4 locked address
    binning_cfg_end: Vc4Addr,  // VC4 binning config end address
}
