//! This libary provides the following features:
//! * Reading & writing wav files.
//! * Interconversion between wav audio data and a tupple that has a wave format structure and a audio data vector(`Vec<Vec<f64>>`). The order of the audio data vector's (`Vec<Vec<f64>>`) dimensions can be specified by corresponding APIs in each.
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;

use std::fs::File;

mod error;
use error::*;

mod tests;

pub const WAVEFORMAT_ID_PCM: usize = 0x0001;
pub const WAVEFORMAT_ID_IEEE_FLOAT: usize = 0x0003;
pub const WAVEFORMAT_ID_EXTENSIBLE: usize = 0xfffe;

const WAVEFORMATEXTENSIBLE_SUBTYPE_PCM_GUID_LEBYTES: [u8; 16] = [
    0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
];
const WAVEFORMATEXTENSIBLE_SUBTYPE_IEEE_FLOAT_GUID_LEBYTES: [u8; 16] = [
    0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x00, 0x80, 0x00, 0x00, 0xaa, 0x00, 0x38, 0x9b, 0x71,
];

/// Wave audio format structure.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct WaveFormat {
    /// Format id.
    pub id: usize,
    /// Number of channels.
    pub channel: usize,
    /// Sampling rate.
    pub sampling_rate: usize,
    /// Bits per sample.
    pub bits: usize,
}

impl WaveFormat {
    /// Check the format us supported.
    pub fn format_check(wave_format: &WaveFormat) -> Result<()> {
        if wave_format.channel < 1 || wave_format.channel > 2 {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::FormatIsNotSupported,
                Some("channel number".to_string()),
            ));
        }
        if wave_format.bits == 0 || wave_format.bits > 64 {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::FormatIsNotSupported,
                Some("bit rate".to_string()),
            ));
        }
        match wave_format.sampling_rate {
            8000 => {}
            16000 => {}
            22050 => {}
            44100 => {}
            32000 => {}
            48000 => {}
            96000 => {}
            192000 => {}
            _ => {
                return Err(WavF64VecError::new(
                    WavF64VecErrorKind::FormatIsNotSupported,
                    Some("sampling rate".to_string()),
                ));
            }
        }
        Ok(())
    }
}

/// RIFF sub chunk.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct SubChunk {
    /// Chuck identifier.
    pub chunk_id: [u8; 4],
    /// Chunk data.
    pub bytes_data_vec: Vec<u8>,
}

impl SubChunk {
    pub fn new() -> SubChunk {
        SubChunk {
            chunk_id: [0, 0, 0, 0],
            bytes_data_vec: Vec::new(),
        }
    }
}

/// Wav file structure.
#[allow(dead_code)]
#[derive(Clone, PartialEq, Debug)]
pub struct WavFile {
    /// Path of wav file.
    pub file_path: PathBuf,
    /// Sub chunk vec.
    pub sub_chunks: Vec<SubChunk>,
}

impl WavFile {
    /// Create structure.
    pub fn new() -> WavFile {
        WavFile {
            file_path: PathBuf::new(),
            sub_chunks: Vec::new(),
        }
    }

