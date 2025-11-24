use crate::database::models::Replay;
use crate::models::stats::HitStats;
use crate::views::components::common::{QuadInstance, quad_from_rect};
use bytemuck::cast_slice;
use serde_json;
use std::collections::HashMap;
use wgpu::{Buffer, Device, Queue, RenderPipeline, TextureView};
use wgpu_text::{glyph_brush::Section, TextBrush};

#[derive(Debug, Clone)]
pub struct ScoreCard {
    pub accuracy: f64,
    pub timestamp: i64,
    pub hit_stats: HitStats,
}

impl ScoreCard {
    pub fn from_replay(replay: &Replay) -> Option<Self> {
        // Parser le JSON pour extraire les hit stats
        let hit_stats = if let Ok(replay_data) = serde_json::from_str::<crate::models::replay::ReplayData>(&replay.data) {
            replay_data.hit_stats.unwrap_or_else(HitStats::new)
        } else {
            HitStats::new()
        };
        
        Some(ScoreCard {
            accuracy: replay.accuracy,
            timestamp: replay.timestamp,
            hit_stats,
        })
    }
}

pub struct LeaderboardDisplay {
    cards: Vec<ScoreCard>,
    screen_width: f32,
    screen_height: f32,
}

impl LeaderboardDisplay {
    pub fn new(screen_width: f32, screen_height: f32) -> Self {
        Self {
            cards: Vec::new(),
            screen_width,
            screen_height,
        }
    }

    pub fn update_size(&mut self, screen_width: f32, screen_height: f32) {
        self.screen_width = screen_width;
        self.screen_height = screen_height;
    }

    pub fn update_scores(&mut self, replays: Vec<Replay>) {
        self.cards = replays
            .iter()
            .filter_map(|r| ScoreCard::from_replay(r))
            .collect();
    }

    pub fn render(
        &self,
        device: &Device,
        queue: &Queue,
        text_brush: &mut TextBrush,
        view: &TextureView,
        quad_pipeline: &RenderPipeline,
        quad_buffer: &Buffer,
    ) -> Result<(), wgpu::SurfaceError> {
        if self.cards.is_empty() {
            return Ok(());
        }

        let panel_width = self.screen_width * 0.28;
        let panel_x = 20.0;
        let panel_y = 20.0;
        let panel_height = self.screen_height - 40.0;
        let card_height = 120.0;
        let card_spacing = 10.0;
        let card_padding = 10.0;

        // Créer les quads pour le panneau et les cards
        let mut quads = Vec::new();

        // Panneau de fond
        quads.push(quad_from_rect(
            panel_x,
            panel_y,
            panel_width,
            panel_height,
            [0.15, 0.15, 0.15, 0.9],
            self.screen_width,
            self.screen_height,
        ));

        // Cards
        for (i, card) in self.cards.iter().take(10).enumerate() {
            let card_y = panel_y + 50.0 + (i as f32 * (card_height + card_spacing));
            if card_y + card_height > panel_y + panel_height {
                break;
            }

            // Card background
            quads.push(quad_from_rect(
                panel_x + card_padding,
                card_y,
                panel_width - card_padding * 2.0,
                card_height,
                [0.2, 0.2, 0.2, 1.0],
                self.screen_width,
                self.screen_height,
            ));
        }

        // Rendre les quads
        if !quads.is_empty() {
            queue.write_buffer(quad_buffer, 0, cast_slice(&quads));

            let mut encoder =
                device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
            {
                let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Leaderboard Render Pass"),
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Load,
                            store: wgpu::StoreOp::Store,
                        },
                        depth_slice: None,
                    })],
                    depth_stencil_attachment: None,
                    timestamp_writes: None,
                    occlusion_query_set: None,
                });

                render_pass.set_pipeline(quad_pipeline);
                render_pass.set_vertex_buffer(0, quad_buffer.slice(..));
                render_pass.draw(0..4, 0..quads.len() as u32);
            }
            queue.submit(std::iter::once(encoder.finish()));
        }

        // Créer les sections de texte
        // Stocker toutes les strings dans un Vec pour éviter les problèmes de lifetime
        let mut all_strings = Vec::new();
        let mut text_sections = Vec::new();

        // Titre
        text_sections.push(Section {
            screen_position: (panel_x + panel_width / 2.0, panel_y + 20.0),
            bounds: (panel_width, panel_height),
            text: vec![
                wgpu_text::glyph_brush::Text::new("Top Scores")
                    .with_scale(24.0)
                    .with_color([1.0, 1.0, 1.0, 1.0]),
            ],
            ..Default::default()
        });

        // Cards - préparer toutes les strings d'abord
        for card in self.cards.iter().take(10) {
            all_strings.push(format!("{:.2}%", card.accuracy));
            all_strings.push(format_date(card.timestamp));
            let stats_text = format_hit_stats(&card.hit_stats);
            for (text, _) in stats_text {
                all_strings.push(text);
            }
        }

        // Maintenant créer les sections de texte
        let mut string_idx = 0;
        for (i, card) in self.cards.iter().take(10).enumerate() {
            let card_y = panel_y + 50.0 + (i as f32 * (card_height + card_spacing));
            if card_y + card_height > panel_y + panel_height {
                break;
            }

            let text_x = panel_x + card_padding + 10.0;
            let mut text_y = card_y + 15.0;

            // Accuracy
            text_sections.push(Section {
                screen_position: (text_x, text_y),
                bounds: (panel_width, panel_height),
                text: vec![
                    wgpu_text::glyph_brush::Text::new(&all_strings[string_idx])
                        .with_scale(20.0)
                        .with_color([1.0, 1.0, 1.0, 1.0]),
                ],
                ..Default::default()
            });
            string_idx += 1;
            text_y += 25.0;

            // Date
            text_sections.push(Section {
                screen_position: (text_x, text_y),
                bounds: (panel_width, panel_height),
                text: vec![
                    wgpu_text::glyph_brush::Text::new(&all_strings[string_idx])
                        .with_scale(14.0)
                        .with_color([0.7, 0.7, 0.7, 1.0]),
                ],
                ..Default::default()
            });
            string_idx += 1;
            text_y += 25.0;

            // Hit stats en couleurs
            let stats_text = format_hit_stats(&card.hit_stats);
            let mut x_offset = 0.0;
            for (_, color) in stats_text {
                text_sections.push(Section {
                    screen_position: (text_x + x_offset, text_y),
                    bounds: (panel_width, panel_height),
                    text: vec![
                        wgpu_text::glyph_brush::Text::new(&all_strings[string_idx])
                            .with_scale(12.0)
                            .with_color(color),
                    ],
                    ..Default::default()
                });
                // Estimation de la largeur du texte (approximatif)
                x_offset += all_strings[string_idx].len() as f32 * 7.0 + 5.0;
                string_idx += 1;
            }
        }

        text_brush
            .queue(device, queue, text_sections)
            .map_err(|_| wgpu::SurfaceError::Lost)?;

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor::default());
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Leaderboard Text Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            text_brush.draw(&mut render_pass);
        }

        queue.submit(std::iter::once(encoder.finish()));
        Ok(())
    }
}

