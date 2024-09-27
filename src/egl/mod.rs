pub const EGL_SUCCESS: i32 = 0x3000;
pub const EGL_NOT_INITIALIZED: i32 = 0x3001;
pub const EGL_BAD_ACCESS: i32 = 0x3002;
pub const EGL_BAD_ALLOC: i32 = 0x3003;
pub const EGL_BAD_ATTRIBUTE: i32 = 0x3004;
pub const EGL_BAD_CONFIG: i32 = 0x3005;
pub const EGL_BAD_CONTEXT: i32 = 0x3006;
pub const EGL_BAD_CURRENT_SURFACE: i32 = 0x3007;
pub const EGL_BAD_DISPLAY: i32 = 0x3008;
pub const EGL_BAD_MATCH: i32 = 0x3009;
pub const EGL_BAD_NATIVE_PIXMAP: i32 = 0x300A;
pub const EGL_BAD_NATIVE_WINDOW: i32 = 0x300B;
pub const EGL_BAD_PARAMETER: i32 = 0x300C;
pub const EGL_BAD_SURFACE: i32 = 0x300D;
pub const EGL_CONTEXT_LOST: i32 = 0x300E;
pub const EGL_BUFFER_SIZE: i32 = 0x3020;
pub const EGL_ALPHA_SIZE: i32 = 0x3021;
pub const EGL_BLUE_SIZE: i32 = 0x3022;
pub const EGL_GREEN_SIZE: i32 = 0x3023;
pub const EGL_RED_SIZE: i32 = 0x3024;
pub const EGL_DEPTH_SIZE: i32 = 0x3025;
pub const EGL_STENCIL_SIZE: i32 = 0x3026;
pub const EGL_CONFIG_CAVEAT: i32 = 0x3027;
pub const EGL_CONFIG_ID: i32 = 0x3028;
pub const EGL_LEVEL: i32 = 0x3029;
pub const EGL_MAX_PBUFFER_HEIGHT: i32 = 0x302A;
pub const EGL_MAX_PBUFFER_PIXELS: i32 = 0x302B;
pub const EGL_MAX_PBUFFER_WIDTH: i32 = 0x302C;
pub const EGL_NATIVE_RENDERABLE: i32 = 0x302D;
pub const EGL_NATIVE_VISUAL_ID: i32 = 0x302E;
pub const EGL_NATIVE_VISUAL_TYPE: i32 = 0x302F;
pub const EGL_SAMPLES: i32 = 0x3031;
pub const EGL_SAMPLE_BUFFERS: i32 = 0x3032;
pub const EGL_SURFACE_TYPE: i32 = 0x3033;
pub const EGL_TRANSPARENT_TYPE: i32 = 0x3034;
pub const EGL_TRANSPARENT_BLUE_VALUE: i32 = 0x3035;
pub const EGL_TRANSPARENT_GREEN_VALUE: i32 = 0x3036;
pub const EGL_TRANSPARENT_RED_VALUE: i32 = 0x3037;
pub const EGL_NONE: i32 = 0x3038;
pub const EGL_BIND_TO_TEXTURE_RGB: i32 = 0x3039;
pub const EGL_BIND_TO_TEXTURE_RGBA: i32 = 0x303A;
pub const EGL_MIN_SWAP_INTERVAL: i32 = 0x303B;
pub const EGL_MAX_SWAP_INTERVAL: i32 = 0x303C;
pub const EGL_LUMINANCE_SIZE: i32 = 0x303D;
pub const EGL_ALPHA_MASK_SIZE: i32 = 0x303E;
pub const EGL_COLOR_BUFFER_TYPE: i32 = 0x303F;
pub const EGL_RENDERABLE_TYPE: i32 = 0x3040;
pub const EGL_MATCH_NATIVE_PIXMAP: i32 = 0x3041;
pub const EGL_CONFORMANT: i32 = 0x3042;
pub const EGL_OPENGL_ES_BIT: i32 = 0x0001;
pub const EGL_OPENVG_BIT: i32 = 0x0002;
pub const EGL_OPENGL_ES2_BIT: i32 = 0x0004;
pub const EGL_OPENGL_BIT: i32 = 0x0008;
pub const EGL_PBUFFER_BIT: i32 = 0x0001;
pub const EGL_PIXMAP_BIT: i32 = 0x0002;
pub const EGL_WINDOW_BIT: i32 = 0x0004;
pub const EGL_VG_COLORSPACE_LINEAR_BIT: i32 = 0x0020;
pub const EGL_VG_ALPHA_FORMAT_PRE_BIT: i32 = 0x0040;
pub const EGL_MULTISAMPLE_RESOLVE_BOX_BIT: i32 = 0x0200;
pub const EGL_SWAP_BEHAVIOR_PRESERVED_BIT: i32 = 0x0400;

extern "C" {
    // 1. Get an EGL display connection
    pub fn eglGetDisplay(display_id: *mut core::ffi::c_void) -> *mut core::ffi::c_void;

    // 2. Initialize EGL
    pub fn eglInitialize(dpy: *mut core::ffi::c_void, major: *mut i32, minor: *mut i32) -> i32;

    // 3. Terminate EGL
    pub fn eglTerminate(dpy: *mut core::ffi::c_void) -> i32;

    // 4. Choose an appropriate EGL frame buffer configuration
    pub fn eglChooseConfig(
        dpy: *mut core::ffi::c_void,
        attrib_list: *const i32,
        configs: *mut *mut core::ffi::c_void,
        config_size: i32,
        num_config: *mut i32,
    ) -> i32;

    // 5. Create an EGL rendering surface
    pub fn eglCreateWindowSurface(
        dpy: *mut core::ffi::c_void,
        config: *mut core::ffi::c_void,
        win: *mut core::ffi::c_void,
        attrib_list: *const i32,
    ) -> *mut core::ffi::c_void;

    // 6. Create an EGL rendering context
    pub fn eglCreateContext(
        dpy: *mut core::ffi::c_void,
        config: *mut core::ffi::c_void,
        share_context: *mut core::ffi::c_void,
        attrib_list: *const i32,
    ) -> *mut core::ffi::c_void;

    // 7. Make the context current
    pub fn eglMakeCurrent(
        dpy: *mut core::ffi::c_void,
        draw: *mut core::ffi::c_void,
        read: *mut core::ffi::c_void,
        ctx: *mut core::ffi::c_void,
    ) -> i32;

    // 8. Swap buffers (for rendering)
    pub fn eglSwapBuffers(
        dpy: *mut core::ffi::c_void,
        surface: *mut core::ffi::c_void,
    ) -> i32;

    // 9. Destroy a context
    pub fn eglDestroyContext(
        dpy: *mut core::ffi::c_void,
        ctx: *mut core::ffi::c_void,
    ) -> i32;

    // 10. Destroy a surface
    pub fn eglDestroySurface(
        dpy: *mut core::ffi::c_void,
        surface: *mut core::ffi::c_void,
    ) -> i32;

    // 11. Get EGL error
    pub fn eglGetError() -> i32;
}