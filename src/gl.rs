#![allow(unused)]

use core::ops::Deref;

use zerocopy::AsBytes;

use crate::v3d::{self, buffer::Writer, MailboxMessage, V3DError};

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

#[inline(always)]
fn gpu_to_arm_addr(addr: Vc4Addr) -> u32 {
    addr & !0xC0000000
}

#[inline(always)]
fn arm_to_gpu_addr(addr: u32) -> Vc4Addr {
    addr | 0xC0000000
}

#[derive(Default)]
pub struct Scene {
    /* This is the current load position */
    loadpos: Vc4Addr, // Physical load address as ARM address

    /* These are all the same thing just handle and two different address GPU and ARM */
    render_handle: GpuHandle, // Renderer memory handle
    render_data: Vc4Addr,     // Renderer data VC4 locked address

    render_width: u16,  // Render width
    render_height: u16, // Render height

    shader_start: Vc4Addr,          // VC4 address shader code starts
    frag_shader_rec_start: Vc4Addr, // VC4 start address for fragment shader record

    bin_width: u32,  // Bin width
    bin_height: u32, // Bin height

    render_control: Vc4Addr,     // VC4 render control start address
    render_control_end: Vc4Addr, // VC4 render control end address

    vertex_vc4: Vc4Addr, // VC4 address to vertex data
    num_verts: u32,      // number of vertices

    index_vertex: Vc4Addr, // VC4 address to start of index vertex data
    index_vertex_cnt: u32, // Index vertex count
    max_index_vertex: u32, // Maximum Index vertex referenced

    /* TILE DATA MEMORY ... HAS TO BE 4K ALIGN */
    tile_handle: GpuHandle,    // Tile memory handle
    tile_mem_size: u32,        // Tiel memory size;
    tile_state_data: Vc4Addr,  // Tile data VC4 locked address
    tile_data_buffer: Vc4Addr, // Tile data buffer VC4 locked address

    /* BINNING DATA MEMORY ... HAS TO BE 4K ALIGN */
    binning_handle: GpuHandle, // Binning memory handle
    binning_data: Vc4Addr,     // Binning data VC4 locked address
    binning_cfg_end: Vc4Addr,  // VC4 binning config end address
}

impl Scene {
    pub fn init(render_width: u16, render_height: u16) -> Result<Scene, SceneError> {
        use v3d::flags::MemAllocFlags;

        let render_handle = mem_alloc(
            0x10000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;
        let render_data = mem_lock(render_handle)?;
        let loadpos = render_data;

        let bin_width = (render_width as u32 + 63) / 64;
        let bin_height = (render_height as u32 + 63) / 64;

        let tile_mem_size = 0x4000;
        let tile_handle = mem_alloc(
            tile_mem_size * 2,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;
        let tile_state_data = mem_lock(tile_handle)?;
        let tile_data_buffer = tile_handle + tile_mem_size;

        let binning_handle = mem_alloc(
            0x10000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;
        let binning_data = mem_lock(binning_handle)?;

        Ok(Scene {
            loadpos,
            render_handle,
            render_data,
            render_width,
            render_height,
            bin_width,
            bin_height,
            tile_handle,
            tile_mem_size,
            tile_state_data,
            tile_data_buffer,
            binning_handle,
            binning_data,
            ..Default::default() // shader_start: todo!(),
                                 // frag_shader_rec_start: todo!(),
                                 // render_control: todo!(),
                                 // render_control_end: todo!(),
                                 // vertex_vc4: todo!(),
                                 // num_verts: todo!(),
                                 // index_vertex: todo!(),
                                 // index_vertex_cnt: todo!(),
                                 // max_index_vertex: todo!(),
                                 // binning_cfg_end: todo!(),
        })
    }

    pub fn add_vertices(&mut self) -> Result<(), SceneError> {
        self.vertex_vc4 = (self.loadpos + 127) & 0xFFFFFF80; // align to 128bits
        let mut writer = Writer::new(self.vertex_vc4, 0x10000);
        // let ptr = gpu_to_arm_addr(self.vertex_vc4) as *mut u8;

        let center_x = self.render_width as u32 / 2;
        let center_y = ((self.render_height / 2) as f32 * 0.4) as u32;
        let half_shape_width = ((center_x as f32) * 0.4) as u32;
        let half_shape_height = ((self.render_height / 2) as f32 * 0.3) as u32;

        // Vertex: Top, red
        writer.write(&(center_x << 4 as u16).as_bytes());
        writer.write(&((center_y - half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());

        // Vertex: bottom left, blue
        writer.write(&((center_x - half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y + half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());

        // Vertex: bottom right, green
        writer.write(&((center_x + half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y + half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());

        let center_y = ((self.render_height / 2) as f32 * 1.35) as u32;

        // Vertex: Top left, blue
        writer.write(&((center_x - half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y - half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());

        // Vertex: bottom left, green
        writer.write(&((center_x - half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y + half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());

        // Vertex: top right, red
        writer.write(&((center_x + half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y - half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());

        // Vertex: bottom right, yellow
        writer.write(&((center_x + half_shape_width) << 4 as u16).as_bytes());
        writer.write(&((center_y - half_shape_height) << 4 as u16).as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&0.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());
        writer.write(&1.0f32.as_bytes());

        self.num_verts = 7;
        self.loadpos = self.vertex_vc4 + writer.bytes_written() as u32;
        drop(writer);

        let index_data = &[0, 1, 2, 3, 4, 5, 4, 6, 5];

        self.index_vertex = (self.loadpos + 127) & 0xFFFFFF80; // align to 128bits
        let mut writer = Writer::new(self.index_vertex, index_data.len());
        writer.write(index_data);

        self.index_vertex_cnt = index_data.len() as u32;
        self.max_index_vertex = 6;

        self.loadpos = self.index_vertex + writer.bytes_written() as u32;

        Ok(())
    }
}

#[derive(Debug)]
pub enum SceneError {
    AllocMemory,
    LockMemory,
    Trash,
}

fn mem_alloc(size: u32, align: u32, flags: u32) -> Result<GpuHandle, SceneError> {
    let handle = MailboxMessage::mem_alloc(size, align, flags)
        .send_and_read(3)
        .map_err(|_| SceneError::AllocMemory)?;

    Ok(handle)
}

fn mem_lock(handle: GpuHandle) -> Result<Vc4Addr, SceneError> {
    MailboxMessage::mem_lock(handle)
        .send_and_read(3)
        .map_err(|_| SceneError::LockMemory)
}

impl core::fmt::Display for SceneError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SceneError::AllocMemory => write!(f, "Failed to allocate GPU memory!"),
            SceneError::LockMemory => write!(f, "Lock Blocked (failed to lock GPU memory)"),
            SceneError::Trash => write!(f, "L + ratio idiot"),
        }
    }
}
