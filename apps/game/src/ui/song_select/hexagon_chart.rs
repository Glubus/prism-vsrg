use egui::{
    Color32, FontId, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, pos2, vec2,
};
use std::f32::consts::PI;

/// Data for one axis of the hexagon chart.
pub struct HexagonChartAxis {
    pub label: String,
    pub value: f32,     // 0.0 to max_value
    pub max_value: f32, // The scale maximum for this chart
    pub color: Color32,
}

pub struct HexagonChart {
    pub axes: Vec<HexagonChartAxis>,
    pub size: f32,
}

impl HexagonChart {
    pub fn new(size: f32) -> Self {
        Self {
            axes: Vec::new(),
            size,
        }
    }

    pub fn add_axis(mut self, label: &str, value: f32, max_value: f32, color: Color32) -> Self {
        self.axes.push(HexagonChartAxis {
            label: label.to_string(),
            value,
            max_value,
            color,
        });
        self
    }

    pub fn render(self, ui: &mut Ui) -> Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());

        if ui.is_rect_visible(rect) {
            let center = rect.center();
            let radius = self.size * 0.4; // Leave room for labels
            let painter = ui.painter();

            self.draw_background_webs(painter, center, radius);
            self.draw_data_polygon(painter, center, radius);
            self.draw_labels(painter, center, radius);
        }

        response
    }

    fn draw_background_webs(&self, painter: &Painter, center: Pos2, radius: f32) {
        let n = self.axes.len();
        if n < 3 {
            return;
        }

        let steps = 4;
        let stroke = Stroke::new(1.0, Color32::from_white_alpha(30));

        for s in 1..=steps {
            let r = radius * (s as f32 / steps as f32);
            let mut points = Vec::with_capacity(n);

            for i in 0..n {
                let angle = (i as f32 * 2.0 * PI / n as f32) - PI / 2.0;
                points.push(center + vec2(r * angle.cos(), r * angle.sin()));
            }

            // Close the loop
            points.push(points[0]);

            painter.add(Shape::line(points, stroke));
        }

        // Draw spokes
        for i in 0..n {
            let angle = (i as f32 * 2.0 * PI / n as f32) - PI / 2.0;
            let end_point = center + vec2(radius * angle.cos(), radius * angle.sin());
            painter.line_segment([center, end_point], stroke);
        }
    }

    fn draw_data_polygon(&self, painter: &Painter, center: Pos2, radius: f32) {
        let n = self.axes.len();
        if n < 3 {
            return;
        }

        let active_color = Color32::from_rgb(255, 0, 60); // Prism Red
        let fill_color = Color32::from_rgba_premultiplied(255, 0, 60, 50);
        let stroke_color = Color32::from_rgb(255, 50, 100);

        let mut points = Vec::with_capacity(n);

        for (i, axis) in self.axes.iter().enumerate() {
            let angle = (i as f32 * 2.0 * PI / n as f32) - PI / 2.0;

            // Normalized value 0.0 - 1.0 clamped
            let t = (axis.value / axis.max_value).clamp(0.0, 1.1);
            let r = radius * t;

            points.push(center + vec2(r * angle.cos(), r * angle.sin()));
        }

        painter.add(Shape::convex_polygon(
            points.clone(),
            fill_color,
            Stroke::new(2.0, stroke_color),
        ));

        // Draw points at vertices
        for point in points {
            painter.circle_filled(point, 3.0, active_color);
        }
    }

    fn draw_labels(&self, painter: &Painter, center: Pos2, radius: f32) {
        let n = self.axes.len();
        let text_color = Color32::LIGHT_GRAY;

        for (i, axis) in self.axes.iter().enumerate() {
            let angle = (i as f32 * 2.0 * PI / n as f32) - PI / 2.0;
            // Push label out a bit further
            let r = radius + 15.0;
            let pos = center + vec2(r * angle.cos(), r * angle.sin());

            painter.text(
                pos,
                egui::Align2::CENTER_CENTER,
                &axis.label,
                FontId::proportional(12.0),
                text_color,
            );

            // Draw Value below label
            let val_pos = pos + vec2(0.0, 12.0);
            painter.text(
                val_pos,
                egui::Align2::CENTER_CENTER,
                format!("{:.1}", axis.value),
                FontId::proportional(10.0),
                axis.color,
            );
        }
    }
}
