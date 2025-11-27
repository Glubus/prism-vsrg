use crate::input::events::{GameAction, InputCommand, RawInputEvent};
use crossbeam_channel::{Receiver, Sender, bounded, unbounded};
// Correction : Import depuis shared::snapshot
use crate::shared::snapshot::RenderState;

#[derive(Debug, Clone)]
pub enum SystemEvent {
    Resize { width: u32, height: u32 },
    FocusLost,
    FocusGained,
    Quit,
}

/// Le Bus contient tous les canaux de communication.
#[derive(Clone)]
pub struct SystemBus {
    // Main -> Input (Touches brutes)
    pub raw_input_tx: Sender<RawInputEvent>,
    pub raw_input_rx: Receiver<RawInputEvent>,

    // Commands vers le thread Input
    pub input_cmd_tx: Sender<InputCommand>,
    pub input_cmd_rx: Receiver<InputCommand>,

    // Input -> Logic (Actions de jeu)
    pub action_tx: Sender<GameAction>,
    pub action_rx: Receiver<GameAction>,

    // Logic -> Render (Snapshot) - ACTIVÉ
    pub render_tx: Sender<RenderState>,
    pub render_rx: Receiver<RenderState>,

    // Main -> Logic (Événements système)
    pub sys_tx: Sender<SystemEvent>,
    pub sys_rx: Receiver<SystemEvent>,
}

impl SystemBus {
    pub fn new() -> Self {
        let (raw_input_tx, raw_input_rx) = unbounded();
        let (input_cmd_tx, input_cmd_rx) = unbounded();
        let (action_tx, action_rx) = unbounded();

        // Canal borné pour le rendu (2 frames max en attente pour éviter la latence)
        let (render_tx, render_rx) = bounded(2);

        let (sys_tx, sys_rx) = unbounded();

        Self {
            raw_input_tx,
            raw_input_rx,
            input_cmd_tx,
            input_cmd_rx,
            action_tx,
            action_rx,
            render_tx,
            render_rx, // Initialisation ajoutée ici
            sys_tx,
            sys_rx,
        }
    }
}
