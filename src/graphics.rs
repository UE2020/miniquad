use std::{ffi::CString, mem};

use sokol_app_sys::sokol_app::*;
use std::option::Option::None;

pub const LINEAR_FILTER: i32 = GL_LINEAR as i32;
pub const NEAREST_FILTER: i32 = GL_NEAREST as i32;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Texture {
    texture: GLuint,
}

impl Texture {
    pub fn from_rgba8(width: u16, height: u16, bytes: &[u8]) -> Texture {
        unsafe {
            let mut texture: GLuint = 0;
            glGenTextures(1, &mut texture as *mut _);
            glActiveTexture(GL_TEXTURE0);
            glBindTexture(GL_TEXTURE_2D, texture);
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                width as i32,
                height as i32,
                0,
                GL_RGBA,
                GL_UNSIGNED_BYTE,
                bytes.as_ptr() as *const _,
            );

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR as i32);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR as i32);

            Texture { texture }
        }
    }

    pub fn set_filter(&self, filter: i32) {
        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.texture);

            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, filter);
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, filter);
        }
    }
}

fn get_uniform_location(program: GLuint, name: &str) -> i32 {
    let cname = CString::new(name).unwrap_or_else(|e| panic!(e));
    let location = unsafe { glGetUniformLocation(program, cname.as_ptr()) };

    assert!(
        location != -1,
        format!("Cant get \"{}\" uniform location", name)
    );

    location
}

#[derive(Clone, Copy, Debug)]
pub enum UniformType {
    Float1,
    Float2,
    Float3,
    Float4,
    Mat4,
}

impl UniformType {
    fn size(&self, count: usize) -> usize {
        match self {
            UniformType::Float1 => 4 * count,
            UniformType::Float2 => 8 * count,
            UniformType::Float3 => 12 * count,
            UniformType::Float4 => 16 * count,
            UniformType::Mat4 => 64 * count,
        }
    }
}

struct Uniform {
    gl_loc: GLint,
    uniform_type: UniformType,
}

pub struct UniformBlockLayout {
    pub uniforms: &'static [(&'static str, UniformType)],
}

pub struct ShaderMeta {
    pub uniforms: UniformBlockLayout,
    pub images: &'static [&'static str],
}

#[derive(Clone, Copy, Debug)]
pub enum VertexFormat {
    Float1,
    Float2,
    Float3,
}

impl VertexFormat {
    pub fn size(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1,
            VertexFormat::Float2 => 2,
            VertexFormat::Float3 => 3,
        }
    }
}

#[derive(Clone)]
pub enum VertexAttribute {
    Position,
    Normal,
    TexCoord0,
    Custom(&'static str),
}

#[derive(Clone)]
pub struct VertexLayout {
    pub attributes: &'static [(VertexAttribute, VertexFormat)],
    len: i32,
}
impl VertexLayout {
    pub fn new(attributes: &'static [(VertexAttribute, VertexFormat)]) -> Self {
        let len = attributes.iter().map(|(_, f)| f.size()).sum();

        VertexLayout { attributes, len }
    }
}

pub struct Shader(usize);

impl Shader {
    pub fn new(
        ctx: &mut Context,
        vertex_shader: &str,
        fragment_shader: &str,
        meta: ShaderMeta,
    ) -> Shader {
        let shader = load_shader_Internal(vertex_shader, fragment_shader, meta);
        ctx.shaders.push(shader);
        Shader(ctx.shaders.len() - 1)
    }
}

pub struct ShaderImage {
    gl_loc: GLint,
}

#[derive(Debug)]
pub struct ShaderUniform {
    gl_loc: GLint,
    offset: usize,
    size: usize,
    uniform_type: UniformType,
}

pub struct ShaderInternal {
    pub program: GLuint,
    pub images: Vec<ShaderImage>,
    pub uniforms: Vec<ShaderUniform>,
}

type BlendState = Option<(Equation, BlendFactor, BlendFactor)>;

pub struct GlCache {
    stored_index_buffer: GLuint,
    stored_vertex_buffer: GLuint,
    index_buffer: GLuint,
    vertex_buffer: GLuint,
    cur_pipeline: Option<Pipeline>,
    blend: BlendState,
}

pub const MAX_VERTEX_ATTRIBUTES: usize = 16;

pub struct Context {
    pub screen_rect: (f32, f32, f32, f32),
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    //default_framebuffer: GLint,
    cache: GlCache,
    attributes: [Option<VertexAttributeInternal>; MAX_VERTEX_ATTRIBUTES],
}

impl Context {
    pub fn new() -> Context {
        unsafe {
            let screen_rect = (-1., -1., 2., 2.);
            //let mut default_framebuffer: GLint = 0;
            unsafe {
                //glGetIntegerv(GL_FRAMEBUFFER_BINDING, &mut default_framebuffer as *mut _);
            }

            let mut vao = 0;
            unsafe {
                glGenVertexArrays(1, &mut vao as *mut _);
                glBindVertexArray(vao);
            }

            Context {
                screen_rect,
                //default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                cache: GlCache {
                    stored_index_buffer: 0,
                    stored_vertex_buffer: 0,
                    index_buffer: 0,
                    vertex_buffer: 0,
                    cur_pipeline: None,
                    blend: None,
                },
                attributes: [None; 16],
            }
        }
    }

