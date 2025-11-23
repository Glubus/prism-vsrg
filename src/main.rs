mod app;
mod renderer;
mod engine;
mod playfield;
mod components;
mod skin;
mod database;
mod menu;

use winit::event_loop::{EventLoop, ControlFlow};
use app::App;

fn main() {
    env_logger::init();
    
    // Créer un runtime tokio global pour les opérations async
    // On doit le garder en vie pendant toute la durée de l'application
    let rt = tokio::runtime::Runtime::new().unwrap();
    let _enter = rt.enter(); // Entrer dans le contexte du runtime
    
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll); // Pour un jeu, on veut Poll (max FPS)

    let mut app = App::new();
    
    // Winit 0.30 utilise run_app
    // Le runtime tokio reste actif grâce à rt qui n'est pas drop
    let _ = event_loop.run_app(&mut app);
}