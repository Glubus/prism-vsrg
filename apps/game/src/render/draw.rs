//! Main draw dispatcher - routes to appropriate draw functions.
//!
//! This file has been refactored to use the new graphics/draw/ architecture.

use crate::render::context::RenderContext;
use crate::render::resources::RenderResources;
use crate::shared::snapshot::{GameplaySnapshot, RenderState};
use crate::views::context::GameplayRenderContext;
use wgpu::{Color, CommandEncoder, LoadOp, Operations, RenderPassDescriptor, TextureView};

/// Main entry point for all rendering based on game state.
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
            clear_screen(encoder, view, "Gameplay Clear");
            draw_gameplay_v2(ctx, res, encoder, view, snapshot, fps);
        }
        RenderState::Editor(snapshot) => {
            clear_screen(encoder, view, "Editor Clear");
            draw_gameplay_v2(ctx, res, encoder, view, &snapshot.game, fps);
        }
        RenderState::Menu(_) => {
            draw_background_pass(ctx, res, encoder, view);
        }
        RenderState::Result(_) => {
            draw_background_pass(ctx, res, encoder, view);
        }
        RenderState::MainMenu => {
            draw_background_pass(ctx, res, encoder, view);
        }
        RenderState::Empty => {
            clear_screen(encoder, view, "Empty Clear");
        }
    }
}

/// Clear the screen to black.
fn clear_screen(encoder: &mut CommandEncoder, view: &TextureView, label: &str) {
    encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Clear(Color::BLACK),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });
}

/// Draw background with the new architecture.
fn draw_background_pass(
    _ctx: &RenderContext,
    res: &RenderResources,
    encoder: &mut CommandEncoder,
    view: &TextureView,
) {
    if let Some(bg_group) = &res.background_bind_group {
        let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
            label: Some("Background Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view,
                resolve_target: None,
                ops: Operations {
                    load: LoadOp::Clear(Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&res.background_pipeline);
        pass.set_bind_group(0, bg_group, &[]);
        pass.draw(0..6, 0..1);
    } else {
        clear_screen(encoder, view, "Clear (No BG)");
    }
}

/// Draw gameplay using the new v2 architecture (hybrid mode).
/// Uses new SkinAssets + Playfield for notes/receptors,
/// but still uses old HUD system for compatibility.
fn draw_gameplay_v2(
    ctx: &RenderContext,
    res: &mut RenderResources,
    encoder: &mut CommandEncoder,
    view: &TextureView,
    snapshot: &GameplaySnapshot,
    fps: f64,
) {
    // Try new system first if skin_assets is loaded
    let use_new_system = res.skin_assets.is_some();

    if use_new_system {
        // NEW: Use graphics/draw/gameplay.rs
        draw_gameplay_new(ctx, res, encoder, view, snapshot);
    }

    // Always draw HUD with old system for now (text, score, etc.)
    draw_hud_legacy(ctx, res, encoder, view, snapshot, fps);
}

/// Draw notes and receptors using new Playfield/SkinAssets system.
fn draw_gameplay_new(
    ctx: &RenderContext,
    res: &mut RenderResources,
    encoder: &mut CommandEncoder,
    view: &TextureView,
    snapshot: &GameplaySnapshot,
) {
    // Extract keys held state from snapshot
    let keys_held: Vec<bool> = snapshot.keys_held.iter().copied().collect();

    // Update playfield with visible notes
    res.playfield.render_notes(
        &snapshot.visible_notes,
        snapshot.audio_time,
        snapshot.scroll_speed,
    );

    // Create render pass for notes
    let mut pass = encoder.begin_render_pass(&RenderPassDescriptor {
        label: Some("Gameplay Pass (v2)"),
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: Operations {
                load: LoadOp::Load, // Don't clear, preserve background
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    });

    // Create temporary pipelines reference (we need to add pipelines to resources)
    // For now, use render_pipeline as sprite pipeline
    if let Some(ref assets) = res.skin_assets {
        // TODO: Once Pipelines struct is in RenderResources, use it here
        // gfx_draw::draw_gameplay(&mut pass, &pipelines, &res.gameplay_buffers, &ctx.queue, &res.playfield, assets, &keys_held);

        // For now, use legacy rendering via the old bind groups
    }
}

/// Draw HUD using legacy system (score, combo, accuracy, text).
fn draw_hud_legacy(
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
        progress_pipeline: &res.progress_pipeline,
        instance_buffer: &res.instance_buffer,
        receptor_buffer: &res.receptor_buffer,
        progress_buffer: &res.progress_buffer,
        note_bind_groups: &res.note_bind_groups,
        receptor_bind_groups: &res.receptor_bind_groups,
        receptor_pressed_bind_groups: &res.receptor_pressed_bind_groups,
        mine_bind_group: res.mine_bind_group.as_ref(),
        hold_body_bind_group: res.hold_body_bind_group.as_ref(),
        hold_end_bind_group: res.hold_end_bind_group.as_ref(),
        burst_body_bind_group: res.burst_body_bind_group.as_ref(),
        burst_end_bind_group: res.burst_end_bind_group.as_ref(),
        view,
        pixel_system: &res.pixel_system,
        screen_width: ctx.config.width as f32,
        screen_height: ctx.config.height as f32,
        fps,
        master_volume: 1.0,
    };

    // Get colors from skin structure
    let judgement = &res.skin.hud.judgement;
    let colors = engine::JudgementColors {
        marv: judgement.marv.color,
        perfect: judgement.perfect.color,
        great: judgement.great.color,
        good: judgement.good.color,
        bad: judgement.bad.color,
        miss: judgement.miss.color,
        ghost_tap: judgement.ghost_tap.color,
    };

    let labels = res.skin.get_judgement_labels();

    // Use legacy gameplay_view for full rendering (including notes for now)
    let _ = res.gameplay_view.render(
        &mut view_ctx,
        encoder,
        snapshot,
        &mut res.score_display,
        &mut res.accuracy_panel,
        &mut res.judgements_panel,
        &mut res.combo_display,
        &mut res.judgement_flash,
        &mut res.hit_bar,
        &mut res.nps_display,
        &mut res.notes_remaining_display,
        &mut res.scroll_speed_display,
        &mut res.time_left_display,
        &colors,
        &labels,
    );
}
