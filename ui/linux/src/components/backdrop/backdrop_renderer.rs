//! OpenGL port of macOS `ArtworkBackdropRenderer`: rotation (3 instances) → Gaussian blur → pinch mesh.
use super::backdrop_mesh::make_mesh;
use glow::HasContext;

const CLEAR: (f32, f32, f32) = (0.07, 0.07, 0.09);
/// ponytail: blur runs at long edge 320px (macOS: 960 + MPS). Output is a heavy blur, so the
/// lower res is invisible and keeps the shader loop short. Raise if banding shows on 4K.
const WORK_LONG_EDGE: f64 = 320.0;
const TRANSITION_SECS: f64 = 0.5;

const FULLSCREEN_VS: &str = r#"
out vec2 v_uv;
void main() {
    vec2 p = vec2(float((gl_VertexID << 1) & 2), float(gl_VertexID & 2));
    v_uv = p;
    gl_Position = vec4(p * 2.0 - 1.0, 0.0, 1.0);
}"#;

const SATURATE: &str = r#"
vec3 saturate_color(vec3 c, float a) {
    float l = dot(c, vec3(0.3, 0.59, 0.11));
    return l + (c - l) * a;
}"#;

const ROTATION_VS: &str = r#"
uniform vec2 u_view;
uniform float u_time;
uniform vec2 u_offset[3];
uniform float u_time_scale[3];
out vec2 v_uv;
const vec2 POS[6] = vec2[6](vec2(-1,-1), vec2(-1,1), vec2(1,1), vec2(-1,-1), vec2(1,1), vec2(1,-1));
const vec2 UV[6] = vec2[6](vec2(0,1), vec2(0,0), vec2(1,0), vec2(0,1), vec2(1,0), vec2(1,1));
void main() {
    float a = u_time * 6.2831853 / u_time_scale[gl_InstanceID];
    mat2 r = mat2(cos(a), -sin(a), sin(a), cos(a));
    vec2 p = r * (r * POS[gl_VertexID] + u_offset[gl_InstanceID]);
    gl_Position = vec4(p * u_view, 0.0, 1.0);
    v_uv = UV[gl_VertexID];
}"#;

const ROTATION_FS: &str = r#"
uniform sampler2D u_current;
uniform sampler2D u_previous;
uniform float u_mix;
in vec2 v_uv;
out vec4 o;
void main() {
    vec3 c = mix(texture(u_current, v_uv).rgb, texture(u_previous, v_uv).rgb, u_mix);
    o = vec4(saturate_color(c, 1.3), 1.0);
}"#;

// Separable Gaussian; outside texels count as black (MPS edgeMode .zero).
const BLUR_FS: &str = r#"
uniform sampler2D u_src;
uniform vec2 u_step;
uniform float u_sigma;
in vec2 v_uv;
out vec4 o;
void main() {
    int r = int(ceil(u_sigma * 3.0));
    vec3 sum = vec3(0.0);
    float total = 0.0;
    for (int i = -r; i <= r; i++) {
        float k = exp(-float(i * i) / (2.0 * u_sigma * u_sigma));
        vec2 uv = v_uv + u_step * float(i);
        float inside = step(0.0, uv.x) * step(uv.x, 1.0) * step(0.0, uv.y) * step(uv.y, 1.0);
        sum += texture(u_src, uv).rgb * k * inside;
        total += k;
    }
    o = vec4(sum / total, 1.0);
}"#;

const PINCH_VS: &str = r#"
in vec2 a_from;
in vec2 a_to;
in vec2 a_uv;
uniform float u_time;
out vec2 v_uv;
void main() {
    float phase = acos(sin(u_time * 3.14159265 / 1.75)) / 3.14159265;
    float blend = phase * phase * (3.0 - 2.0 * phase);
    gl_Position = vec4(mix(a_from, a_to, blend), 0.0, 1.0);
    vec2 uv = (a_uv - 0.5) * 0.8 + 0.5;
    v_uv = vec2(uv.x, 1.0 - uv.y); // Metal textures are top-down, GL render targets bottom-up
}"#;

