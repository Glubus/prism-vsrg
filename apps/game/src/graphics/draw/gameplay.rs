//! Gameplay rendering - notes, receptors, playfield.
//!
//! (Unused/Deprecated)

// use std::sync::Arc;
// use wgpu::RenderPass;

// use crate::graphics::Pipelines;
// use crate::graphics::assets::SkinAssets;
// use crate::graphics::primitives::InstanceRaw;
// use crate::ui::gameplay::playfield::{NoteInstancesByType, Playfield};

// /// Buffer allocations for gameplay rendering.
// pub struct GameplayBuffers {
//     pub instance_buffer: wgpu::Buffer,
//     pub receptor_buffer: wgpu::Buffer,
// }

// impl GameplayBuffers {
//     pub fn new(device: &wgpu::Device) -> Self {
//         use wgpu::util::DeviceExt;

//         let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Gameplay Instance Buffer"),
//             contents: bytemuck::cast_slice(
//                 &[InstanceRaw {
//                     offset: [0.0, 0.0],
//                     scale: [0.0, 0.0],
//                 }; 4096],
//             ),
//             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//         });

//         let receptor_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
//             label: Some("Receptor Instance Buffer"),
//             contents: bytemuck::cast_slice(
//                 &[InstanceRaw {
//                     offset: [0.0, 0.0],
//                     scale: [0.0, 0.0],
//                 }; 32],
//             ),
//             usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
//         });

//         Self {
//             instance_buffer,
//             receptor_buffer,
//         }
//     }
// }

// /// Draw the gameplay (receptors and notes).
// pub fn draw_gameplay<'a>(
//     render_pass: &mut RenderPass<'a>,
//     pipelines: &'a Pipelines,
//     buffers: &'a GameplayBuffers,
//     queue: &wgpu::Queue,
//     playfield: &'a Playfield,
//     assets: &'a SkinAssets,
//     keys_held: &[bool],
// ) {
//     // Set up sprite pipeline
//     render_pass.set_pipeline(&pipelines.sprite);

//     // Draw receptors
//     draw_receptors(render_pass, buffers, queue, playfield, keys_held);

//     // Collect note instances
//     let note_instances = playfield.collect_instances();

//     // Draw tap notes by column
//     draw_tap_notes(render_pass, buffers, queue, playfield, &note_instances);

//     // Draw special notes (mines, holds, bursts)
//     draw_special_notes(render_pass, buffers, queue, assets, &note_instances);
// }

// fn draw_receptors<'a>(
//     render_pass: &mut RenderPass<'a>,
//     buffers: &'a GameplayBuffers,
//     queue: &wgpu::Queue,
//     playfield: &'a Playfield,
//     keys_held: &[bool],
// ) {
//     let receptors = playfield.receptor_instances();
//     if receptors.is_empty() {
//         return;
//     }

//     queue.write_buffer(
//         &buffers.receptor_buffer,
//         0,
//         bytemuck::cast_slice(&receptors),
//     );

//     for (i, col) in playfield.columns().iter().enumerate() {
//         let is_pressed = keys_held.get(i).copied().unwrap_or(false);
//         render_pass.set_bind_group(0, col.receptor_bind_group(is_pressed), &[]);
//         render_pass.set_vertex_buffer(0, buffers.receptor_buffer.slice(..));
//         let offset = i as u32;
//         render_pass.draw(0..6, offset..offset + 1);
//     }
// }

// fn draw_tap_notes<'a>(
//     render_pass: &mut RenderPass<'a>,
//     buffers: &'a GameplayBuffers,
//     queue: &wgpu::Queue,
//     playfield: &'a Playfield,
//     instances: &NoteInstancesByType,
// ) {
//     if instances.taps.is_empty() {
//         return;
//     }

//     // Group by column for efficient batch rendering
//     for col in playfield.columns() {
//         let col_taps: Vec<_> = instances
//             .taps
//             .iter()
//             .filter(|(c, _)| *c == col.index)
//             .map(|(_, inst)| *inst)
//             .collect();

//         if col_taps.is_empty() {
//             continue;
//         }

//         queue.write_buffer(&buffers.instance_buffer, 0, bytemuck::cast_slice(&col_taps));

//         render_pass.set_bind_group(0, col.note_bind_group(), &[]);
//         render_pass.set_vertex_buffer(0, buffers.instance_buffer.slice(..));
//         render_pass.draw(0..6, 0..col_taps.len() as u32);
//     }
// }

// fn draw_special_notes<'a>(
//     render_pass: &mut RenderPass<'a>,
//     buffers: &'a GameplayBuffers,
//     queue: &wgpu::Queue,
//     assets: &'a SkinAssets,
//     instances: &NoteInstancesByType,
// ) {
//     let Some(mode) = assets.current_mode() else {
//         return;
//     };

//     // Draw mines
//     if let Some(mine_bg) = &mode.mine {
//         draw_batch(render_pass, buffers, queue, mine_bg, &instances.mines);
//     }

//     // Draw hold bodies
//     if let Some(hold_body_bg) = &mode.hold_body {
//         draw_batch(
//             render_pass,
//             buffers,
//             queue,
//             hold_body_bg,
//             &instances.hold_bodies,
//         );
//     }

//     // Draw hold ends
//     if let Some(hold_end_bg) = &mode.hold_end {
//         draw_batch(
//             render_pass,
//             buffers,
//             queue,
//             hold_end_bg,
//             &instances.hold_ends,
//         );
//     }

//     // Draw burst bodies
//     if let Some(burst_body_bg) = &mode.burst_body {
//         draw_batch(
//             render_pass,
//             buffers,
//             queue,
//             burst_body_bg,
//             &instances.burst_bodies,
//         );
//     }

//     // Draw burst ends
//     if let Some(burst_end_bg) = &mode.burst_end {
//         draw_batch(
//             render_pass,
//             buffers,
//             queue,
//             burst_end_bg,
//             &instances.burst_ends,
//         );
//     }
// }

// fn draw_batch<'a>(
//     render_pass: &mut RenderPass<'a>,
//     buffers: &'a GameplayBuffers,
//     queue: &wgpu::Queue,
//     bind_group: &'a Arc<wgpu::BindGroup>,
//     instances: &[InstanceRaw],
// ) {
//     if instances.is_empty() {
//         return;
//     }

//     queue.write_buffer(&buffers.instance_buffer, 0, bytemuck::cast_slice(instances));
//     render_pass.set_bind_group(0, bind_group.as_ref(), &[]);
//     render_pass.set_vertex_buffer(0, buffers.instance_buffer.slice(..));
//     render_pass.draw(0..6, 0..instances.len() as u32);
// }
