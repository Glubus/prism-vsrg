pub mod events;
pub mod manager;

use std::thread;
use crate::system::bus::SystemBus;
use crate::input::manager::InputManager;

pub fn start_thread(bus: SystemBus, mut manager: InputManager) {
    thread::Builder::new()
        .name("Input Thread".to_string())
        .spawn(move || {
            log::info!("INPUT: Thread started");

            // Boucle bloquante : attend un event, le traite, recommence.
            // C'est ultra efficace et ne consomme pas de CPU à vide.
            while let Ok(raw_event) = bus.raw_input_rx.recv() {
                // Le Manager contient la map des touches (KeyBindings)
                // Il retourne Some(Action) si la touche correspond à quelque chose.
                if let Some(action) = manager.process(raw_event) {
                    // On envoie direct à la Logique
                    if let Err(e) = bus.action_tx.send(action) {
                        log::error!("INPUT: Failed to send action (Logic thread died?): {}", e);
                        break;
                    }
                }
            }

            log::info!("INPUT: Thread stopped");
        })
        .expect("Failed to spawn Input thread");
}