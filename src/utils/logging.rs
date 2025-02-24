// logging.rs - Logging-Funktionalität
use log::{info, warn, error};
use simplelog::*;
use std::fs::File;
use std::io::Result as IoResult;

pub fn init_logging() -> IoResult<()> {
    // Versuche, die Log-Datei zu erstellen. Falls ein Fehler auftritt, wird dieser zurückgegeben.
    let log_file = File::create("error.log")?;
    
    // Kombiniertes Logging: Terminal- und Datei-Logger
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
        WriteLogger::new(LevelFilter::Error, Config::default(), log_file),
    ])
    .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    Ok(())
}
