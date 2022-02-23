use egui::{ClippedMesh, CtxRef};
use egui_wgpu_backend::{BackendError, RenderPass, ScreenDescriptor};
use pixels::{wgpu, PixelsContext};
use winit::window::Window;

/// Manages all state required for rendering egui over `Pixels`.
pub(crate) struct Framework {
    // State for egui.
    egui_ctx: CtxRef,
    egui_state: egui_winit::State,
    screen_descriptor: ScreenDescriptor,
    rpass: RenderPass,
    paint_jobs: Vec<ClippedMesh>,
}

impl Framework {
    /// Create egui.
    pub(crate) fn new(width: u32, height: u32, scale_factor: f32, pixels: &pixels::Pixels) -> Self {
        let egui_ctx = CtxRef::default();
        let egui_state = egui_winit::State::from_pixels_per_point(scale_factor);
        let screen_descriptor = ScreenDescriptor {
            physical_width: width,
            physical_height: height,
            scale_factor,
        };
        let rpass = RenderPass::new(pixels.device(), pixels.render_texture_format(), 1);

        Self {
            egui_ctx,
            egui_state,
            screen_descriptor,
            rpass,
            paint_jobs: Vec::new(),
        }
    }

    /// Handle input events from the window manager.
    pub(crate) fn handle_event(&mut self, event: &winit::event::WindowEvent) -> bool {
        self.egui_state.on_event(&self.egui_ctx, event)
    }

    pub(crate) fn wants_keyboard_input(&self) -> bool {
        self.egui_ctx.wants_keyboard_input()
    }
    pub(crate) fn wants_pointer_input(&self) -> bool {
        self.egui_ctx.wants_pointer_input()
    }

    /// Resize egui.
    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.screen_descriptor.physical_width = width;
            self.screen_descriptor.physical_height = height;
        }
    }

    /// Update scaling factor.
    pub(crate) fn scale_factor(&mut self, scale_factor: f32) {
        self.screen_descriptor.scale_factor = scale_factor;
    }

    /// Prepare egui.
    pub(crate) fn prepare(&mut self, window: &Window, run_ui: impl FnOnce(&CtxRef)) {
        // Run the egui frame and create all paint jobs to prepare for rendering.
        let raw_input = self.egui_state.take_egui_input(window);
        let (output, paint_commands) = self.egui_ctx.run(raw_input, run_ui);

        self.egui_state
            .handle_output(window, &self.egui_ctx, output);
        self.paint_jobs = self.egui_ctx.tessellate(paint_commands);
    }

    /// Render egui.
    pub(crate) fn render(
        &mut self,
        encoder: &mut wgpu::CommandEncoder,
        render_target: &wgpu::TextureView,
        context: &PixelsContext,
    ) -> Result<(), BackendError> {
        // Upload all resources to the GPU.
        self.rpass
            .update_texture(&context.device, &context.queue, &self.egui_ctx.font_image());
        self.rpass
            .update_user_textures(&context.device, &context.queue);
        self.rpass.update_buffers(
            &context.device,
            &context.queue,
            &self.paint_jobs,
            &self.screen_descriptor,
        );

        // Record all render passes.
        self.rpass.execute(
            encoder,
            render_target,
            &self.paint_jobs,
            &self.screen_descriptor,
            None,
        )
    }
}
