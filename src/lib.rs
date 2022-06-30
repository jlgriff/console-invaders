pub mod frame;
pub mod render;
pub mod player;
pub mod laser;
pub mod invaders;

// audio constants
pub const AUDIO_DIRECTORY: &str = "./src/resources/sounds/";
pub const AUDIO_FILE_EXTENSION: &str = ".wav";
pub const AUDIO_EXPLODE: &str = "explode";
pub const AUDIO_LOSE: &str = "lose2";
pub const AUDIO_MOVE: &str = "move";
pub const AUDIO_PEW: &str = "pew2";
pub const AUDIO_STARTUP: &str = "startup";
pub const AUDIO_WIN: &str = "win";

// game constants
pub const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
pub const NUM_SHOTS: usize = 2;
pub const NUM_ROWS_INVADERS_START_IN: usize = 9;