    pub(crate) fn resize(&mut self, w: u32, h: u32) {
        unsafe {
            glViewport(0, 0, w as i32, h as i32);
        }
    }

    pub fn screen_size(&self) -> (f32, f32) {
        unsafe { (sapp_width() as f32, sapp_height() as f32) }
    }

    fn bind_buffer(&mut self, target: GLenum, buffer: GLuint) {
        if target == GL_ARRAY_BUFFER {
            if self.cache.vertex_buffer != buffer {
                self.cache.vertex_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        } else {
            if self.cache.index_buffer != buffer {
                self.cache.index_buffer = buffer;
                unsafe {
                    glBindBuffer(target, buffer);
                }
            }
        }
    }

    fn store_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.cache.stored_vertex_buffer = self.cache.vertex_buffer;
        } else {
            self.cache.stored_index_buffer = self.cache.index_buffer;
        }
    }

    fn restore_buffer_binding(&mut self, target: GLenum) {
        if target == GL_ARRAY_BUFFER {
            self.bind_buffer(target, self.cache.stored_vertex_buffer);
        } else {
            self.bind_buffer(target, self.cache.stored_index_buffer);
        }
    }

    pub fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        self.cache.cur_pipeline = Some(*pipeline);

        let pipeline = &mut self.pipelines[pipeline.0];
        let shader = &mut self.shaders[pipeline.shader.0];
        unsafe {
            glUseProgram(shader.program);
        }

        if self.cache.blend != pipeline.params.color_blend {
            unsafe {
                if let Some((equation, src, dst)) = pipeline.params.color_blend {
                    if self.cache.blend.is_none() {
                        glEnable(GL_BLEND);
                    }

                    glBlendFunc(src.into(), dst.into());
                    glBlendEquationSeparate(equation.into(), equation.into());
                } else if self.cache.blend.is_some() {
                    glDisable(GL_BLEND);
                }

                self.cache.blend = pipeline.params.color_blend;
            }
        }
    }

    pub fn apply_bindings(&mut self, bindings: &Bindings) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, image) in shader.images.iter().enumerate() {
            unsafe {
                glActiveTexture(GL_TEXTURE0 + n as u32);
                glBindTexture(GL_TEXTURE_2D, bindings.images[n].texture);
                glUniform1i(image.gl_loc, n as i32);
            }
        }

        let vb_dirty = self.cache.vertex_buffer != bindings.vertex_buffer.gl_buf;

        self.bind_buffer(GL_ELEMENT_ARRAY_BUFFER, bindings.index_buffer.gl_buf);
        self.bind_buffer(GL_ARRAY_BUFFER, bindings.vertex_buffer.gl_buf);

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for i in 0..MAX_VERTEX_ATTRIBUTES {
            let mut cached_attribute = &mut self.attributes[i];
            let pip_attribute = pip.layout.get(i).copied();

            match (&cached_attribute, pip_attribute) {
                // cached and the new one are the same
                (Some(old), Some(new)) if *old == new && vb_dirty == false => {}
                // there was no cached attribute or cached and the new are different
                (None, Some(new)) | (Some(_), Some(new)) => {
                    unsafe {
                        glVertexAttribPointer(
                            new.gl_loc,
                            new.size,
                            GL_FLOAT,
                            GL_FALSE as u8,
                            new.stride as i32,
                            new.offset as *mut _,
                        );
                        glEnableVertexAttribArray(new.gl_loc);
                    };
                }
                (Some(attr), None) => unsafe {
                    glDisableVertexAttribArray(attr.gl_loc);
                },
                // attributes are always consecutive in the cache, so its safe to just break from the loop when both
                // cached and the new attributes are None
                (None, None) => break,
            }
            *cached_attribute = pip_attribute;
        }
    }

    pub unsafe fn apply_uniforms<U>(&mut self, uniforms: &U) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for (n, uniform) in shader.uniforms.iter().enumerate() {
            use UniformType::*;

            unsafe {
                let data = (uniforms as *const _ as *const f32).offset(offset / 4);

                match uniform.uniform_type {
                    Float1 => {
                        glUniform1fv(uniform.gl_loc, 1, data);
                    }
                    Float2 => {
                        glUniform2fv(uniform.gl_loc, 1, data);
                    }
                    Float3 => {
                        glUniform3fv(uniform.gl_loc, 1, data);
                    }
                    Float4 => {
                        glUniform4fv(uniform.gl_loc, 1, data);
                    }
                    Mat4 => {
                        glUniformMatrix4fv(uniform.gl_loc, 1, 0, data);
                    }
                }
                offset += uniform.uniform_type.size(1) as isize;
            }
        }
    }

    pub fn clear(&self, color: (f32, f32, f32, f32)) {
        unsafe {
            glClearColor(color.0, color.1, color.2, color.3);
            glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
        }
    }

    pub fn begin_render_pass(&self) {
    }

    pub fn end_render_pass(&self) {}

    pub fn commit_frame(&self) {}

    pub fn draw(&self, num_elements: i32) {
        let p_type = GL_TRIANGLES;
        unsafe {
            glDrawElements(p_type, num_elements, GL_UNSIGNED_SHORT, std::ptr::null());
        }
    }
}