fn format_date(timestamp: i64) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let datetime = SystemTime::from(UNIX_EPOCH) + std::time::Duration::from_secs(timestamp as u64);
    
    // Format simple : JJ/MM/AAAA
    // Pour l'instant, on utilise une approche simple
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let diff = now - timestamp;
    
    if diff < 3600 {
        // Moins d'une heure
        format!("{} min ago", diff / 60)
    } else if diff < 86400 {
        // Moins d'un jour
        format!("{} hours ago", diff / 3600)
    } else if diff < 604800 {
        // Moins d'une semaine
        format!("{} days ago", diff / 86400)
    } else {
        // Plus d'une semaine - afficher la date
        let days_since_epoch = timestamp / 86400;
        format!("{} days ago", diff / 86400)
    }
}

fn format_hit_stats(stats: &HitStats) -> Vec<(String, [f32; 4])> {
    let mut result = Vec::new();
    
    if stats.marv > 0 {
        result.push((format!("M:{} ", stats.marv), [0.0, 1.0, 1.0, 1.0])); // Cyan
    }
    if stats.perfect > 0 {
        result.push((format!("P:{} ", stats.perfect), [1.0, 1.0, 0.0, 1.0])); // Yellow
    }
    if stats.great > 0 {
        result.push((format!("G:{} ", stats.great), [0.0, 1.0, 0.0, 1.0])); // Green
    }
    if stats.good > 0 {
        result.push((format!("Go:{} ", stats.good), [0.0, 0.0, 0.5, 1.0])); // Dark blue
    }
    if stats.bad > 0 {
        result.push((format!("B:{} ", stats.bad), [1.0, 0.41, 0.71, 1.0])); // Pink
    }
    if stats.miss > 0 {
        result.push((format!("Mi:{} ", stats.miss), [1.0, 0.0, 0.0, 1.0])); // Red
    }
    
    result
}

