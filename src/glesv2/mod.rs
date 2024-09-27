pub const GL_NO_ERROR: u32 = 0x0;
pub const GL_INVALID_ENUM: u32 = 0x0500;
pub const GL_INVALID_VALUE: u32 = 0x0501;
pub const GL_INVALID_OPERATION: u32 = 0x0502;
pub const GL_OUT_OF_MEMORY: u32 = 0x0505;
pub const GL_TRIANGLES: u32 = 0x0004;
pub const GL_LINES: u32 = 0x0001;
pub const GL_COLOR_BUFFER_BIT: u32 = 0x4000;
pub const GL_DEPTH_BUFFER_BIT: u32 = 0x0100;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_FRAGMENT_SHADER: u32 = 0x8B30;
pub const GL_VERTEX_SHADER: u32 = 0x8B31;
pub const GL_COMPILE_STATUS: u32 = 0x8B81;
pub const GL_LINK_STATUS: u32 = 0x8B82;
pub const GL_ARRAY_BUFFER: u32 = 0x8892;
pub const GL_STATIC_DRAW: u32 = 0x88E4;

extern "C" {
    // 1. Clear the screen or buffers
    pub fn glClear(mask: u32);

    // 2. Set the clear color
    pub fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32);

    // 3. Enable a specific OpenGL capability
    pub fn glEnable(cap: u32);

    // 4. Disable a specific OpenGL capability
    pub fn glDisable(cap: u32);

    // 5. Specify the viewport
    pub fn glViewport(x: i32, y: i32, width: i32, height: i32);

    // 6. Bind a buffer
    pub fn glBindBuffer(target: u32, buffer: u32);

    // 7. Buffer data
    pub fn glBufferData(target: u32, size: isize, data: *const core::ffi::c_void, usage: u32);

    // 8. Enable a vertex attribute array
    pub fn glEnableVertexAttribArray(index: u32);

    // 9. Disable a vertex attribute array
    pub fn glDisableVertexAttribArray(index: u32);

    // 10. Define an array of vertex attribute data
    pub fn glVertexAttribPointer(
        index: u32,
        size: i32,
        type_: u32,
        normalized: u8,
        stride: i32,
        pointer: *const core::ffi::c_void,
    );

    // 11. Compile a shader
    pub fn glCompileShader(shader: u32);

    // 12. Create a shader
    pub fn glCreateShader(type_: u32) -> u32;

    // 13. Create a program
    pub fn glCreateProgram() -> u32;

    // 14. Attach a shader to a program
    pub fn glAttachShader(program: u32, shader: u32);

    // 15. Link a program
    pub fn glLinkProgram(program: u32);

    // 16. Use a program
    pub fn glUseProgram(program: u32);

    // 17. Draw arrays
    pub fn glDrawArrays(mode: u32, first: i32, count: i32);

    // 18. Get error
    pub fn glGetError() -> u32;

    // 19. Delete a shader
    pub fn glDeleteShader(shader: u32);

    // 20. Delete a program
    pub fn glDeleteProgram(program: u32);

    // 21. Get the shader compile status
    pub fn glGetShaderiv(shader: u32, pname: u32, params: *mut i32);

    // 22. Get the program link status
    pub fn glGetProgramiv(program: u32, pname: u32, params: *mut i32);

    // 23. Shader source
    pub fn glShaderSource(shader: u32, count: i32, string: *const u8, length: *const i32);

    // 24. Get shader info log
    pub fn glGetShaderInfoLog(
        shader: u32,
        maxLength: i32,
        length: *mut i32,
        infoLog: *mut u8,
    );

    // 25. Get program info log
    pub fn glGetProgramInfoLog(
        program: u32,
        maxLength: i32,
        length: *mut i32,
        infoLog: *mut u8,
    );
    // 26. Get Attribute Location
    pub fn glGetAttribLocation(program: u32, name: *const u8) -> i32;

    // Generate buffer object names
    pub fn glGenBuffers(n: i32, buffers: *mut u32);
}