// saturation 2, dark scrim 0.25, dark factor 0.2, floor 0.07, ceiling 0.97 (macOS uniforms).
// Dither: port of Windows `DitherNoise` (BackdropShaders.cs). Per-pixel temporal ±1 LSB, separate
// channel seeds, so the 8-bit output shows no contour steps in smooth dark gradients.
const PINCH_FS: &str = r#"
uniform sampler2D u_src;
uniform float u_time;
in vec2 v_uv;
out vec4 o;
float dither_noise(uvec2 p, uint frame, uint channel) {
    uint n = p.x * 1664525u + p.y * 1013904223u;
    n += frame * 747796405u + channel * 2891336453u;
    n ^= n >> 16;
    n *= 2246822519u;
    n ^= n >> 13;
    return float(n & 0xffffu) / 65535.0 * 2.0 - 1.0;
}
void main() {
    vec3 c = min(saturate_color(texture(u_src, v_uv).rgb, 2.0), vec3(0.995));
    c = mix(c, vec3(0.0), 0.25) - 0.02;
    c = clamp(c, vec3(0.07), vec3(0.97));
    uvec2 px = uvec2(gl_FragCoord.xy);
    uint f = uint(u_time * 60.0);
    c += vec3(dither_noise(px, f, 0u), dither_noise(px, f, 1u), dither_noise(px, f, 2u)) / 255.0;
    o = vec4(clamp(c, 0.0, 1.0), 1.0);
}"#;

struct Target {
    tex: glow::Texture,
    fbo: glow::Framebuffer,
}

pub struct BackdropRenderer {
    gl: glow::Context,
    rotation: glow::Program,
    blur: glow::Program,
    pinch: glow::Program,
    empty_vao: glow::VertexArray,
    mesh_vao: glow::VertexArray,
    mesh_buffers: [glow::Buffer; 2],
    index_count: i32,
    targets: Vec<Target>, // [rotation, blur horizontal, blur vertical]
    work_size: (i32, i32),
    current: glow::Texture,
    previous: Option<glow::Texture>,
    transition_start: Option<f64>,
}