fn load_shader_Internal(
    vertex_shader: &str,
    fragment_shader: &str,
    meta: ShaderMeta,
) -> ShaderInternal {
    unsafe {
        let vertex_shader = load_shader(GL_VERTEX_SHADER, vertex_shader);
        let fragment_shader = load_shader(GL_FRAGMENT_SHADER, fragment_shader);

        let program = glCreateProgram();
        glAttachShader(program, vertex_shader);
        glAttachShader(program, fragment_shader);
        glLinkProgram(program);

        let mut link_status = 0;
        glGetProgramiv(program, GL_LINK_STATUS, &mut link_status as *mut _);
        if link_status == 0 {
            let mut max_length = 100;
            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetProgramInfoLog(
                program,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );

            let error_message = std::string::String::from_utf8_lossy(&error_message);
            panic!("{}", error_message);
        }

        glUseProgram(program);

        #[rustfmt::skip]
        let images = meta.images.iter().map(|name| ShaderImage {
                gl_loc: get_uniform_location(program, name),
            }).collect();
        #[rustfmt::skip]
        let uniforms = meta.uniforms.uniforms.iter().scan(0, |offset, uniform| {
            let res = ShaderUniform {
                gl_loc: get_uniform_location(program, uniform.0),
                offset: *offset,
                size: uniform.1.size(1),
                uniform_type: uniform.1
            };
            *offset += uniform.1.size(1);
            Some(res)
        }).collect();
        ShaderInternal {
            program,
            images,
            uniforms,
        }
    }
}

