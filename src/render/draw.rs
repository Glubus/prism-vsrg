use wgpu::{CommandEncoder, TextureView, RenderPassDescriptor, LoadOp, Operations, Color};
use crate::shared::snapshot::{RenderState, GameplaySnapshot};
use crate::render::context::RenderContext;
use crate::render::resources::RenderResources;
use crate::views::context::GameplayRenderContext;

pub fn draw_game(
    ctx: &RenderContext,
    res: &mut RenderResources,
    encoder: &mut CommandEncoder,
    view: &TextureView,
    state: &RenderState,
    fps: f64,
) {
    match state {
        RenderState::InGame(snapshot) => {
            encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Gameplay Clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view, 
                    resolve_target: None,
                    ops: Operations { load: LoadOp::Clear(Color::BLACK), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            draw_gameplay(ctx, res, encoder, view, snapshot, fps);
        },
        // CORRECTION : Gestion de l'Editor (comme InGame pour le fond)
        RenderState::Editor(snapshot) => {
             encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Editor Clear"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view, 
                    resolve_target: None,
                    ops: Operations { load: LoadOp::Clear(Color::BLACK), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
            // On dessine le jeu figé en fond
            draw_gameplay(ctx, res, encoder, view, &snapshot.game, fps);
        },
        RenderState::Menu(_) => {
            draw_background(ctx, res, encoder, view);
        },
        RenderState::Result(_) => {
            draw_background(ctx, res, encoder, view);
        },
        RenderState::Empty => {
            encoder.begin_render_pass(&RenderPassDescriptor {
                label: Some("Clear Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view, 
                    resolve_target: None,
                    ops: Operations { load: LoadOp::Clear(Color::BLACK), store: wgpu::StoreOp::Store },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
            });
        }
    }
}

fn draw_background(_ctx: &RenderContext, res: &RenderResources, encoder: &mut CommandEncoder, view: &TextureView) {
    if let Some(bg_group) = &res.background_bind_group {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Background Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view, resolve_target: None,
                ops: Operations { load: LoadOp::Clear(Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
        });
        pass.set_pipeline(&res.background_pipeline);
        pass.set_bind_group(0, bg_group, &[]);
        pass.draw(0..6, 0..1);
    } else {
        encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Clear Pass (No BG)"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view, resolve_target: None,
                ops: Operations { load: LoadOp::Clear(Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
        });
    }
}

// CORRECTION : pub fn pour être accessible depuis renderer.rs
pub fn draw_gameplay(
    ctx: &RenderContext,
    res: &mut RenderResources,
    encoder: &mut CommandEncoder,
    view: &TextureView,
    snapshot: &GameplaySnapshot,
    fps: f64,
) {
    let mut view_ctx = GameplayRenderContext {
        device: &ctx.device, 
        queue: &ctx.queue, 
        text_brush: &mut res.text_brush,
        render_pipeline: &res.render_pipeline, 
        instance_buffer: &res.instance_buffer,
        receptor_buffer: &res.receptor_buffer, 
        note_bind_groups: &res.note_bind_groups, 
        receptor_bind_groups: &res.receptor_bind_groups,
        receptor_pressed_bind_groups: &res.receptor_pressed_bind_groups,
        view, 
        pixel_system: &res.pixel_system,
        screen_width: ctx.config.width as f32, 
        screen_height: ctx.config.height as f32,
        fps, 
        master_volume: 1.0,
    };

    let _ = res.gameplay_view.render(
        &mut view_ctx, 
        encoder,
        snapshot,
        &mut res.score_display, 
        &mut res.accuracy_panel, 
        &mut res.judgements_panel, 
        &mut res.combo_display, 
        &mut res.judgement_flash, 
        &mut res.hit_bar
    );
}