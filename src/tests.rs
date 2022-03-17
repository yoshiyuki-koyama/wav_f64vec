#[cfg(test)]
mod tests {
    use super::super::WavFile;
    use super::super::WaveFormat;
    use std::fs::{remove_file, File};
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn read_write_convert_test() {
        {
            // formatid:1 , bits per sample:8, channel:1, sampling rate:8000Hz,
            let half = (0x3Fi8) as f64 / (i8::MAX) as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 1.00, half, 0.00, -half, -1.00, -1.00, -half, 0.00]];
            let path_buf = create_test_file(1, 1, 8000, 8, &written_channel_vec);
            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x2F, 0x00, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x01, 0x00],
                &[0x40, 0x1F, 0x00, 0x00],
                &[0x40, 0x1F, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x08, 0x00],
                &[0x0B, 0x00, 0x00, 0x00],
                &[0x80, 0xBF, 0xFF, 0xFF, 0xBF, 0x80, 0x41, 0x01, 0x01, 0x41, 0x80],
            );
            check_contents(
                &path_buf,
                &vec_of_fmt_chunk,
                &vec_of_data_chunk,
                &vec_of_file,
                &expected_channel_vec,
            );
            remove_file(&path_buf).unwrap();
        }

        {
            // formatid:1 , bits per sample:16, channel:1, sampling rate:16000Hz,
            let half = (0x3FFFi16) as f64 / (i16::MAX) as f64;
            let written_channel_vec: Vec<Vec<f64>> = vec![
                vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00],
                vec![0.00, -half, -1.00, -2.00, -half, 0.00, half, 1.00, 2.00, half, 0.00],
            ];
            let expected_channel_vec: Vec<Vec<f64>> = vec![
                vec![0.00, half, 1.00, 1.00, half, 0.00, -half, -1.00, -1.00, -half, 0.00],
                vec![0.00, -half, -1.00, -1.00, -half, 0.00, half, 1.00, 1.00, half, 0.00],
            ];
            let path_buf = create_test_file(1, 2, 16000, 16, &written_channel_vec);

            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x50, 0x00, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x02, 0x00],
                &[0x80, 0x3E, 0x00, 0x00],
                &[0x00, 0xFA, 0x00, 0x00],
                &[0x04, 0x00],
                &[0x10, 0x00],
                &[0x2C, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0x00, 0xFF, 0x3F, 0x01, 0xC0, 0xFF, 0x7F, 0x01, 0x80, 0xFF, 0x7F, 0x01, 0x80, 0xFF, 0x3F,
                    0x01, 0xC0, 0x00, 0x00, 0x00, 0x00, 0x01, 0xC0, 0xFF, 0x3F, 0x01, 0x80, 0xFF, 0x7F, 0x01, 0x80, 0xFF, 0x7F,
                    0x01, 0xC0, 0xFF, 0x3F, 0x00, 0x00, 0x00, 0x00,
                ],
            );
            check_contents(
                &path_buf,
                &vec_of_fmt_chunk,
                &vec_of_data_chunk,
                &vec_of_file,
                &expected_channel_vec,
            );
            remove_file(&path_buf).unwrap();
        }

        {
            // formatid:1 , bits per sample:24, channel:1, sampling rate:22050Hz,
            let half = (0x3FFFFFi32) as f64 / (0x7FFFFFi32) as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 1.00, half, 0.00, -half, -1.00, -1.00, -half, 0.00]];
            let path_buf = create_test_file(1, 1, 22050, 24, &written_channel_vec);
            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x45, 0x00, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x01, 0x00],
                &[0x22, 0x56, 0x00, 0x00],
                &[0x66, 0x02, 0x01, 0x00],
                &[0x03, 0x00],
                &[0x18, 0x00],
                &[0x21, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0xFF, 0xFF, 0x3F, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0x3F, 0x00, 0x00, 0x00,
                    0x01, 0x00, 0xC0, 0x01, 0x00, 0x80, 0x01, 0x00, 0x80, 0x01, 0x00, 0xC0, 0x00, 0x00, 0x00,
                ],
            );
            check_contents(
                &path_buf,
                &vec_of_fmt_chunk,
                &vec_of_data_chunk,
                &vec_of_file,
                &expected_channel_vec,
            );
            remove_file(&path_buf).unwrap();
        }

        {
            // formatid:1 , bits per sample:32, channel:1, sampling rate:44100Hz,
            let half = (0x3FFFFFFFi32) as f64 / (i32::MAX) as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 1.00, half, 0.00, -half, -1.00, -1.00, -half, 0.00]];
            let path_buf = create_test_file(1, 1, 44100, 32, &written_channel_vec);
            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x50, 0x00, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x01, 0x00],
                &[0x44, 0xAC, 0x00, 0x00],
                &[0x10, 0xB1, 0x02, 0x00],
                &[0x04, 0x00],
                &[0x20, 0x00],
                &[0x2C, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0xFF, 0x3F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF,
                    0xFF, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xC0, 0x01, 0x00, 0x00, 0x80, 0x01, 0x00, 0x00, 0x80,
                    0x01, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00,
                ],
            );
            check_contents(
                &path_buf,
                &vec_of_fmt_chunk,
                &vec_of_data_chunk,
                &vec_of_file,
                &expected_channel_vec,
            );
            remove_file(&path_buf).unwrap();
        }

        {
            // formatid:3 , bits per sample:32, channel:1, sampling rate:192000Hz,
            let half = (0.5 as f32) as f64;
            let written_channel_vec: Vec<Vec<f64>> = vec![vec![
                0.00,
                half,
                1.00,
                f32::MAX as f64 + 1.0,
                half,
                0.00,
                -half,
                -1.00,
                f32::MIN as f64 - 1.0,
                -half,
                0.00,
            ]];
            let expected_channel_vec: Vec<Vec<f64>> = vec![vec![
                0.00,
                half,
                1.00,
                f32::MAX as f64,
                half,
                0.00,
                -half,
                -1.00,
                f32::MIN as f64,
                -half,
                0.00,
            ]];
            let path_buf = create_test_file(3, 1, 192000, 32, &written_channel_vec);
            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x50, 0x00, 0x00, 0x00],
                &[0x03, 0x00],
                &[0x01, 0x00],
                &[0x00, 0xEE, 0x02, 0x00],
                &[0x00, 0xB8, 0x0B, 0x00],
                &[0x04, 0x00],
                &[0x20, 0x00],
                &[0x2C, 0x00, 0x00, 0x00],
                &[
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x80, 0x3F, 0xFF, 0xFF, 0x7F, 0x7F, 0x00, 0x00,
                    0x00, 0x3F, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBF, 0x00, 0x00, 0x80, 0xBF, 0xFF, 0xFF, 0x7F, 0xFF,
                    0x00, 0x00, 0x00, 0xBF, 0x00, 0x00, 0x00, 0x00,
                ],
            );
            check_contents(
                &path_buf,
                &vec_of_fmt_chunk,
                &vec_of_data_chunk,
                &vec_of_file,
                &expected_channel_vec,
            );
            remove_file(&path_buf).unwrap();
        }
    }

    #[test]
    fn vec_order_test() {
        let channel_vec: Vec<Vec<f64>> = vec![
            vec![0.00, 0.50, 1.00, 0.50, 0.00, -0.50, -1.00, -0.50, 0.00],
            vec![0.00, -0.50, -1.00, -0.50, 0.00, 0.50, 1.00, 0.50, 0.00],
        ];

        let channel_vec_path = create_test_file_from_channel_vec(1, 2, 16000, 16, &channel_vec);

        let data_vec: Vec<Vec<f64>> = vec![
            vec![0.00, 0.00],
            vec![0.50, -0.50],
            vec![1.00, -1.00],
            vec![0.50, -0.50],
            vec![0.00, 0.00],
            vec![-0.50, 0.50],
            vec![-1.00, 1.00],
            vec![-0.50, 0.50],
            vec![0.00, 0.00],
        ];

        let data_vec_path = create_test_file_from_data_vec(1, 2, 16000, 16, &data_vec);

        {
            let channel_vec_file = File::open(channel_vec_path.clone()).unwrap();
            let data_vec_file = File::open(data_vec_path.clone()).unwrap();
            let mut channel_vec_buf = Vec::new();
            let mut data_vec_buf = Vec::new();
            BufReader::new(&channel_vec_file).read_to_end(&mut channel_vec_buf).unwrap();
            BufReader::new(&data_vec_file).read_to_end(&mut data_vec_buf).unwrap();
            assert_eq!(channel_vec_buf, data_vec_buf);
        }

        let mut channel_vec_wav_file = WavFile::new();
        let mut data_vec_wav_file = WavFile::new();
        channel_vec_wav_file.open(&channel_vec_path).unwrap();
        data_vec_wav_file.open(&data_vec_path).unwrap();
        assert_eq!(channel_vec_wav_file.sub_chunks, data_vec_wav_file.sub_chunks);

        let (channel_vec_format, channel_data_vec) = channel_vec_wav_file.get_channel_vec_audio().unwrap();
        let (data_vec_format, data_channel_vec) = data_vec_wav_file.get_data_vec_audio().unwrap();
        assert_eq!(channel_vec_format, data_vec_format);
        assert_eq!(channel_data_vec.len(), data_channel_vec[0].len());
        assert_eq!(channel_data_vec[0].len(), data_channel_vec.len());
        for channel_idx in 0..channel_data_vec.len() {
            for data_idx in 0..data_channel_vec.len() {
                assert_eq!(
                    channel_data_vec[channel_idx][data_idx],
                    data_channel_vec[data_idx][channel_idx]
                );
            }
        }
        remove_file(&channel_vec_path).unwrap();
        remove_file(&data_vec_path).unwrap();
    }

    fn create_test_file(id: usize, channel: usize, sampling_rate: usize, bits: usize, channel_vec: &Vec<Vec<f64>>) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };
        let mut wav_file = WavFile::new();
        wav_file.update_channel_vec_audio(&wave_format, &channel_vec).unwrap();
        let path_string = format!(
            "./test_id{}_{}ch_{}hz_{}bits.wav",
            wave_format.id, wave_format.channel, wave_format.sampling_rate, wave_format.bits
        );
        wav_file.save_as(Path::new(&path_string)).unwrap();
        PathBuf::from(&path_string)
    }

    fn create_test_file_from_channel_vec(
        id: usize,
        channel: usize,
        sampling_rate: usize,
        bits: usize,
        channel_vec: &Vec<Vec<f64>>,
    ) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };
        let mut wav_file = WavFile::new();
        wav_file.update_channel_vec_audio(&wave_format, &channel_vec).unwrap();
        wav_file.save_as(Path::new("./test_channel_vec.wav")).unwrap();
        PathBuf::from(&"./test_channel_vec.wav")
    }

    fn create_test_file_from_data_vec(
        id: usize,
        channel: usize,
        sampling_rate: usize,
        bits: usize,
        data_vec: &Vec<Vec<f64>>,
    ) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };

        let mut wav_file = WavFile::new();
        wav_file.update_data_vec_audio(&wave_format, &data_vec).unwrap();
        wav_file.save_as(Path::new("./test_data_vec.wav")).unwrap();
        PathBuf::from(&"./test_data_vec.wav")
    }

    fn create_u8_vecs(
        riff_chunk_size: &[u8],
        fmt_chunk_format_id: &[u8],
        fmt_chunk_channel: &[u8],
        fmt_chunk_sampling_rate: &[u8],
        fmt_chunk_bits: &[u8],
        fmt_chunk_block_size: &[u8],
        fmt_chunk_bit_rate: &[u8],
        data_chunk_size: &[u8],
        data_chunk_data: &[u8],
    ) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
        let riff_chunk_id: [u8; 4] = [0x52, 0x49, 0x46, 0x46];
        let riff_chunk_wave: [u8; 4] = [0x57, 0x41, 0x56, 0x45];
        let fmt_chunk_id: [u8; 4] = [0x66, 0x6D, 0x74, 0x20];
        let fmt_chunk_size: [u8; 4] = [0x10, 0x00, 0x00, 0x00];
        let data_chunk_id: [u8; 4] = [0x64, 0x61, 0x74, 0x61];

        let mut vec_of_fmt_chunk = fmt_chunk_format_id.to_vec();
        vec_of_fmt_chunk.append(&mut fmt_chunk_channel.to_vec());
        vec_of_fmt_chunk.append(&mut fmt_chunk_sampling_rate.to_vec());
        vec_of_fmt_chunk.append(&mut fmt_chunk_bits.to_vec());
        vec_of_fmt_chunk.append(&mut fmt_chunk_block_size.to_vec());
        vec_of_fmt_chunk.append(&mut fmt_chunk_bit_rate.to_vec());

        let vec_of_data_chunk = data_chunk_data.to_vec();

        let mut vec_of_file = riff_chunk_id.to_vec();
        vec_of_file.append(&mut riff_chunk_size.to_vec());
        vec_of_file.append(&mut riff_chunk_wave.to_vec());
        vec_of_file.append(&mut fmt_chunk_id.to_vec());
        vec_of_file.append(&mut fmt_chunk_size.to_vec());
        vec_of_file.append(&mut vec_of_fmt_chunk.clone());
        vec_of_file.append(&mut data_chunk_id.to_vec());
        vec_of_file.append(&mut data_chunk_size.to_vec());
        vec_of_file.append(&mut vec_of_data_chunk.clone());

        (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file)
    }

    fn check_contents(
        file_path: &Path,
        vec_of_fmt_chunk: &[u8],
        vec_of_data_chunk: &[u8],
        vec_of_file: &[u8],
        expected_channel_vec: &Vec<Vec<f64>>,
    ) {
        {
            // file check
            let target_file = File::open(file_path).unwrap();
            let mut buf = Vec::new();
            BufReader::new(&target_file).read_to_end(&mut buf).unwrap();
            dbg!("file");
            dbg!(&buf, &vec_of_file);
            assert_eq!(buf, vec_of_file);
        }

        // sub chunk check
        let mut wav_file = WavFile::new();
        wav_file.open(Path::new(file_path)).unwrap();

        let mut fmt_checked = false;
        let mut data_checked = false;
        for sub_chunk in &wav_file.sub_chunks {
            if sub_chunk.chunk_id == [b'f', b'm', b't', b' '] {
                fmt_checked = true;
                dbg!("fmt chunk");
                dbg!(&sub_chunk.bytes_data_vec, &vec_of_fmt_chunk);
                assert_eq!(sub_chunk.bytes_data_vec, vec_of_fmt_chunk);
            }
            if sub_chunk.chunk_id == [b'd', b'a', b't', b'a'] {
                data_checked = true;
                dbg!("data chunk");
                dbg!(&sub_chunk.bytes_data_vec, &vec_of_data_chunk);
                assert_eq!(sub_chunk.bytes_data_vec, vec_of_data_chunk);
            }
        }
        if !fmt_checked || !data_checked {
            panic!();
        }

        let (_, channel_data_vec) = wav_file.get_channel_vec_audio().unwrap();
        dbg!(&channel_data_vec, expected_channel_vec);
        assert_eq!(channel_data_vec, *expected_channel_vec);
    }
}
