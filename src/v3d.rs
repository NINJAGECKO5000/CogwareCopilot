use crate::info;
const RPI_IO_BASE_ADDR: usize = 0x3F00_0000; // Replace with actual base address
const V3D_OFFSET: usize = 0xc00000;
const V3D_IDENT0: usize = 0x000;

// The memory address for the V3D base address
fn get_v3d_ptr() -> *mut u32 {
    let address = RPI_IO_BASE_ADDR + V3D_OFFSET;
    address as *mut u32
}

fn check_v3d_ident0() -> bool {
    unsafe {
        // Get the pointer to the V3D registers
        let v3d_ptr = get_v3d_ptr();
        info!("V3DPTR = {:?}", v3d_ptr);
        // Read the value at V3D_IDENT0 offset using volatile read
        let v3d_ident0_value = core::ptr::read_volatile(v3d_ptr.add(V3D_IDENT0 / 4)); // Divide by 4 because u32 is 4 bytes
        info!("V3D IDENT0 VAL {:?}", v3d_ident0_value);
        // Check if the value matches 0x02443356
        if v3d_ident0_value == 0x02443356 {
            return true;
        }
    }
    false
}