    /// Open wav file and add data to self.
    pub fn open(&mut self, file_path: &Path) -> Result<()> {
        // -- Check Parameter --
        if !file_path.is_file() {
            return Err(WavF64VecError::new(WavF64VecErrorKind::PathIsNotFile, None));
        }
        if let Some(extension_str) = file_path.extension() {
            let extension_string = extension_str.to_ascii_lowercase();
            if extension_string != "wav" {
                return Err(WavF64VecError::new(WavF64VecErrorKind::PathHasNoWavExtention, None));
            }
        } else {
            return Err(WavF64VecError::new(WavF64VecErrorKind::PathHasNoWavExtention, None));
        }

        // -- Read Whole File --
        let target_file = File::open(file_path)?;
        let mut buf = Vec::new();
        let file_size = BufReader::new(&target_file).read_to_end(&mut buf)?;

        // -- Get WavFile Construction --
        // "RIFF"
        if buf[0x00..0x04] != [b'R', b'I', b'F', b'F'] {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::FileIsNotCompatibleFormat,
                Some("\"RIFF\"".to_string()),
            ));
        }
        // RIFF Size
        let riff_size = usize::try_from(u32::from_le_bytes(<[u8; 4]>::try_from(&buf[0x04..0x08])?))?;
        if riff_size != file_size - 8 {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::FileIsNotCompatibleFormat,
                Some("RIFF Size".to_string()),
            ));
        }
        // "WAVE"
        if buf[0x08..0x0c] != [b'W', b'A', b'V', b'E'] {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::FileIsNotCompatibleFormat,
                Some("\"WAVE\"".to_string()),
            ));
        }

        let sub_chunks_vec = self.extract_sub_chunks(buf[0x0c..].to_vec(), file_size - 12)?;

        self.file_path = file_path.to_path_buf();
        self.sub_chunks = sub_chunks_vec;
        Ok(())
    }

    /// Save self to wav file.
    pub fn save(&mut self) -> Result<()> {
        let file_path = self.file_path.clone();
        self.save_as(&file_path)?;
        Ok(())
    }

    /// Save self to wav file as argument path.
    pub fn save_as(&mut self, file_path: &Path) -> Result<()> {
        // -- Check Parameter --
        if let Some(extension_str) = file_path.extension() {
            let extension_string = extension_str.to_ascii_lowercase();
            if extension_string != "wav" {
                return Err(WavF64VecError::new(WavF64VecErrorKind::PathHasNoWavExtention, None));
            }
        } else {
            return Err(WavF64VecError::new(WavF64VecErrorKind::PathHasNoWavExtention, None));
        }
        let mut buf: Vec<u8> = Vec::new();
        buf.append(&mut [b'R', b'I', b'F', b'F'].to_vec());
        let mut riff_size: usize = 4;
        for sub_chunk in &self.sub_chunks {
            if let Some (new_riff_size) = riff_size.checked_add(sub_chunk.bytes_data_vec.len() + 8) {
                dbg!(new_riff_size);
                riff_size = new_riff_size;
            }
            else {
                return Err(WavF64VecError::new(WavF64VecErrorKind::SubChunkSizeTooLarge, None));
            }
        }
        if riff_size > 0xffffffff - 8 {
            return Err(WavF64VecError::new(WavF64VecErrorKind::SubChunkSizeTooLarge, None));
        }
        buf.append(&mut riff_size.to_le_bytes()[0..4].to_vec());
        buf.append(&mut [b'W', b'A', b'V', b'E'].to_vec());
        for sub_chunk in &mut self.sub_chunks {
            buf.append(&mut sub_chunk.chunk_id.to_vec());
            buf.append(&mut sub_chunk.bytes_data_vec.len().to_le_bytes()[0..4].to_vec());
            buf.append(&mut sub_chunk.bytes_data_vec.clone());
        }
        let mut target_file = File::create(file_path)?;
        target_file.write_all(&buf)?;

        // Update Self Infomation
        self.file_path = file_path.to_path_buf();
        Ok(())
    }

    fn extract_sub_chunks(&self, buf: Vec<u8>, chunks_size: usize) -> Result<Vec<SubChunk>> {
        let mut sub_chunks_vec: Vec<SubChunk> = Vec::new();
        let mut chunk_head_addr: usize = 0x00;
        while chunks_size - chunk_head_addr >= 8 {
            let chunk_head_buf = &buf[chunk_head_addr..];
            let chunk_body_size = usize::try_from(u32::from_le_bytes(<[u8; 4]>::try_from(&chunk_head_buf[0x04..0x08])?))?;
            if chunk_head_buf.len() < chunk_body_size + 8 {
                return Err(WavF64VecError::new(WavF64VecErrorKind::SubChunkSizeError, None));
            }
            let sub_chunk = SubChunk {
                chunk_id: [
                    chunk_head_buf[0x00],
                    chunk_head_buf[0x01],
                    chunk_head_buf[0x02],
                    chunk_head_buf[0x03],
                ],
                bytes_data_vec: chunk_head_buf[8..(chunk_body_size + 8)].to_vec(),
            };
            sub_chunks_vec.push(sub_chunk);
            chunk_head_addr += 8 + chunk_body_size;
        }
        return Ok(sub_chunks_vec);
    }

    /// Get WaveFormat
    pub fn get_format(&self) -> Result<Option<WaveFormat>> {
        for sub_chunk in &self.sub_chunks {
            match sub_chunk.chunk_id {
                [b'f', b'm', b't', b' '] => {
                    return Ok(Some(Self::get_format_from_chunk(&sub_chunk.bytes_data_vec)?));
                }
                _ => {}
            }
        }
        Ok(None)
    }

    fn get_format_from_chunk(chunk_body: &Vec<u8>) -> Result<WaveFormat> {
        // format id
        if chunk_body.len() < 0x10 {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::SubChunkSizeError,
                Some("\"fmt\"".to_string()),
            ));
        }
        let mut format_id = usize::from(u16::from_le_bytes(<[u8; 2]>::try_from(&chunk_body[0x00..0x02])?));
        match format_id {
            WAVEFORMAT_ID_PCM => {}
            WAVEFORMAT_ID_IEEE_FLOAT => {}
            WAVEFORMAT_ID_EXTENSIBLE => {
                if chunk_body.len() < 0x28 {
                    return Err(WavF64VecError::new(
                        WavF64VecErrorKind::SubChunkSizeError,
                        Some("\"fmt\"".to_string()),
                    ));
                }
                format_id = usize::from(u16::from_le_bytes(<[u8; 2]>::try_from(&chunk_body[0x18..0x1A])?));

                let waveextensible_subtype_guid: &[u8; 16];
                if format_id == WAVEFORMAT_ID_PCM {
                    waveextensible_subtype_guid = &WAVEFORMATEXTENSIBLE_SUBTYPE_PCM_GUID_LEBYTES;
                } else if format_id == WAVEFORMAT_ID_IEEE_FLOAT {
                    waveextensible_subtype_guid = &WAVEFORMATEXTENSIBLE_SUBTYPE_IEEE_FLOAT_GUID_LEBYTES;
                } else {
                    return Err(WavF64VecError::new(
                        WavF64VecErrorKind::SubChunkSizeError,
                        Some("format id".to_string()),
                    ));
                }
                for (idx, byte_data) in chunk_body[0x18..0x28].iter().enumerate() {
                    if *byte_data != waveextensible_subtype_guid[idx] {
                        return Err(WavF64VecError::new(
                            WavF64VecErrorKind::SubChunkSizeError,
                            Some("format id".to_string()),
                        ));
                    }
                }
            }
            _ => {
                return Err(WavF64VecError::new(
                    WavF64VecErrorKind::SubChunkSizeError,
                    Some("format id".to_string()),
                ));
            }
        }

        // channel
        let channel = usize::from(u16::from_le_bytes(<[u8; 2]>::try_from(&chunk_body[0x02..0x04])?));
        // Sampling Rate
        let sampling_rate = usize::try_from(u32::from_le_bytes(<[u8; 4]>::try_from(&chunk_body[0x04..0x08])?))?;
        // Byte Per Sec
        let bytes_per_sec = usize::try_from(u32::from_le_bytes(<[u8; 4]>::try_from(&chunk_body[0x08..0x0c])?))?;
        // Block Size
        let block_size = usize::from(u16::from_le_bytes(<[u8; 2]>::try_from(&chunk_body[0x0c..0x0e])?));
        // Bit Rate
        let bits = usize::from(u16::from_le_bytes(<[u8; 2]>::try_from(&chunk_body[0x0e..0x10])?));

        // Check Byte Per Sec.
        if bytes_per_sec != channel * sampling_rate * (bits / 8) {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::SubChunkSizeError,
                Some("bytes per sec".to_string()),
            ));
        }
        // Check Block Size.
        if block_size != channel * bits / 8 {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::SubChunkSizeError,
                Some("block size".to_string()),
            ));
        }
        Ok(WaveFormat {
            id: format_id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        })
    }

    fn set_format(&self, wave_format: &WaveFormat) -> Result<Vec<u8>> {
        // fmt chunk
        let mut chunk_body: Vec<u8> = Vec::new();
        // format id
        chunk_body.append(&mut wave_format.id.to_le_bytes()[0..2].to_vec());
        // channel
        chunk_body.append(&mut wave_format.channel.to_le_bytes()[0..2].to_vec());
        // Sampling Rate
        chunk_body.append(&mut wave_format.sampling_rate.to_le_bytes()[0..4].to_vec());
        // Byte Per Sec
        chunk_body
            .append(&mut (wave_format.channel * wave_format.sampling_rate * (wave_format.bits / 8)).to_le_bytes()[0..4].to_vec());
        // Block Size
        chunk_body.append(&mut (wave_format.channel * wave_format.bits / 8).to_le_bytes()[0..2].to_vec());
        // Bit Rate
        chunk_body.append(&mut wave_format.bits.to_le_bytes()[0..2].to_vec());
        Ok(chunk_body)
    }

    /// Get audio data. Retrun Value: Vec<Vec<f64>: Outer is channel vec. Inner is data vec.
    pub fn get_channel_vec_audio(&self) -> Result<(WaveFormat, Vec<Vec<f64>>)> {
        let (wave_format, bytes_data) = self.get_bytes_audio()?;
        let channel_data_vec = Self::to_channel_vec(&wave_format, bytes_data)?;
        Ok((wave_format, channel_data_vec))
    }

    /// Get audio data. Retrun Value: Vec<Vec<f64>: Outer is data vec. Inner is channel vec.
    pub fn get_data_vec_audio(&self) -> Result<(WaveFormat, Vec<Vec<f64>>)> {
        let (wave_format, bytes_data) = self.get_bytes_audio()?;
        let data_channel_vec = Self::to_data_vec(&wave_format, bytes_data)?;
        Ok((wave_format, data_channel_vec))
    }

    fn get_bytes_audio(&self) -> Result<(WaveFormat, Vec<u8>)> {
        let mut op_wave_format: Option<WaveFormat> = None;
        let mut op_bytes_data: Option<Vec<u8>> = None;
        for sub_chunk in &self.sub_chunks {
            match sub_chunk.chunk_id {
                [b'f', b'm', b't', b' '] => {
                    if op_wave_format.is_none() {
                        op_wave_format = Some(Self::get_format_from_chunk(&sub_chunk.bytes_data_vec)?);
                    } else {
                        return Err(WavF64VecError::new(
                            WavF64VecErrorKind::SubChunkDuplication,
                            Some("\"fmt\"".to_string()),
                        ));
                    }
                }
                [b'd', b'a', b't', b'a'] => {
                    if op_bytes_data.is_none() {
                        op_bytes_data = Some(sub_chunk.bytes_data_vec.clone());
                    } else {
                        return Err(WavF64VecError::new(
                            WavF64VecErrorKind::SubChunkDuplication,
                            Some("\"data\"".to_string()),
                        ));
                    }
                }
                _ => {}
            }
        }
        if op_wave_format.is_none() {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::NoRequiredSubChunk,
                Some("\"fmt\"".to_string()),
            ));
        } else if op_bytes_data.is_none() {
            return Err(WavF64VecError::new(
                WavF64VecErrorKind::NoRequiredSubChunk,
                Some("\"data\"".to_string()),
            ));
        } else {
            Ok((op_wave_format.unwrap(), op_bytes_data.unwrap()))
        }
    }

    fn to_channel_vec(wave_format: &WaveFormat, bytes_data_vec: Vec<u8>) -> Result<Vec<Vec<f64>>> {
        WaveFormat::format_check(wave_format)?;
        let mut channel_vec = Vec::new();

        let size = wave_format.bits / 8;
        let step = wave_format.channel * size;
        for _ in 0..wave_format.channel {
            channel_vec.push(Vec::new());
        }
        for (pos, _) in bytes_data_vec.iter().enumerate().step_by(step) {
            for channel_idx in 0..wave_format.channel {
                let stt = pos + channel_idx * size;
                channel_vec[channel_idx].push(bytes_to_f64wave(wave_format.id, &bytes_data_vec[stt..stt + size])?);
            }
        }
        Ok(channel_vec)
    }

    fn to_data_vec(wave_format: &WaveFormat, bytes_data_vec: Vec<u8>) -> Result<Vec<Vec<f64>>> {
        WaveFormat::format_check(wave_format)?;
        let mut data_vec = Vec::new();

        let size = wave_format.bits / 8;
        let step = wave_format.channel * size;

        for (pos, _) in bytes_data_vec.iter().enumerate().step_by(step) {
            let mut channel_vec = Vec::new();
            for channel_idx in 0..wave_format.channel {
                let stt = pos + channel_idx * size;
                channel_vec.push(bytes_to_f64wave(wave_format.id, &bytes_data_vec[stt..stt + size])?);
            }
            data_vec.push(channel_vec);
        }
        Ok(data_vec)
    }

    /// Update audio data (update "fmt" and "data" chunk). If "fmt" or "data" argument chunk do not exist, those chunks are added.
    /// Parameters: channel_data_vec(Vec<Vec<f64>): Outer is channel vec. Inner is data vec.
    pub fn update_channel_vec_audio(&mut self, wave_format: &WaveFormat, channel_data_vec: &Vec<Vec<f64>>) -> Result<()> {
        let format_buf: Vec<u8> = self.set_format(wave_format)?;
        let mut bytes_data_vec: Vec<u8> = Vec::new();
        for (data_idx, _) in channel_data_vec[0].iter().enumerate() {
            for channel_idx in 0..wave_format.channel {
                bytes_data_vec.append(&mut f64wave_to_bytes(
                    wave_format.id,
                    channel_data_vec[channel_idx][data_idx],
                    wave_format.bits,
                )?);
            }
        }
        self.update_audio(format_buf, bytes_data_vec)?;
        Ok(())
    }

    /// Update audio data (update "fmt" and "data" chunk). If "fmt" or "data" argument chunk do not exist, those chunks are added.
    /// Parameters: channel_data_vec(Vec<Vec<f64>): Outer is data vec. Inner is channel vec.
    pub fn update_data_vec_audio(&mut self, wave_format: &WaveFormat, data_channel_vec: &Vec<Vec<f64>>) -> Result<()> {
        let format_buf: Vec<u8> = self.set_format(wave_format)?;
        let mut bytes_data_vec: Vec<u8> = Vec::new();
        for (data_idx, _) in data_channel_vec.iter().enumerate() {
            for channel_idx in 0..wave_format.channel {
                bytes_data_vec.append(&mut f64wave_to_bytes(
                    wave_format.id,
                    data_channel_vec[data_idx][channel_idx],
                    wave_format.bits,
                )?);
            }
        }
        self.update_audio(format_buf, bytes_data_vec)?;
        Ok(())
    }

    fn update_audio(&mut self, format_buf: Vec<u8>, bytes_data_vec: Vec<u8>) -> Result<()> {
        let mut op_format_chunk_idx: Option<usize> = None;
        let mut op_data_chunk_idx: Option<usize> = None;
        for (chunk_idx, sub_chunk) in self.sub_chunks.iter().enumerate() {
            match sub_chunk.chunk_id {
                [b'f', b'm', b't', b' '] => {
                    if op_format_chunk_idx.is_none() {
                        op_format_chunk_idx = Some(chunk_idx);
                    } else {
                        return Err(WavF64VecError::new(
                            WavF64VecErrorKind::SubChunkDuplication,
                            Some("\"fmt\"".to_string()),
                        ));
                    }
                }
                [b'd', b'a', b't', b'a'] => {
                    if op_data_chunk_idx.is_none() {
                        op_data_chunk_idx = Some(chunk_idx);
                    } else {
                        return Err(WavF64VecError::new(
                            WavF64VecErrorKind::SubChunkDuplication,
                            Some("\"data\"".to_string()),
                        ));
                    }
                }
                _ => {}
            }
        }

        self.precheck_sub_chunk_size(op_format_chunk_idx, format_buf.len(), "fmt ".to_string())?;
        self.precheck_sub_chunk_size(op_data_chunk_idx, bytes_data_vec.len(), "data".to_string())?;

        if let Some(chunk_idx) = op_format_chunk_idx {
            self.sub_chunks[chunk_idx].bytes_data_vec = format_buf;
        } else {
            // Create New Fmt Chunk
            let mut sub_chunk = SubChunk::new();
            sub_chunk.chunk_id = [b'f', b'm', b't', b' '];
            sub_chunk.bytes_data_vec = format_buf;
            self.sub_chunks.push(sub_chunk);
        }
        if let Some(idx) = op_data_chunk_idx {
            self.sub_chunks[idx].bytes_data_vec = bytes_data_vec;
        } else {
            // Create New Data Chunk
            let mut sub_chunk = SubChunk::new();
            sub_chunk.chunk_id = [b'd', b'a', b't', b'a'];
            sub_chunk.bytes_data_vec = bytes_data_vec;
            self.sub_chunks.push(sub_chunk);
        }
        Ok(())
    }

    /// Update sub chunk. If the argument chunk's identifer does not exist, the argument chunk is added.
    pub fn update_sub_chunk(&mut self, new_chunk: SubChunk) -> Result<()> {
        let mut op_chunk_idx: Option<usize> = None;
        for (idx, existing_chunk) in self.sub_chunks.iter().enumerate() {
            if existing_chunk.chunk_id == new_chunk.chunk_id {
                op_chunk_idx = Some(idx);
                break;
            }
        }
        self.precheck_sub_chunk_size(op_chunk_idx, new_chunk.bytes_data_vec.len(), String::from_utf8(new_chunk.chunk_id.to_vec())?)?;
        if let Some(idx) = op_chunk_idx {
            self.sub_chunks[idx] = new_chunk;
        } else {
            self.sub_chunks.push(new_chunk);
        }
        Ok(())
    }

    fn precheck_sub_chunk_size(&self, op_chunk_idx: Option<usize>, new_sub_chunk_body_size: usize, chunk_id_string: String) -> Result<()> {
        let mut total_size: usize = 0;
        if op_chunk_idx.is_none() {
            total_size = new_sub_chunk_body_size;
        }

        for (idx, sub_chunk) in self.sub_chunks.iter().enumerate() {
            let sub_chunk_body_size: usize;
            if let Some(chunk_idx) = op_chunk_idx {
                if idx == chunk_idx {
                    sub_chunk_body_size = new_sub_chunk_body_size;
                }
                else {
                    sub_chunk_body_size = sub_chunk.bytes_data_vec.len();
                }
            }
            else {
                sub_chunk_body_size = sub_chunk.bytes_data_vec.len();
            }
            // 8 = chunk_id + body_size
            if let Some(sub_chunk_size) = sub_chunk_body_size.checked_add(8) {
                if let Some(new_total_size) = total_size.checked_add(sub_chunk_size) {
                    total_size = new_total_size;
                    // 12 = "RIFF" + RIFF Size + "WAVE"
                    if total_size > 0xffffffff - 12 {
                        return Err(WavF64VecError::new(WavF64VecErrorKind::SubChunkSizeTooLarge, Some(chunk_id_string)));
                    }
                }
            }
            else {
                return Err(WavF64VecError::new(WavF64VecErrorKind::SubChunkSizeTooLarge, Some(chunk_id_string)));
            }
        }
        Ok(())
    }

    /// Delete sub chunk. If the chunk was deleted, return true. If the argument chunk identifer does not exist, return false.
    pub fn delete_sub_chunk(&mut self, sub_chunk_id: [u8; 4]) -> bool {
        for (idx, existing_chunk) in &mut self.sub_chunks.iter().enumerate() {
            if existing_chunk.chunk_id == sub_chunk_id {
                self.sub_chunks.remove(idx);
                return true;
            }
        }
        false
    }

    /// Get sub chunk identifier vector.
    pub fn get_sub_chunk_id_vec(&mut self) -> Vec<[u8; 4]> {
        let mut sub_chunk_id_vec: Vec<[u8; 4]> = Vec::new();
        for existing_chunk in &self.sub_chunks {
            sub_chunk_id_vec.push(existing_chunk.chunk_id.clone());
        }
        sub_chunk_id_vec
    }

    /// Get sub_chunk index. If the argument chunk identifer exists, returns Some(Index). Otherwise None.
    pub fn get_sub_chunk_idx(&mut self, sub_chunk_id: [u8; 4]) -> Option<usize> {
        for (idx, existing_chunk) in &mut self.sub_chunks.iter().enumerate() {
            if existing_chunk.chunk_id == sub_chunk_id {
                return Some(idx);
            }
        }
        None
    }
}

