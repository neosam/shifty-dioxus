use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShiftyError {
    #[error("reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Time ComponentRange error: {0}")]
    TimeComponentRange(#[from] time::error::ComponentRange),
}

pub fn error_handler(e: ShiftyError) {
    match e {
        ShiftyError::Reqwest(e) => {
            eprintln!("Error: {}", e);
            if let Some(StatusCode::UNAUTHORIZED) = e.status() {
                let _ = web_sys::window().expect("no window").location().reload();
            }
        }
        ShiftyError::TimeComponentRange(e) => {
            eprintln!("Error: {}", e);
        }
    }
}

pub fn result_handler<T>(res: Result<T, ShiftyError>) -> Option<T> {
    match res {
        Ok(t) => Some(t),
        Err(e) => {
            error_handler(e);
            None
        }
    }
}