pub fn load_shader(shader_type: GLenum, source: &str) -> GLuint {
    unsafe {
        let shader = glCreateShader(shader_type);

        assert!(shader != 0);

        let cstring = CString::new(source).unwrap_or_else(|e| panic!(e));
        let csource = [cstring];
        glShaderSource(shader, 1, csource.as_ptr() as *const _, std::ptr::null());
        glCompileShader(shader);

        let mut is_compiled = 0;
        glGetShaderiv(shader, GL_COMPILE_STATUS, &mut is_compiled as *mut _);
        if is_compiled == 0 {
            let mut max_length: i32 = 0;
            glGetShaderiv(shader, GL_INFO_LOG_LENGTH, &mut max_length as *mut _);

            let mut error_message = vec![0u8; max_length as usize + 1];
            glGetShaderInfoLog(
                shader,
                max_length,
                &mut max_length as *mut _,
                error_message.as_mut_ptr() as *mut _,
            );

            #[cfg(target_arch = "wasm32")]
            test_log(error_message.as_ptr() as *const _);

            let error_message = std::string::String::from_utf8_lossy(&error_message);
            eprintln!("{} {:?}", max_length, error_message);
            glDeleteShader(shader);
            panic!("cant compile shader!");
        }

        shader
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FilterMode {
    Linear = LINEAR_FILTER as isize,
    Nearest = NEAREST_FILTER as isize,
}

/// Specify whether front- or back-facing polygons can be culled.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/// Define front- and back-facing polygons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrontFaceOrder {
    Clockwise,
    CounterClockwise,
}

/// A pixel-wise comparison function.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Never,
    Less,
    LessOrEqual,
    Greater,
    GreaterOrEqual,
    Equal,
    NotEqual,
    Always,
}
/// Specifies how incoming RGBA values (source) and the RGBA in framebuffer (destination)
/// are combined.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Equation {
    /// Adds source and destination. Source and destination are multiplied
    /// by blending parameters before addition.
    Add,
    /// Subtracts destination from source. Source and destination are
    /// multiplied by blending parameters before subtraction.
    Subtract,
    /// Subtracts source from destination. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ReverseSubtract,
}

/// Blend values.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestinationColor,
    DestinationAlpha,
}

/// Blend factors.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendFactor {
    Zero,
    One,
    Value(BlendValue),
    OneMinusValue(BlendValue),
}

impl From<Equation> for GLenum {
    fn from(eq: Equation) -> Self {
        match eq {
            Equation::Add => GL_FUNC_ADD,
            Equation::Subtract => GL_FUNC_SUBTRACT,
            Equation::ReverseSubtract => GL_FUNC_REVERSE_SUBTRACT,
        }
    }
}

impl From<BlendFactor> for GLenum {
    fn from(factor: BlendFactor) -> GLenum {
        match factor {
            BlendFactor::Zero => GL_ZERO,
            BlendFactor::One => GL_ONE,
            BlendFactor::Value(BlendValue::SourceColor) => GL_SRC_COLOR,
            BlendFactor::Value(BlendValue::SourceAlpha) => GL_SRC_ALPHA,
            BlendFactor::Value(BlendValue::DestinationColor) => GL_DST_COLOR,
            BlendFactor::Value(BlendValue::DestinationAlpha) => GL_DST_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::SourceColor) => GL_ONE_MINUS_SRC_COLOR,
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha) => GL_ONE_MINUS_SRC_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::DestinationColor) => GL_ONE_MINUS_DST_COLOR,
            BlendFactor::OneMinusValue(BlendValue::DestinationAlpha) => GL_ONE_MINUS_DST_ALPHA,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PipelineParams {
    pub cull_face: CullFace,
    pub front_face_order: FrontFaceOrder,
    pub depth_test: Comparison,
    pub depth_write: bool,
    pub depth_write_offset: Option<(f32, f32)>,
    pub color_blend: BlendState,
    pub color_write: (bool, bool, bool, bool),
}

#[derive(Copy, Clone, Debug)]
pub struct Pipeline(usize);

