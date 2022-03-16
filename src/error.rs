use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + 'static>>;

#[derive(Debug, Clone)]
pub struct WavF64VecError {
    err_kind: WavF64VecErrorKind,
    op_additional_message:Option<String>,
}

impl WavF64VecError {
    pub fn new(err_kind: WavF64VecErrorKind, op_additional_message:Option<String>) -> Box<dyn std::error::Error + Send + Sync + 'static> {
        Box::<WavF64VecError>::new(WavF64VecError {
            err_kind: err_kind,
            op_additional_message: op_additional_message,
        })
    }
}

impl fmt::Display for WavF64VecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        for err_message in WAVE_AUDIO_ERR_MESSAGE {
            if err_message.err_kind == self.err_kind {
                if let Some(additional_message) = &self.op_additional_message {
                    return write!(f, "{}", format!("{} : {}", err_message.message, additional_message));
                }
                else {
                    return write!(f, "{}", format!("{}", err_message.message));
                }
            }
        }
        panic!()
    }
}

impl std::error::Error for WavF64VecError {}

#[derive(Debug, Clone, PartialEq)]
pub enum WavF64VecErrorKind {
    PathIsNotFile,
    PathHasNoWavExtention,
    FileIsNotCompatibleFormat,
    SubChunkSizeError,
    SubChunkDuplication,
    NoRequiredSubChunk,
    FormatIsNotSupported,
    BytesLengthError,
}

struct WavF64VecErrorMessage {
    err_kind: WavF64VecErrorKind,
    message: &'static str,
}

const WAVE_AUDIO_ERR_MESSAGE:[WavF64VecErrorMessage;8] = [
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::PathIsNotFile,
        message: "Specified path is not file.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::PathHasNoWavExtention,
        message: "Specified path extension is not \".wav\".",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::FileIsNotCompatibleFormat,
        message: "Specified file is not compatible wav format.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::SubChunkSizeError,
        message: "Sub chunk size is wrong.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::SubChunkDuplication,
        message: "There is sub chunk dupulication.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::NoRequiredSubChunk,
        message: "There is no required sub chunk.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::FormatIsNotSupported,
        message: "Specified Wave format is not supported.",
    },
    WavF64VecErrorMessage {
        err_kind: WavF64VecErrorKind::BytesLengthError,
        message: "Bytes length is wrong.",
    },
];