impl BackdropRenderer {
    /// # Safety
    /// GL context of `gl` must be current.
    pub unsafe fn new(gl: glow::Context, es: bool) -> Result<Self, String> {
        let header = if es { "#version 300 es\nprecision highp float;\nprecision highp int;\n" } else { "#version 150 core\n" };
        let program = |vs: &str, fs: &str, attribs: &[&str]| -> Result<glow::Program, String> {
            let p = gl.create_program()?;
            let mut shaders = Vec::new();
            for (kind, src) in [(glow::VERTEX_SHADER, vs.to_string()), (glow::FRAGMENT_SHADER, format!("{SATURATE}{fs}"))] {
                let s = gl.create_shader(kind)?;
                gl.shader_source(s, &format!("{header}{src}"));
                gl.compile_shader(s);
                if !gl.get_shader_compile_status(s) {
                    return Err(gl.get_shader_info_log(s));
                }
                gl.attach_shader(p, s);
                shaders.push(s);
            }
            for (i, name) in attribs.iter().enumerate() {
                gl.bind_attrib_location(p, i as u32, name);
            }
            gl.link_program(p);
            for s in shaders {
                gl.detach_shader(p, s);
                gl.delete_shader(s);
            }
            if !gl.get_program_link_status(p) {
                return Err(gl.get_program_info_log(p));
            }
            Ok(p)
        };
        let rotation = program(ROTATION_VS, ROTATION_FS, &[])?;
        let blur = program(FULLSCREEN_VS, BLUR_FS, &[])?;
        let pinch = program(PINCH_VS, PINCH_FS, &["a_from", "a_to", "a_uv"])?;

        let empty_vao = gl.create_vertex_array()?;
        let mesh_vao = gl.create_vertex_array()?;
        let (vertices, indices) = make_mesh();
        gl.bind_vertex_array(Some(mesh_vao));
        let vbo = gl.create_buffer()?;
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        let bytes: Vec<u8> = vertices.iter().flatten().flat_map(|f| f.to_ne_bytes()).collect();
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &bytes, glow::STATIC_DRAW);
        for i in 0..3 {
            gl.enable_vertex_attrib_array(i);
            gl.vertex_attrib_pointer_f32(i, 2, glow::FLOAT, false, 24, i as i32 * 8);
        }
        let ibo = gl.create_buffer()?;
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ibo));
        let ibytes: Vec<u8> = indices.iter().flat_map(|i| i.to_ne_bytes()).collect();
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, &ibytes, glow::STATIC_DRAW);
        gl.bind_vertex_array(None);

        // Until the cover/blocks arrive: Music's placeholder grey rgb(51,51,54).
        let current = Self::upload(&gl, 1, 1, 4, &[51, 51, 54, 255])?;
        Ok(Self {
            gl,
            rotation,
            blur,
            pinch,
            empty_vao,
            mesh_vao,
            mesh_buffers: [vbo, ibo],
            index_count: indices.len() as i32,
            targets: Vec::new(),
            work_size: (0, 0),
            current,
            previous: None,
            transition_start: None,
        })
    }

    pub fn gl(&self) -> &glow::Context {
        &self.gl
    }

    unsafe fn upload(gl: &glow::Context, w: i32, h: i32, stride: i32, rgba: &[u8]) -> Result<glow::Texture, String> {
        Self::texture(gl, w, h, stride, glow::RGBA8, glow::UNSIGNED_BYTE, Some(rgba))
    }

    unsafe fn texture(
        gl: &glow::Context, w: i32, h: i32, stride: i32, internal: u32, ty: u32, data: Option<&[u8]>,
    ) -> Result<glow::Texture, String> {
        let tex = gl.create_texture()?;
        gl.bind_texture(glow::TEXTURE_2D, Some(tex));
        for (k, v) in [
            (glow::TEXTURE_MIN_FILTER, glow::LINEAR),
            (glow::TEXTURE_MAG_FILTER, glow::LINEAR),
            (glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE),
            (glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE),
        ] {
            gl.tex_parameter_i32(glow::TEXTURE_2D, k, v as i32);
        }
        gl.pixel_store_i32(glow::UNPACK_ROW_LENGTH, stride / 4);
        gl.tex_image_2d(
            glow::TEXTURE_2D, 0, internal as i32, w, h, 0,
            glow::RGBA, ty, glow::PixelUnpackData::Slice(data),
        );
        gl.pixel_store_i32(glow::UNPACK_ROW_LENGTH, 0);
        Ok(tex)
    }

    /// New artwork crossfades in over 0.5s. `rgba` is top row first.
    pub unsafe fn set_artwork(&mut self, w: i32, h: i32, stride: i32, rgba: &[u8], clock: f64) {
        let Ok(tex) = Self::upload(&self.gl, w, h, stride, rgba) else { return };
        if let Some(old) = self.previous.take() {
            self.gl.delete_texture(old);
        }
        self.previous = Some(std::mem::replace(&mut self.current, tex));
        self.transition_start = Some(clock);
    }

    unsafe fn resize(&mut self, width: i32, height: i32) {
        let scale = (WORK_LONG_EDGE / width.max(height) as f64).min(1.0);
        let size = (((width as f64 * scale) as i32).max(1), ((height as f64 * scale) as i32).max(1));
        if size == self.work_size {
            return;
        }
        self.drop_targets();
        for _ in 0..3 {
            // Half-float like Windows (R16G16B16A16_Float): blur keeps precision, quantize once at output.
            let Ok(tex) = Self::texture(&self.gl, size.0, size.1, 0, glow::RGBA16F, glow::HALF_FLOAT, None) else { return };
            let Ok(fbo) = self.gl.create_framebuffer() else { return };
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, Some(fbo));
            self.gl.framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, Some(tex), 0);
            self.targets.push(Target { tex, fbo });
        }
        self.work_size = size;
    }

    /// Draws one frame into the currently bound framebuffer (`out` = its size in px).
    /// `time` = animation seconds, `clock` = wall seconds (transitions).
    pub unsafe fn render(&mut self, out_fbo: Option<glow::Framebuffer>, out: (i32, i32), time: f32, clock: f64) {
        if out.0 <= 0 || out.1 <= 0 {
            return;
        }
        self.resize(out.0, out.1);
        if self.targets.len() != 3 {
            return;
        }
        let gl = &self.gl;
        if self.transition_start.is_some_and(|s| clock - s >= TRANSITION_SECS) {
            if let Some(old) = self.previous.take() {
                gl.delete_texture(old);
            }
            self.transition_start = None;
        }
        let mix = self.transition_start.map_or(0.0, |s| (1.0 - (clock - s) / TRANSITION_SECS).max(0.0) as f32);
        let (ww, wh) = self.work_size;
        gl.disable(glow::BLEND);
        gl.disable(glow::DEPTH_TEST);
        gl.disable(glow::SCISSOR_TEST);
        gl.clear_color(CLEAR.0, CLEAR.1, CLEAR.2, 1.0);
        gl.active_texture(glow::TEXTURE0);

        // 1. Rotation: three rotating copies of the artwork.
        gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.targets[0].fbo));
        gl.viewport(0, 0, ww, wh);
        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.use_program(Some(self.rotation));
        let aspect = out.0 as f32 / out.1 as f32;
        let u = |p, name: &str| gl.get_uniform_location(p, name);
        gl.uniform_2_f32(u(self.rotation, "u_view").as_ref(), (1.0 / aspect).max(1.0), aspect.max(1.0));
        gl.uniform_1_f32(u(self.rotation, "u_time").as_ref(), time);
        gl.uniform_2_f32_slice(u(self.rotation, "u_offset").as_ref(), &[0.0, 0.0, -0.5, 0.7, -0.95, -0.7]);
        gl.uniform_1_f32_slice(u(self.rotation, "u_time_scale").as_ref(), &[60.0, 45.0, 35.0]);
        gl.uniform_1_f32(u(self.rotation, "u_mix").as_ref(), mix);
        gl.uniform_1_i32(u(self.rotation, "u_current").as_ref(), 0);
        gl.uniform_1_i32(u(self.rotation, "u_previous").as_ref(), 1);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.current));
        gl.active_texture(glow::TEXTURE1);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.previous.unwrap_or(self.current)));
        gl.active_texture(glow::TEXTURE0);
        gl.bind_vertex_array(Some(self.empty_vao));
        gl.draw_arrays_instanced(glow::TRIANGLES, 0, 6, 3);

        // 2. Blur: sigma from macOS, scaled to the work texture.
        let scale = ww as f64 / out.0 as f64;
        let sigma = (((out.0 as f64).hypot(out.1 as f64) * 0.045394707).floor() * scale).max(1.0) as f32;
        gl.use_program(Some(self.blur));
        gl.uniform_1_f32(u(self.blur, "u_sigma").as_ref(), sigma);
        gl.uniform_1_i32(u(self.blur, "u_src").as_ref(), 0);
        for (src, dst, step) in [(0, 1, (1.0 / ww as f32, 0.0)), (1, 2, (0.0, 1.0 / wh as f32))] {
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.targets[dst].fbo));
            gl.uniform_2_f32(u(self.blur, "u_step").as_ref(), step.0, step.1);
            gl.bind_texture(glow::TEXTURE_2D, Some(self.targets[src].tex));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }

        // 3. Pinch mesh into the widget.
        gl.bind_framebuffer(glow::FRAMEBUFFER, out_fbo);
        gl.viewport(0, 0, out.0, out.1);
        gl.clear(glow::COLOR_BUFFER_BIT);
        gl.use_program(Some(self.pinch));
        gl.uniform_1_f32(u(self.pinch, "u_time").as_ref(), time);
        gl.uniform_1_i32(u(self.pinch, "u_src").as_ref(), 0);
        gl.bind_texture(glow::TEXTURE_2D, Some(self.targets[2].tex));
        gl.bind_vertex_array(Some(self.mesh_vao));
        gl.draw_elements(glow::TRIANGLES, self.index_count, glow::UNSIGNED_INT, 0);
        gl.bind_vertex_array(None);
    }

    unsafe fn drop_targets(&mut self) {
        for t in self.targets.drain(..) {
            self.gl.delete_framebuffer(t.fbo);
            self.gl.delete_texture(t.tex);
        }
        self.work_size = (0, 0);
    }

    /// # Safety
    /// GL context must be current.
    pub unsafe fn destroy(mut self) {
        self.drop_targets();
        let gl = &self.gl;
        for p in [self.rotation, self.blur, self.pinch] {
            gl.delete_program(p);
        }
        gl.delete_vertex_array(self.empty_vao);
        gl.delete_vertex_array(self.mesh_vao);
        for b in self.mesh_buffers {
            gl.delete_buffer(b);
        }
        gl.delete_texture(self.current);
        if let Some(p) = self.previous {
            gl.delete_texture(p);
        }
    }
}
