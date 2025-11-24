use crate::models::menu::MenuState;
use crate::views::components::{menu::LeaderboardDisplay, SongSelectionDisplay};
use crate::views::context::MenuRenderContext;
use std::sync::{Arc, Mutex};
use wgpu::SurfaceError;

pub struct MenuView {
    song_menu: SongSelectionDisplay,
    leaderboard: LeaderboardDisplay,
}

impl MenuView {
    pub fn new() -> Self {
        Self {
            song_menu: SongSelectionDisplay::new(1280.0, 720.0),
            leaderboard: LeaderboardDisplay::new(1280.0, 720.0),
        }
    }

    pub fn update_leaderboard(&mut self, replays: Vec<crate::database::models::Replay>) {
        self.leaderboard.update_scores(replays);
    }

    pub fn render(
        &mut self,
        ctx: &mut MenuRenderContext<'_>,
        menu_state: &Arc<Mutex<MenuState>>,
    ) -> Result<(), SurfaceError> {
        let mut encoder = ctx
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor::default());

        if let (Some(pipeline), Some(bind_group)) =
            (ctx.background_pipeline, ctx.background_bind_group)
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Background Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: ctx.menu_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            render_pass.set_pipeline(pipeline);
            render_pass.set_bind_group(0, bind_group, &[]);
            render_pass.draw(0..6, 0..1);
        } else {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Menu Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: ctx.menu_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        }

        ctx.queue.submit(std::iter::once(encoder.finish()));

        self.song_menu
            .update_size(ctx.screen_width, ctx.screen_height);
        self.song_menu.update(menu_state);

        self.leaderboard
            .update_size(ctx.screen_width, ctx.screen_height);

        // Rendre le leaderboard à gauche
        self.leaderboard.render(
            ctx.device,
            ctx.queue,
            ctx.text_brush,
            ctx.menu_view,
            ctx.quad_pipeline,
            ctx.quad_buffer,
        )?;

        // Rendre le menu de sélection de chansons
        self.song_menu.render(
            ctx.device,
            ctx.queue,
            ctx.text_brush,
            ctx.menu_view,
            ctx.quad_pipeline,
            ctx.quad_buffer,
            ctx.fps,
            menu_state,
        )?;

        Ok(())
    }
}