/// Convert from a bytes data vector to a audio data value(`f64`).
pub fn bytes_to_f64wave(format_id: usize, bytes: &[u8]) -> Result<f64> {
    let bytes_len = bytes.len();

    match format_id {
        WAVEFORMAT_ID_PCM => {
            if bytes_len == 1 {
                let mut buffer: [u8; 1] = [0; 1];
                //to signed 8bit
                if bytes[0] < 128 {
                    buffer[0] = bytes[0] + 128;
                } else {
                    buffer[0] = bytes[0] - 128;
                }
                Ok(f64::from(i8::from_le_bytes(buffer)) / f64::from(i8::MAX))
            } else if bytes_len == 2 {
                //signed 16bit
                let mut buffer: [u8; 2] = [0; 2];
                for i in 0..bytes_len {
                    buffer[i] = bytes[i];
                }
                Ok(f64::from(i16::from_le_bytes(buffer)) / f64::from(i16::MAX))
            } else if bytes_len == 3 {
                //signed 24bit
                let mut buffer: [u8; 4] = [0; 4];
                for i in 0..bytes_len {
                    buffer[i] = bytes[i];
                }
                let mut i32_val = i32::from_le_bytes(buffer);
                if i32_val > 0x7fffffi32 {
                    i32_val -= 0x1000000i32;
                }
                Ok(f64::from(i32_val) / f64::from(0x7fffffi32))
            } else if bytes_len == 4 {
                //signed 32bit
                let mut buffer: [u8; 4] = [0; 4];
                for i in 0..bytes_len {
                    buffer[i] = bytes[i];
                }
                Ok(f64::from(i32::from_le_bytes(buffer)) / f64::from(i32::MAX))
            } else {
                Err(WavF64VecError::new(WavF64VecErrorKind::BytesLengthError, None))
            }
        }
        WAVEFORMAT_ID_IEEE_FLOAT => {
            if bytes_len == 4 {
                //32bit float
                Ok(f64::from(f32::from_le_bytes(<[u8; 4]>::try_from(bytes)?)))
            } else {
                Err(WavF64VecError::new(WavF64VecErrorKind::BytesLengthError, None))
            }
        }
        _ => Err(WavF64VecError::new(
            WavF64VecErrorKind::FormatIsNotSupported,
            Some("format id".to_string()),
        )),
    }
}

