#![allow(unused)]

use core::ops::Deref;

use alloc::{format, string::String};
use zerocopy::AsBytes;

use crate::{info, v3d::{self, buffer::Writer, flags::MemAllocFlags, get_v3d_ptr, MailboxMessage, V3DError}};

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

const ALIGN_128_BITS: u32 = 0xFFFFFF80;

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
    // loadpos: Vc4Addr, // Physical load address as ARM address

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
    tile_state_handle: GpuHandle,
    tile_state_data: Vc4Addr,  // Tile data VC4 locked address
    tile_state_size: u32,
    tile_data_buffer: Vc4Addr, // Tile data buffer VC4 locked address

    /* BINNING DATA MEMORY ... HAS TO BE 4K ALIGN */
    binning_handle: GpuHandle, // Binning memory handle
    binning_data: Vc4Addr,     // Binning data VC4 locked address
    binning_cfg_end: Vc4Addr,  // VC4 binning config end address
}

// #[repr(C, packed)]
// #[derive(Debug, AsBytes)]
// pub struct TileRenderModeConfig {
//     cmd: u8,
//     fb_ptr: u32,
//     render_width: u16,
//     render_height: u16,
//     something1: u8,
//     something2: u8,
// }
//
// // const GL_CLEAR_COLORS: u8 = 114;
// impl TileRenderModeConfig {
//     pub fn new(
//         fb_ptr: u32,
//         render_width: u16,
//         render_height: u16,
//         something1: u8,
//         something2: u8,
//     ) -> Self {
//         Self {
//             cmd: 113, // GL_TILE_RENDER_CONFIG
//             fb_ptr,
//             render_width,
//             render_height,
//             something1,
//             something2,
//         }
//     }
// }

#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct TileRenderModeConfig {
    cmd: u8,
    render_width: u16,
    render_height: u16,
    format: u32,
    other: u8,
    fb_ptr: u32,
}

// const GL_CLEAR_COLORS: u8 = 114;
impl TileRenderModeConfig {
    pub fn new(
        render_width: u16,
        render_height: u16,
        format: u32,
        other: u8,
        fb_ptr: u32,
    ) -> Self {
        Self {
            cmd: 113, // GL_TILE_RENDER_CONFIG
            render_width,
            render_height,
            format,
            other,
            fb_ptr,
        }
    }
}
#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct TileCoords {
    cmd: u8,
    arg1: u8,
    arg2: u8,
}

