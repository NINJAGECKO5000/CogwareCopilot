
use crate::egl::*;
use crate::glesv2::*;
use alloc::ffi::CString;
pub const EGL_CONTEXT_CLIENT_VERSION: i32 = 0x3098;
pub struct TriangleRenderer {
    program: u32,
    vbo: u32,
}

const VERTEX_SHADER: &str = "
    attribute vec2 aPos;
    void main() {
        gl_Position = vec4(aPos, 0.0, 1.0);
    }
";

const FRAGMENT_SHADER: &str = "
    void main() {
        gl_FragColor = vec4(1.0, 0.5, 0.2, 1.0);  // Orange color
    }
";

const TRIANGLE_VERTICES: [f32; 6] = [
    -0.5, -0.5,  // Bottom left
     0.5, -0.5,  // Bottom right
     0.0,  0.5,  // Top center
];

impl TriangleRenderer {
    /// Initializes the EGL context and sets up a triangle.
    /// Takes a framebuffer pointer as input.
    pub fn init(framebuffer: *mut core::ffi::c_void) -> TriangleRenderer {
        // Initialize EGL and bind it to the framebuffer
        let (egl_display, surface, context) = init_egl(framebuffer);

        // Compile shaders and link the program
        let vertex_shader = compile_shader(VERTEX_SHADER, GL_VERTEX_SHADER);
        let fragment_shader = compile_shader(FRAGMENT_SHADER, GL_FRAGMENT_SHADER);
        let program = link_program(vertex_shader, fragment_shader);

        let mut vbo = 0;
        unsafe {
            glGenBuffers(1, &mut vbo);
            glBindBuffer(GL_ARRAY_BUFFER, vbo);
            glBufferData(
                GL_ARRAY_BUFFER,
                (TRIANGLE_VERTICES.len() * core::mem::size_of::<f32>()) as isize,
                TRIANGLE_VERTICES.as_ptr() as *const _,
                GL_STATIC_DRAW,
            );
        }

        TriangleRenderer { program, vbo }
    }

    pub fn draw(&self) {
        unsafe {
            glClear(GL_COLOR_BUFFER_BIT);
            glClearColor(0.1, 0.1, 0.1, 1.0);

            glUseProgram(self.program);

            let pos_attrib = glGetAttribLocation(self.program, "aPos\0".as_ptr() as *const u8) as u32;
            glEnableVertexAttribArray(pos_attrib);
            glVertexAttribPointer(pos_attrib, 2, GL_FLOAT, 0, 0, core::ptr::null());

            glDrawArrays(GL_TRIANGLES, 0, 3);

            glDisableVertexAttribArray(pos_attrib);
        }
    }
}

/// Initializes the EGL context and binds it to the provided framebuffer pointer.
fn init_egl(framebuffer: *mut core::ffi::c_void) -> (*mut core::ffi::c_void, *mut core::ffi::c_void, *mut core::ffi::c_void) {
    let egl_display = unsafe { eglGetDisplay(framebuffer) };
    let mut major = 0;
    let mut minor = 0;

    unsafe {
        eglInitialize(egl_display, &mut major, &mut minor);
    }

    let attribs = [
        EGL_RED_SIZE, 8,
        EGL_GREEN_SIZE, 8,
        EGL_BLUE_SIZE, 8,
        EGL_ALPHA_SIZE, 8,
        EGL_DEPTH_SIZE, 16,
        EGL_STENCIL_SIZE, 8,
        EGL_SURFACE_TYPE, EGL_WINDOW_BIT,
        EGL_RENDERABLE_TYPE, EGL_OPENGL_ES2_BIT,
        EGL_NONE,
    ];

    let mut config: *mut core::ffi::c_void = core::ptr::null_mut();
    let mut num_config = 0;

    unsafe {
        eglChooseConfig(egl_display, attribs.as_ptr(), &mut config, 1, &mut num_config);
    }

    let surface = unsafe {
        eglCreateWindowSurface(egl_display, config, framebuffer, core::ptr::null())
    };

    let context_attribs = [
        EGL_CONTEXT_CLIENT_VERSION, 2,
        EGL_NONE,
    ];

    let context = unsafe {
        eglCreateContext(egl_display, config, core::ptr::null_mut(), context_attribs.as_ptr())
    };

    unsafe {
        eglMakeCurrent(egl_display, surface, surface, context);
    }

    (egl_display, surface, context)
}

fn compile_shader(source: &str, shader_type: u32) -> u32 {
    let shader = unsafe { glCreateShader(shader_type) };
    let c_str = CString::new(source).unwrap();
    let ptr = c_str.as_ptr();

    unsafe {
        glShaderSource(shader, 1, ptr as *const u8, core::ptr::null());
        glCompileShader(shader);

        let mut success = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut success);
        if success == 0 {
            panic!("Shader compilation failed");
        }
    }

    shader
}

fn link_program(vertex_shader: u32, fragment_shader: u32) -> u32 {
    let program = unsafe { glCreateProgram() };
    unsafe {
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);

        let mut success = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut success);
        if success == 0 {
            panic!("Program linking failed");
        }
    }

    program
}