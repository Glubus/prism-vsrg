pub mod audio;
pub mod engine;
pub mod state;

use std::thread;
use std::time::{Duration, Instant};
use crate::system::bus::{SystemBus, SystemEvent};
use crate::logic::state::GlobalState;
use crate::database::DbManager;

const TPS: u64 = 200; 

pub fn start_thread(bus: SystemBus, db_manager: DbManager) {
    thread::Builder::new()
        .name("Logic Thread".to_string())
        .spawn(move || {
            log::info!("LOGIC: Thread started");

            // 1. Connexion DB
            db_manager.init();
            // 2. Force Rescan au démarrage pour peupler la DB
            db_manager.rescan();

            let mut state = GlobalState::new(db_manager);
            
            let mut accumulator = Duration::new(0, 0);
            let mut last_time = Instant::now();
            let target_dt = Duration::from_secs_f64(1.0 / TPS as f64);

            loop {
                // 1. Inputs
                while let Ok(action) = bus.action_rx.try_recv() {
                    state.handle_action(action);
                }

                // 2. Système
                while let Ok(sys_evt) = bus.sys_rx.try_recv() {
                    match sys_evt {
                        SystemEvent::Quit => {
                            log::info!("LOGIC: Quit received...");
                            state.shutdown();
                            return; 
                        }
                        SystemEvent::Resize { width, height } => {
                            state.resize(width, height);
                        }
                        _ => {}
                    }
                }

                // 3. Physique
                let current_time = Instant::now();
                let delta = current_time - last_time;
                last_time = current_time;
                accumulator += delta;

                let mut loops = 0;
                while accumulator >= target_dt && loops < 10 {
                    state.update(target_dt.as_secs_f64());
                    accumulator -= target_dt;
                    loops += 1;
                }

                // 4. Rendu
                let snapshot = state.create_snapshot();
                let _ = bus.render_tx.try_send(snapshot); 
                state.frame_end();
                thread::sleep(Duration::from_millis(1));
            }
        })
        .expect("Failed to spawn Logic thread");
}