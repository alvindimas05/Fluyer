// WGPU Renderer module for rendering behind webview

use std::borrow::Cow;
use std::sync::Mutex;
use tauri::{async_runtime::block_on, Manager};
use wgpu::{Backend, BackendOptions, Backends, InstanceDescriptor, InstanceFlags};

/// Initialize WGPU rendering for a window
pub fn setup_wgpu(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let window = app.get_webview_window("main").unwrap();
    let size = window.inner_size()?;

    let instance = wgpu::Instance::new(&InstanceDescriptor {
        backends: Backends::DX12,
        flags: InstanceFlags::default(),
        backend_options: BackendOptions::default(),
    });

    let surface = instance.create_surface(window.clone()).unwrap();
    let adapter = block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::default(),
        force_fallback_adapter: false,
        // Request an adapter which can render to our surface
        compatible_surface: Some(&surface),
    }))
    .expect("Failed to find an appropriate adapter");

    // Create the logical device and command queue
    let (device, queue) = block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            // Make sure we use the texture resolution limits from the adapter, so we can support images the size of the swapchain.
            required_limits:
                wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: wgpu::MemoryHints::default(),
        },
        None,
    ))
    .expect("Failed to create device");

    // Load the shaders from disk
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
            r#"
@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> @builtin(position) vec4<f32> {
    let x = f32(i32(in_vertex_index) - 1);
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1);
    return vec4<f32>(x, y, 0.0, 1.0);
}

@fragment
fn fs_main() -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}
"#,
        )),
    });

    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let swapchain_format = swapchain_capabilities.formats[0];

    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: None,
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            buffers: &[],
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            targets: &[Some(swapchain_format.into())],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        multiview: None,
        cache: None,
    });

    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
        desired_maximum_frame_latency: 2,
    };

    surface.configure(&device, &config);

    app.manage(WgpuSurface(surface));
    app.manage(render_pipeline);
    app.manage(device);
    app.manage(queue);
    app.manage(Mutex::new(config));

    Ok(())
}

/// Wrapper for wgpu::Surface to make it Send + Sync
pub struct WgpuSurface<'a>(pub wgpu::Surface<'a>);

// SAFETY: The surface is only accessed from the main thread in Tauri
unsafe impl Send for WgpuSurface<'_> {}
unsafe impl Sync for WgpuSurface<'_> {}

/// Handle window resize for WGPU
pub fn handle_wgpu_resize(app_handle: &tauri::AppHandle, width: u32, height: u32) {
    let config = app_handle.state::<Mutex<wgpu::SurfaceConfiguration>>();
    let surface = app_handle.state::<WgpuSurface>();
    let device = app_handle.state::<wgpu::Device>();

    let mut config = config.lock().unwrap();
    config.width = if width > 0 { width } else { 1 };
    config.height = if height > 0 { height } else { 1 };
    surface.0.configure(&device, &config);
}

/// Render a frame using WGPU
pub fn render_frame(app_handle: &tauri::AppHandle) {
    let surface = app_handle.state::<WgpuSurface>();
    let render_pipeline = app_handle.state::<wgpu::RenderPipeline>();
    let device = app_handle.state::<wgpu::Device>();
    let queue = app_handle.state::<wgpu::Queue>();

    let frame = match surface.0.get_current_texture() {
        Ok(frame) => frame,
        Err(e) => {
            crate::debug!("Failed to acquire next swap chain texture: {:?}", e);
            return;
        }
    };

    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        rpass.set_pipeline(&render_pipeline);
        rpass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
}
