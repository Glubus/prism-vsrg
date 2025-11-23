use wgpu::{Device, Queue, SurfaceError, TextureView, RenderPipeline, Buffer, BindGroup};
use wgpu_text::{TextBrush, glyph_brush::Section};
use crate::engine::{GameEngine, InstanceRaw};
use crate::components::{Component, ScoreComponent, AccuracyComponent, JudgementsComponent, ComboComponent, JudgementComponent, HitBar, PlayfieldComponent};
use crate::playfield::Playfield;
use crate::engine::PixelSystem;

/// Rend le gameplay (notes, receptors, UI)
pub fn render_gameplay(
    device: &Device,
    queue: &Queue,
    text_brush: &mut TextBrush,
    render_pipeline: &RenderPipeline,
    instance_buffer: &Buffer,
    receptor_buffer: &Buffer,
    note_bind_groups: &[BindGroup],
    receptor_bind_groups: &[BindGroup],
    engine: &mut GameEngine,
    playfield_component: &PlayfieldComponent,
    playfield: &Playfield,
    pixel_system: &PixelSystem,
    score_component: &mut ScoreComponent,
    accuracy_component: &mut AccuracyComponent,
    judgements_component: &mut JudgementsComponent,
    combo_component: &mut ComboComponent,
    judgement_component: &mut JudgementComponent,
    hit_bar: &mut HitBar,
    screen_width: f32,
    screen_height: f32,
    fps: f64,
    view: &TextureView,
) -> Result<(), SurfaceError> {
    // Mettre à jour les notes actives et détecter les misses
    engine.update_active_notes();
    engine.detect_misses();

    // Démarre l'audio si game_time >= 0
    engine.start_audio_if_needed();
    
    // Calculer le game_time (commence à -5000ms)
    let song_time = engine.get_game_time();
    let max_future_time = song_time + engine.scroll_speed_ms;
    let min_past_time = song_time - 200.0;

    // Avancer le curseur et collecter les notes visibles
    while engine.head_index < engine.chart.len() {
        if engine.chart[engine.head_index].timestamp_ms < min_past_time {
            engine.head_index += 1;
            engine.notes_passed += 1;
        } else {
            break;
        }
    }

    let visible_notes: Vec<_> = engine.chart
        .iter()
        .skip(engine.head_index)
        .take_while(|note| note.timestamp_ms <= max_future_time)
        .cloned()
        .collect();

    // Utiliser le playfield component pour rendre les notes
    let instances_with_columns = playfield_component.render_notes(
        &visible_notes,
        song_time,
        engine.scroll_speed_ms,
        pixel_system,
    );
    
    // Grouper les instances par colonne
    let mut instances_by_column: Vec<Vec<InstanceRaw>> = vec![Vec::new(); crate::engine::NUM_COLUMNS];
    for (column, instance) in instances_with_columns {
        if column < instances_by_column.len() {
            instances_by_column[column].push(instance);
        }
    }
    
    // Calculer les offsets pour chaque colonne
    let mut column_offsets: Vec<u64> = Vec::new();
    let mut total_instances = 0u64;
    for col_instances in &instances_by_column {
        column_offsets.push(total_instances);
        total_instances += col_instances.len() as u64;
    }
    
    // Écrire toutes les instances dans le buffer
    let mut all_instances: Vec<InstanceRaw> = Vec::new();
    for col_instances in &instances_by_column {
        all_instances.extend(col_instances.iter().cloned());
    }
    
    if !all_instances.is_empty() {
        queue.write_buffer(instance_buffer, 0, bytemuck::cast_slice(&all_instances));
    }
    
    use bytemuck;
    
    // Préparer toutes les sections de texte avec les components
    let mut text_sections = Vec::new();
    
    // FPS en haut à droite
    let fps_text = format!("FPS: {:.0}", fps);
    text_sections.push(Section {
        screen_position: (screen_width - 100.0, 20.0),
        bounds: (screen_width, screen_height),
        text: vec![
            wgpu_text::glyph_brush::Text::new(&fps_text)
                .with_scale(24.0)
                .with_color([1.0, 1.0, 1.0, 1.0]),
        ],
        ..Default::default()
    });
    
    // Utiliser les components pour le rendu
    text_sections.extend(score_component.render(engine, pixel_system, screen_width, screen_height));
    text_sections.extend(accuracy_component.render(engine, pixel_system, screen_width, screen_height));
    text_sections.extend(judgements_component.render(engine, pixel_system, screen_width, screen_height));
    text_sections.extend(combo_component.render(engine, pixel_system, screen_width, screen_height));
    text_sections.extend(judgement_component.render(engine, pixel_system, screen_width, screen_height));
    text_sections.extend(hit_bar.render(engine, pixel_system, screen_width, screen_height));
    
    text_brush.queue(device, queue, text_sections).map_err(|_| SurfaceError::Lost)?;

    // Mettre à jour le buffer des receptors
    let receptor_instances = playfield.render_receptors(pixel_system);
    if !receptor_instances.is_empty() {
        queue.write_buffer(receptor_buffer, 0, bytemuck::cast_slice(&receptor_instances));
    }

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
    {
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view, resolve_target: None,
                ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
                depth_slice: None,
            })],
            depth_stencil_attachment: None, timestamp_writes: None, occlusion_query_set: None,
        });

        // Rendre les receptors
        if !receptor_instances.is_empty() {
            render_pass.set_pipeline(render_pipeline);
            for (col, _) in receptor_instances.iter().enumerate() {
                if col < receptor_bind_groups.len() {
                    render_pass.set_bind_group(0, &receptor_bind_groups[col], &[]);
                    let offset = (col * std::mem::size_of::<InstanceRaw>()) as u64;
                    let size = std::mem::size_of::<InstanceRaw>() as u64;
                    render_pass.set_vertex_buffer(0, receptor_buffer.slice(offset..offset + size));
                    render_pass.draw(0..6, 0..1);
                }
            }
        }

        // Rendre les notes - une colonne à la fois avec sa texture
        render_pass.set_pipeline(render_pipeline);
        
        for (col, col_instances) in instances_by_column.iter().enumerate() {
            if col_instances.is_empty() || col >= note_bind_groups.len() {
                continue;
            }
            
            let offset_bytes = column_offsets[col] * std::mem::size_of::<InstanceRaw>() as u64;
            let size_bytes = col_instances.len() as u64 * std::mem::size_of::<InstanceRaw>() as u64;
            
            render_pass.set_bind_group(0, &note_bind_groups[col], &[]);
            render_pass.set_vertex_buffer(0, instance_buffer.slice(offset_bytes..offset_bytes + size_bytes));
            render_pass.draw(0..6, 0..col_instances.len() as u32);
        }
        
        // Dessiner le texte
        text_brush.draw(&mut render_pass);
    }

    queue.submit(std::iter::once(encoder.finish()));
    Ok(())
}

