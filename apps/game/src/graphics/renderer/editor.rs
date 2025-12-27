//! Skin editor rendering.

use super::Renderer;

pub fn render(renderer: &mut Renderer, ctx: &egui::Context) {
    // Affiche l'UI de l'éditeur
    if renderer
        .skin_editor
        .show(ctx, &mut renderer.resources.skin, renderer.offscreen_id)
    {
        let s = renderer.resources.skin.clone();
        renderer.resources.reload_textures(&renderer.ctx, ctx, &s);
    }

    // MISE À JOUR TEMPS RÉEL DES POSITIONS
    // On met à jour les RenderResources avec les dimensions de la preview
    // si elle existe, sinon avec la taille écran.
    // Cela permet de voir les éléments bouger instantanément.
    let (w, h) = if renderer.offscreen_view.is_some() {
        (
            renderer.skin_editor.state.preview_width as f32,
            renderer.skin_editor.state.preview_height as f32,
        )
    } else {
        (
            renderer.ctx.config.width as f32,
            renderer.ctx.config.height as f32,
        )
    };

    renderer.resources.update_component_positions(w, h);
}