impl TileCoords {
    pub fn new(arg1: u8, arg2: u8) -> Self {
        Self {
            cmd: 115,
            arg1,
            arg2,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct TileBuffer {
    cmd: u8,
    arg1: u16, // wtf
    arg2: u32, // is this?
}

impl TileBuffer {
    pub fn new(arg1: u16, arg2: u32) -> Self {
        Self {
            cmd: 28u8,
            arg1,
            arg2,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct TileBinningConfig {
    cmd: u8,
    bin_width: u32,
    bin_height: u32,
    buffer_ptr: u32,
    mem_size: u32,
    state_data_ptr: u32,
    some_tag_thing: u8,
}

impl TileBinningConfig {
    pub fn new(
        bin_width: u32,
        bin_height: u32,
        buffer_ptr: u32,
        mem_size: u32,
        state_data_ptr: u32,
        some_tag_thing: u8,
    ) -> Self {
        Self {
            cmd: 112,
            bin_width,
            bin_height,
            buffer_ptr,
            mem_size,
            state_data_ptr,
            some_tag_thing,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct ClipWindowConfig {
    cmd: u8,
    arg0: u16,
    arg1: u16,
    width: u16,
    height: u16,
}

impl ClipWindowConfig {
    pub fn new(arg0: u16, arg1: u16, width: u16, height: u16) -> Self {
        Self {
            cmd: 102,
            arg0,
            arg1,
            width,
            height,
        }
    }
}

#[repr(C, packed)]
#[derive(Debug, AsBytes)]
pub struct ViewportOffset {
    cmd: u8,
    x_probably: u16,
    y_probably: u16,
}

impl ViewportOffset {
    pub fn new(x_probably: u16, y_probably: u16) -> Self {
        Self {
            cmd: 103,
            x_probably,
            y_probably,
        }
    }
}

impl Scene {
    pub fn init(render_width: u16, render_height: u16) -> Result<Scene, SceneError> {
        use v3d::flags::MemAllocFlags;

        // let render_handle = mem_alloc(
        //     0x10000,
        //     0x1000,
        //     MemAllocFlags::Coherent | MemAllocFlags::Zero,
        // )?;
        // let render_data = mem_lock(render_handle)?;
        // let loadpos = render_data;

        // let bin_width = (render_width as u32 + 63) / 64;
        // let bin_height = (render_height as u32 + 63) / 64;

        // let tile_mem_size = 0x4000;
        // let tile_handle = mem_alloc(
        //     tile_mem_size * 2,
        //     0x1000,
        //     MemAllocFlags::Coherent | MemAllocFlags::Zero,
        // )?;
        // let tile_state_data = mem_lock(tile_handle)?;

        // let tile_handle2 = mem_alloc(
        //     tile_mem_size * 2,
        //     0x1000,
        //     MemAllocFlags::Coherent | MemAllocFlags::Zero,
        // )?;
        // let tile_data_buffer = mem_lock(tile_handle2)?;

        // let binning_handle = mem_alloc(
        //     0x10000,
        //     0x1000,
        //     MemAllocFlags::Coherent | MemAllocFlags::Zero,
        // )?;
        // let binning_data = mem_lock(binning_handle)?;

        Ok(Scene {
            // loadpos,
            // render_handle,
            // render_data,
            render_width,
            render_height,
            // bin_width,
            // bin_height,
            // tile_handle,
            // tile_mem_size,
            // tile_state_data,
            // tile_data_buffer,
            // binning_handle,
            // binning_data,
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
        // self.vertex_vc4 = self.new_record_addr();

        self.vertex_vc4 = mem_lock(mem_alloc(
            0x1000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?)?;
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
        // self.loadpos = self.vertex_vc4 + writer.bytes_written() as u32;
        drop(writer);

        let index_data = &[0, 1, 2, 3, 4, 5, 4, 6, 5];

        // self.index_vertex = self.new_record_addr();
        self.index_vertex = mem_lock(mem_alloc(
            0x1000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?)?;
        let mut writer = Writer::new(self.index_vertex, index_data.len());
        writer.write(index_data);

        self.index_vertex_cnt = index_data.len() as u32;
        self.max_index_vertex = 6;

        // self.loadpos = self.index_vertex + writer.bytes_written() as u32;

        Ok(())
    }

    pub fn add_test_shaders(&mut self) -> Result<(), SceneError> {
        self.add_shader(&VERTEX_SHADER)?;
        // self.add_shader(&FILL_SHADER)?;
        Ok(())
    }

    pub fn add_shader(&mut self, shader: &[u32]) -> Result<(), SceneError> {
        // self.shader_start = self.new_record_addr();
        self.shader_start = mem_lock(mem_alloc(
            0x1000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?)?;
        let mut writer = Writer::new(
            self.shader_start,
            shader.len() * core::mem::size_of::<u32>(),
        );

        shader.iter().for_each(|val| writer.write(val.as_bytes()));
        // self.loadpos = self.shader_start + writer.bytes_written() as u32;
        drop(writer);

        // self.frag_shader_rec_start = self.new_record_addr();
        self.frag_shader_rec_start = mem_lock(mem_alloc(
            0x1000,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?)?;

        let shader_record = [
            [0x01u8, 6 * 4, 0xcc, 3],
            self.shader_start.to_ne_bytes(),
            0u32.to_ne_bytes(),
            self.vertex_vc4.to_ne_bytes(),
        ];
        let record_bytes = shader_record.as_bytes();

        let mut writer = Writer::new(self.frag_shader_rec_start, record_bytes.len());
        writer.write(&record_bytes);
        // self.loadpos = self.frag_shader_rec_start + writer.bytes_written() as u32;

        Ok(())
    }

    pub fn setup_render_control(&mut self, fb_addr: u32) -> Result<(), SceneError> {

        self.bin_width = (self.render_width as u32 + 31) >>5;
        self.bin_height = (self.render_height as u32 + 31) >>5;
        
        let size = ((self.bin_width * self.bin_height *20) + 37);

        self.render_handle = mem_alloc( //not released
            size,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;
        

        //SRC and DEST IMG HERE lock taken and not released on these
        //FMT SETTINGS HERE

        // mem_lock(self.tile_data_buffer); //will be released at the end
        let tile_render_config = 
            TileRenderModeConfig::new(self.render_width, self.render_height,0x04, 0, fb_addr);

        self.render_control = mem_lock(self.render_handle)?;
        // self.render_control = self.new_record_addr();
        let mut writer = Writer::new(self.render_control, 0x10000);

        writer.write(&[114]);
        writer.write(0u32.as_bytes());
        writer.write(0u32.as_bytes());
        writer.write(0u32.as_bytes());
        writer.write(&[0]);

        writer.write(tile_render_config.as_bytes());

        writer.write(&[115]);
        writer.write(&[0]);
        writer.write(&[0]);
        writer.write(&[28]);
        writer.write(0u16.as_bytes());
        writer.write(0u32.as_bytes());
        writer.write(&[1]);
        writer.write(&[1]);

        for y in 0..self.bin_height {
            for x in 0..self.bin_width {
                writer.write(TileCoords::new(x as u8, y as u8).as_bytes());
                writer.write(&[17]); // GL_BRANCH_TO_SUBLIST
                writer.write((self.tile_data_buffer + ((y + x * self.bin_width) * 32)).as_bytes());
                
                if (x == self.bin_width - 1) && (y == self.bin_height - 1){
                    writer.write(&[25]); // GL_STORE_MULTISAMPLE_END <-- special boi
                }
                else {
                    writer.write(&[24]); // GL_STORE_MULTISAMPLE
                }
            }
        }
        self.render_control_end = self.render_control + writer.bytes_written() as u32;
        // mem_unlock(self.tile_data_buffer);
        Ok(())
    }

    pub fn setup_binning_config(&mut self) -> Result<(), SceneError> {

        self.bin_width = (self.render_width as u32 + 31) >>5;
        self.bin_height = (self.render_height as u32 + 31) >>5;

        self.tile_mem_size = self.bin_width * self.bin_height * 64;
        self.tile_state_size = self.bin_width * self.bin_height * 48;

        self.tile_handle = mem_alloc(
            self.tile_mem_size,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;

        self.tile_state_handle = mem_alloc(
            self.tile_state_size,
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;

        self.binning_handle = mem_alloc(
            4096, //hard number from android
            0x1000,
            MemAllocFlags::Coherent | MemAllocFlags::Zero,
        )?;
        
        self.tile_data_buffer = mem_lock(self.tile_handle)?;
        self.tile_state_data = mem_lock(self.tile_state_handle)?;
        self.binning_data = mem_lock(self.binning_handle)?;

        let mut writer = Writer::new(self.binning_data, 0x4096);

        writer.write(
            TileBinningConfig::new(
                self.bin_width,
                self.bin_height,
                self.tile_data_buffer,
                self.tile_mem_size,
                self.tile_state_data,
                (1<<2) | (0<<5), // funny
            )
            .as_bytes(),
        );

        writer.write(&[6]); // GL_START_TILE_BINNING

        // primitive type
        writer.write(&[
            56,   // GL_PRIMITIVE_LIST_FORMAT
            0x12, // was 0x32 but android does 0x12 here
        ]);

        // clip window
        writer.write(ClipWindowConfig::new(0, 0, self.render_width, self.render_height).as_bytes());

        writer.write(&[
            96,   // GL_CONFIG_STATE
            0x03, // just
            0x00, // magic
            0x02, // bullshit
        ]);

        writer.write(ViewportOffset::new(0, 0).as_bytes());


        // everything after this android does not do, probably for universal. they just bin cfg end here

        // the fucking triangle baby we're finally fucking here holy shti what the fuck dude
        //
        // "No Vertex Shader" state (takes pre-transformed vertices so we don't have to supply a
        // working coordinate shader)
        // (apparently, except we gave it a fucking vertex shader already so idk why we even did
        // that)
        writer.write(&[65]); // GL_NV_SHADER_STATE
        writer.write(self.frag_shader_rec_start.as_bytes());

        // primitive index list
        writer.write(&[
           32, // GL_INDEXED_PRIMITIVE_LIST
           4,  // PRIM_TRIANGLE
        ]);

        writer.write(self.index_vertex_cnt.as_bytes());
        writer.write(self.index_vertex.as_bytes());
        writer.write(self.max_index_vertex.as_bytes());

        // End of bin list
        // So flush this shit and fucking get rid of it I never want to fucking see this aga

        writer.write(&[
            5, // GL_FLUSH_ALL_STATE
            1, // GL_NOP
            0, // GL_HALT
        ]);

        self.binning_cfg_end = self.binning_data + writer.bytes_written() as u32;

        Ok(())
    }

    /// Safety: no idea. The entire body of this function would've been wrapped in unsafe
    /// anyway, so for now, it's an unsafe fn. Deal with it.
    pub unsafe fn render(&self) -> Result<(), SceneError> {
        use crate::v3d::V3DRegisters as V3DReg;

        // clear caches
        write_v3d(V3DReg::L2CacheCtrl, 4);
        write_v3d(V3DReg::SliceCacheCtrl, 0x0f0f0f0f);

        // stop the thread
        write_v3d(V3DReg::ControlList0CS, 0x20);
        check_v3d_status();
        // wait for it to stop
        //
        // TODO: is this logic right? I always forget how to convert this kind of garbage logic
        // that C people love to do
        //
        // original was `while (v3d[V3D_CT0CS] & 0x20);`
        info!("while CL0CS != 0");
        while read_v3d(V3DReg::ControlList0CS) & 0x20 != 0 {
            info!("waiting for 0CS");
            core::hint::spin_loop();
        }

        // run our control list
        info!("waiting for BinningFlushCnt to be 0");
        write_v3d(V3DReg::BinningFlushCnt, 1);
        write_v3d(V3DReg::ControlList0CA, self.binning_data);
        write_v3d(V3DReg::ControlList0EA, self.binning_cfg_end);
        check_v3d_status();
        // wait for binning to finish
        while read_v3d(V3DReg::BinningFlushCnt) != 0 {
            core::hint::spin_loop();
        }

        // stop the thread
        write_v3d(V3DReg::ControlList1CS, 0x20);
        info!("CL1CS stop");
        check_v3d_status();
        // wait for thread to stop
        //
        // TODO: same stupid C shit as above
        // I think this is right but idk
        //while read_v3d(V3DReg::ControlList1CS) & 0x20 != 0 {
        //    info!("bingus");
        //    core::hint::spin_loop();
        //}
        info!("boingus");
        // run the god damn renderer finally
        
        info!("Sending RenderFrameCnt");
        write_v3d(V3DReg::RenderFrameCnt, 1);
        info!("Sending ControlList1CA");
        write_v3d(V3DReg::ControlList1CA, self.render_control);
        info!("Sending ControlList1EA");
        write_v3d(V3DReg::ControlList1EA, self.render_control_end);
        info!("Checking Status");
        check_v3d_status();
        info!("waiting for frame");
        while read_v3d(V3DReg::RenderFrameCnt) == 0 {
            core::hint::spin_loop();
        }

        Ok(()) // probably not ok lol
    }

    pub fn new_record_addr(&self) -> u32 {
        info!("something tried to align, find it");
        42069
        // (self.loadpos + 127) & ALIGN_128_BITS
    }
}

pub fn check_v3d_status() {
    use crate::v3d::V3DRegisters as V3DReg;

    let mut report = String::from("V3D Core Status Report:\n");

    // Check Control List 0 (binning) status and errors
    let ct0_status = read_v3d(V3DReg::ControlList0CS);
    report.push_str(&format!("Control List 0 Status: 0x{:X}\n", ct0_status));
    if ct0_status & 0xE00 != 0 {
        report.push_str("Errors in Control List 0:\n");
        if ct0_status & 0x200 != 0 {
            report.push_str("  - Out of Memory\n");
        }
        if ct0_status & 0x400 != 0 {
            report.push_str("  - Queue Full\n");
        }
        if ct0_status & 0x800 != 0 {
            report.push_str("  - DMA Overflow\n");
        }
    } else {
        report.push_str("  No errors in Control List 0.\n");
    }

    // Check Control List 1 (rendering) status and errors
    let ct1_status = read_v3d(V3DReg::ControlList1CS);
    report.push_str(&format!("Control List 1 Status: 0x{:X}\n", ct1_status));
    if ct1_status & 0xE00 != 0 {
        report.push_str("Errors in Control List 1:\n");
        if ct1_status & 0x200 != 0 {
            report.push_str("  - Out of Memory\n");
        }
        if ct1_status & 0x400 != 0 {
            report.push_str("  - Queue Full\n");
        }
        if ct1_status & 0x800 != 0 {
            report.push_str("  - DMA Overflow\n");
        }
    } else {
        report.push_str("  No errors in Control List 1.\n");
    }


    // Performance Counters
    let perf_counter0 = read_v3d(V3DReg::PerfCntrCnt0); // Cache hits
    let perf_counter1 = read_v3d(V3DReg::PerfCntrCnt1); // Cache misses
    let perf_counter2 = read_v3d(V3DReg::PerfCntrCnt2); // Vertex shader stalls
    let perf_counter3 = read_v3d(V3DReg::PerfCntrCnt3); // Instruction fetch stalls
    report.push_str(&format!(
        "Performance Counters:\n  - Cache Hits: {}\n  - Cache Misses: {}\n  - Vertex Shader Stalls: {}\n  - Instruction Fetch Stalls: {}\n",
        perf_counter0, perf_counter1, perf_counter2, perf_counter3
    ));

    // V3D Identifier Registers
    let ident0 = read_v3d(V3DReg::Ident0);
    let ident1 = read_v3d(V3DReg::Ident1);
    let ident2 = read_v3d(V3DReg::Ident2);
    report.push_str(&format!(
        "V3D Identifiers:\n  - Ident0: 0x{:X}\n  - Ident1: 0x{:X}\n  - Ident2: 0x{:X}\n",
        ident0, ident1, ident2
    ));

    // L2 Cache and Slice Cache Control
    let l2_cache_ctrl = read_v3d(V3DReg::L2CacheCtrl);
    let slice_cache_ctrl = read_v3d(V3DReg::SliceCacheCtrl);
    report.push_str(&format!(
        "Cache Controls:\n  - L2 Cache Control: 0x{:X}\n  - Slice Cache Control: 0x{:X}\n",
        l2_cache_ctrl, slice_cache_ctrl
    ));

    // Check Binning and Rendering Completion
    let binning_flush_count = read_v3d(V3DReg::BinningFlushCnt);
    let render_frame_count = read_v3d(V3DReg::RenderFrameCnt);
    report.push_str("Pipeline Status:\n");
    if binning_flush_count == 0 {
        report.push_str("  - Binning complete.\n");
    } else {
        report.push_str(&format!("  - Binning in progress ({} flushes remaining).\n", binning_flush_count));
    }
    if render_frame_count == 0 {
        report.push_str("  - Rendering complete.\n");
    } else {
        report.push_str(&format!("  - Rendering in progress ({} frames remaining).\n", render_frame_count));
    }

    // Final consolidated status output
    info!("{}", report);
}
fn read_v3d(reg: u32) -> u32 {
    unsafe{
    core::ptr::read_volatile(get_v3d_ptr().add(reg as usize / 4))
}}

fn write_v3d(reg: u32, val: u32) {
    unsafe{
    core::ptr::write_volatile(get_v3d_ptr().add(reg as usize / 4), val);
}}

#[derive(Debug)]
pub enum SceneError {
    AllocMemory,
    LockMemory,
    Trash,
}

fn mem_alloc(size: u32, align: u32, flags: u32) -> Result<GpuHandle, SceneError> {
    let handle = MailboxMessage::mem_alloc(size, align, flags)
        .send_and_read(5)
        .map_err(|_| SceneError::AllocMemory)?;

    Ok(handle)
}

fn mem_lock(handle: GpuHandle) -> Result<Vc4Addr, SceneError> {
    MailboxMessage::mem_lock(handle)
        .send_and_read(5)
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
