pub mod frame;
pub mod render;
pub mod player;

// audio constants
pub const AUDIO_DIRECTORY: &str = "./src/resources/sounds/";
pub const AUDIO_FILE_EXTENSION: &str = ".wav";
pub const AUDIO_EXPLODE: &str = "explode";
pub const AUDIO_LOSE: &str = "lose";
pub const AUDIO_MOVE: &str = "move";
pub const AUDIO_PEW: &str = "pew";
pub const AUDIO_STARTUP: &str = "startup";
pub const AUDIO_WIN: &str = "win";

// game constants
pub  const NUM_ROWS: usize = 20;
pub const NUM_COLS: usize = 40;