impl Default for PipelineParams {
    fn default() -> PipelineParams {
        PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always, // no depth test,
            depth_write: false,             // no depth write,
            depth_write_offset: None,
            color_blend: None,
            color_write: (true, true, true, true),
        }
    }
}

impl Pipeline {
    pub fn new(ctx: &mut Context, layout: VertexLayout, shader: Shader) -> Pipeline {
        Self::with_params(ctx, layout, shader, Default::default())
    }

    pub fn with_params(
        ctx: &mut Context,
        layout: VertexLayout,
        shader: Shader,
        params: PipelineParams,
    ) -> Pipeline {
        let program = ctx.shaders[shader.0].program;

        let attributes = layout
            .attributes
            .iter()
            .scan(0, |mut offset, (attribute, format)| {
                let gl_loc = match attribute {
                    VertexAttribute::Position => unimplemented!(),
                    VertexAttribute::Normal => unimplemented!(),
                    VertexAttribute::TexCoord0 => unimplemented!(),
                    VertexAttribute::Custom(name) => {
                        let cname = CString::new(*name).unwrap_or_else(|e| panic!(e));
                        let attrib =
                            unsafe { glGetAttribLocation(program, cname.as_ptr() as *const _) };
                        if attrib == -1 {
                            panic!();
                        }
                        attrib as u32
                    }
                };
                let attr = VertexAttributeInternal {
                    gl_loc,
                    size: format.size(),
                    offset: *offset,
                    stride: std::mem::size_of::<f32>() as i32 * layout.len,
                };

                *offset += (std::mem::size_of::<f32>() as i32 * format.size()) as i64;

                Some(attr)
            })
            .collect::<Vec<_>>();

        let pipeline = PipelineInternal {
            layout: attributes,
            layout_len: layout.len,
            shader,
            params,
        };
        ctx.pipelines.push(pipeline);
        Pipeline(ctx.pipelines.len() - 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct VertexAttributeInternal {
    gl_loc: GLuint,
    size: i32,
    offset: i64,
    stride: i32,
}

struct PipelineInternal {
    layout: Vec<VertexAttributeInternal>,
    layout_len: i32,
    shader: Shader,
    params: PipelineParams,
}

#[derive(Clone, Debug)]
pub struct Bindings {
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub images: Vec<Texture>,
}

#[derive(Clone, Copy, Debug)]
pub enum BufferType {
    VertexBuffer,
    IndexBuffer,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Usage {
    Immutable,
    Dynamic,
    Stream,
}

fn gl_buffer_target(buffer_type: &BufferType) -> GLenum {
    match buffer_type {
        BufferType::VertexBuffer => GL_ARRAY_BUFFER,
        BufferType::IndexBuffer => GL_ELEMENT_ARRAY_BUFFER,
    }
}

fn gl_usage(usage: &Usage) -> GLenum {
    match usage {
        Usage::Immutable => GL_STATIC_DRAW,
        Usage::Dynamic => GL_DYNAMIC_DRAW,
        Usage::Stream => GL_STREAM_DRAW,
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    gl_buf: GLuint,
}

impl Buffer {
    pub unsafe fn new<T>(
        ctx: &mut Context,
        buffer_type: BufferType,
        usage: Usage,
        data: &[T],
    ) -> Buffer {
        if usage != Usage::Immutable {
            unimplemented!();
        }
        //println!("{} {}", mem::size_of::<T>(), mem::size_of_val(data));
        let gl_target = gl_buffer_target(&buffer_type);
        let gl_usage = gl_usage(&usage);
        let size = mem::size_of_val(data) as i64;
        let mut gl_buf: u32 = 0;

        unsafe {
            glGenBuffers(1, &mut gl_buf as *mut _);
            ctx.store_buffer_binding(gl_target);
            ctx.bind_buffer(gl_target, gl_buf);
            glBufferData(gl_target, size as _, std::ptr::null() as *const _, gl_usage);
            if usage == Usage::Immutable {
                glBufferSubData(gl_target, 0, size as _, data.as_ptr() as *const _)
            }
            ctx.restore_buffer_binding(gl_target);
        }

        Buffer { gl_buf }
    }
}