/// Convert from a audio data value(`f64`) to a bytes data vector .
pub fn f64wave_to_bytes(format_id: usize, f64_val: f64, bits: usize) -> Result<Vec<u8>> {
    let bytes_len = bits / 8;
    match format_id {
        WAVEFORMAT_ID_PCM => {
            // TODO: Max Value Check for each length
            if bytes_len == 1 {
                let i8_val: i8;
                if f64_val < -1.0 {
                    i8_val = i8::MIN + 1;
                } else if 1.0 < f64_val {
                    i8_val = i8::MAX;
                } else {
                    i8_val = (f64_val * f64::from(i8::MAX)).round() as i8;
                }

                let mut buffer: [u8; 1] = i8_val.to_le_bytes();
                //to unsigned 8bit
                if buffer[0] < 128 {
                    buffer[0] += 128;
                } else {
                    buffer[0] -= 128;
                }
                Ok(buffer.to_vec())
            } else if bytes_len == 2 {
                let i16_val: i16;
                if f64_val < -1.0 {
                    i16_val = i16::MIN + 1;
                } else if 1.0 < f64_val {
                    i16_val = i16::MAX;
                } else {
                    i16_val = (f64_val * f64::from(i16::MAX)).round() as i16;
                }

                let buffer: [u8; 2] = i16_val.to_le_bytes();
                //signed 16bit
                Ok(buffer.to_vec())
            } else if bytes_len == 3 {
                let i32_val: i32;
                if f64_val < -1.0 {
                    i32_val = 0x800000i32 + 1;
                } else if 1.0 < f64_val {
                    i32_val = 0x7fffffi32;
                } else {
                    i32_val = (f64_val * f64::from(0x7fffffi32)).round() as i32;
                }

                let buffer: [u8; 4] = i32_val.to_le_bytes();
                //signed 24bit
                Ok(buffer[0..3].to_vec())
            } else if bytes_len == 4 {
                let i32_val: i32;
                if f64_val < -1.0 {
                    i32_val = i32::MIN + 1;
                } else if 1.0 < f64_val {
                    i32_val = i32::MAX;
                } else {
                    i32_val = (f64_val * f64::from(i32::MAX)).round() as i32;
                }

                let buffer: [u8; 4] = i32_val.to_le_bytes();
                //signed 32bit
                Ok(buffer.to_vec())
            } else {
                Err(WavF64VecError::new(WavF64VecErrorKind::BytesLengthError, None))
            }
        }
        WAVEFORMAT_ID_IEEE_FLOAT => {
            if bytes_len == 4 {
                let f32_val: f32;
                if f64_val < f32::MIN as f64 {
                    f32_val = f32::MIN;
                } else if (f32::MAX as f64) < f64_val {
                    f32_val = f32::MAX;
                } else {
                    f32_val = f64_val as f32;
                }
                let buffer: [u8; 4] = f32_val.to_le_bytes();
                //32bit float
                Ok(buffer.to_vec())
            } else {
                Err(WavF64VecError::new(WavF64VecErrorKind::BytesLengthError, None))
            }
        }
        _ => Err(WavF64VecError::new(
            WavF64VecErrorKind::FormatIsNotSupported,
            Some("format id".to_string()),
        )),
    }
}
