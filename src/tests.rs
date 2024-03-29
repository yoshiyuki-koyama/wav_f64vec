#[cfg(test)]
mod tests {
    use crate::SubChunk;

    use super::super::error::*;
    use super::super::WavFile;
    use super::super::WaveFormat;
    use super::super::{convert_sampling_rate_for_channel_data_vec, convert_sampling_rate_for_data_channel_vec};
    use super::super::{BIT16_WAVE_DIVISOR, BIT24_WAVE_DIVISOR, BIT32_WAVE_DIVISOR, BIT8_WAVE_DIVISOR};
    use std::fs::{remove_file, File};
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn read_write_convert_test() {
        {
            // formatid:1 , bits per sample:8, channel:1, sampling rate:8000Hz,
            let half = (0x40i8) as f64 / BIT8_WAVE_DIVISOR as f64;
            let max_plus = i8::MAX as f64 / BIT8_WAVE_DIVISOR as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> = vec![vec![
                0.00, half, max_plus, max_plus, half, 0.00, -half, -1.00, -1.00, -half, 0.00,
            ]];
            let path_buf = create_test_file(1, 1, 8000, 8, &written_channel_vec);
            #[rustfmt::skip]
            let (vec_of_fmt_chunk, vec_of_data_chunk, vec_of_file) = create_u8_vecs(
                &[0x2F, 0x00, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x01, 0x00],
                &[0x40, 0x1F, 0x00, 0x00],
                &[0x40, 0x1F, 0x00, 0x00],
                &[0x01, 0x00],
                &[0x08, 0x00],
                &[0x0B, 0x00, 0x00, 0x00],
                &[
                    0x80, 0xC0, 0xFF,
                    0xFF, 0xC0, 0x80, 0x40, 0x00, 0x00, 0x40, 0x80,
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
            // formatid:1 , bits per sample:16, channel:1, sampling rate:16000Hz,
            let half = (0x4000i16) as f64 / BIT16_WAVE_DIVISOR as f64;
            let max_plus = i16::MAX as f64 / BIT16_WAVE_DIVISOR as f64;
            let written_channel_vec: Vec<Vec<f64>> = vec![
                vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00],
                vec![0.00, -half, -1.00, -2.00, -half, 0.00, half, 1.00, 2.00, half, 0.00],
            ];
            let expected_channel_vec: Vec<Vec<f64>> = vec![
                vec![0.00, half, max_plus, max_plus, half, 0.00, -half, -1.00, -1.00, -half, 0.00],
                vec![0.00, -half, -1.00, -1.00, -half, 0.00, half, max_plus, max_plus, half, 0.00],
            ];
            let path_buf = create_test_file(1, 2, 16000, 16, &written_channel_vec);
            #[rustfmt::skip]
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
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x40, 0x00, 0xC0, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0x00, 0x40, 0x00, 0xC0,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x40, 0x00, 0x80, 0xFF, 0x7F, 0x00, 0x80, 0xFF, 0x7F,
                    0x00, 0xC0, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00,
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
            let half = (0x400000i32) as f64 / BIT24_WAVE_DIVISOR as f64;
            let max_plus = (BIT24_WAVE_DIVISOR - 1) as f64 / BIT24_WAVE_DIVISOR as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> = vec![vec![
                0.00, half, max_plus, max_plus, half, 0.00, -half, -1.00, -1.00, -half, 0.00,
            ]];
            let path_buf = create_test_file(1, 1, 22050, 24, &written_channel_vec);
            #[rustfmt::skip]
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
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x40, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0xC0, 0x00, 0x00, 0x80, 0x00, 0x00, 0x80, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00,
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
            let half = (0x40000000i32) as f64 / BIT32_WAVE_DIVISOR as f64;
            let max_plus = i32::MAX as f64 / BIT32_WAVE_DIVISOR as f64;
            let written_channel_vec: Vec<Vec<f64>> =
                vec![vec![0.00, half, 1.00, 2.00, half, 0.00, -half, -1.00, -2.00, -half, 0.00]];
            let expected_channel_vec: Vec<Vec<f64>> = vec![vec![
                0.00, half, max_plus, max_plus, half, 0.00, -half, -1.00, -1.00, -half, 0.00,
            ]];
            let path_buf = create_test_file(1, 1, 44100, 32, &written_channel_vec);
            #[rustfmt::skip]
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
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x40, 0xFF, 0xFF, 0xFF, 0x7F, 0xFF, 0xFF, 0xFF, 0x7F, 0x00, 0x00, 0x00, 0x40,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x80,
                    0x00, 0x00, 0x00, 0xC0, 0x00, 0x00, 0x00, 0x00,
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
            #[rustfmt::skip]
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
                    0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0x00, 0x3F, 0x00, 0x00, 0x80, 0x3F, 0xFF, 0xFF, 0x7F, 0x7F, 0x00, 0x00, 0x00, 0x3F,
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xBF, 0x00, 0x00, 0x80, 0xBF, 0xFF, 0xFF, 0x7F, 0xFF,
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
        #[rustfmt::skip]
        let channel_data_vec: Vec<Vec<f64>> = vec![
            vec![0.00, 0.50, 1.00, 0.50, 0.00, -0.50, -1.00, -0.50, 0.00],
            vec![0.00, -0.50, -1.00, -0.50, 0.00, 0.50, 1.00, 0.50, 0.00],
        ];

        let channel_vec_path = create_test_file_from_channel_vec(1, 2, 16000, 16, &channel_data_vec);
        #[rustfmt::skip]
        let data_channel_vec: Vec<Vec<f64>> = vec![
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

        let data_vec_path = create_test_file_from_data_vec(1, 2, 16000, 16, &data_channel_vec);

        {
            let channel_vec_file = File::open(channel_vec_path.clone()).unwrap();
            let data_vec_file = File::open(data_vec_path.clone()).unwrap();
            let mut channel_vec_buf = Vec::new();
            let mut data_vec_buf = Vec::new();
            BufReader::new(&channel_vec_file).read_to_end(&mut channel_vec_buf).unwrap();
            BufReader::new(&data_vec_file).read_to_end(&mut data_vec_buf).unwrap();
            assert_eq!(channel_vec_buf, data_vec_buf);
        }

        let channel_vec_wav_file = WavFile::open(&channel_vec_path).unwrap();
        let data_vec_wav_file = WavFile::open(&data_vec_path).unwrap();

        assert_eq!(channel_vec_wav_file.sub_chunks, data_vec_wav_file.sub_chunks);

        let (channel_vec_format, channel_data_vec) = channel_vec_wav_file.get_audio_for_channel_data_vec().unwrap();
        let (data_vec_format, data_channel_vec) = data_vec_wav_file.get_audio_for_data_channel_vec().unwrap();
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

    #[test]
    fn convert_sampling_rate_test() {
        // 33200 to 48000
        let dst_data48k = |dst_data_idx, src_data1, src_data2| {
            let fraction = if (dst_data_idx * 32000 % 48000) == 0 {
                0.0
            } else {
                ((dst_data_idx * 32000) as f64 / 48000 as f64).fract()
            };
            src_data1 * (1.0 - fraction) + src_data2 * fraction
        };
        // 33200 to 22050
        let dst_data22k = |dst_data_idx, src_data1, src_data2| {
            let fraction = if (dst_data_idx * 32000 % 22050) == 0 {
                0.0
            } else {
                ((dst_data_idx * 32000) as f64 / 22050 as f64).fract()
            };
            src_data1 * (1.0 - fraction) + src_data2 * fraction
        };

        {
            #[rustfmt::skip]
            let channel_data_32000_vec: Vec<Vec<f64>> = vec![
                vec![0.00, 0.50, 1.00, 0.50, 0.00, -0.50, -1.00, -0.50, 0.00],
                vec![0.00, -0.50, -1.00, -0.50, 0.00, 0.50, 1.00, 0.50, 0.00],
            ];

            let new_channel_data_vec = convert_sampling_rate_for_channel_data_vec(&channel_data_32000_vec, 32000, 16000).unwrap();
            #[rustfmt::skip]
            let channel_data_16000_vec: Vec<Vec<f64>> = vec![
                vec![0.00, 1.00, 0.00, -1.00, 0.00],
                vec![0.00, -1.00, 0.00, 1.00, 0.00],
            ];
            assert_eq!(new_channel_data_vec, channel_data_16000_vec);
            let new_channel_data_vec = convert_sampling_rate_for_channel_data_vec(&channel_data_32000_vec, 32000, 48000).unwrap();
            #[rustfmt::skip]
            let channel_data_48000_vec: Vec<Vec<f64>> = vec![
                vec![0.00, dst_data48k(1, 0.00, 0.50), dst_data48k(2, 0.50, 1.00), 1.00, dst_data48k(4, 1.00, 0.50), dst_data48k(5, 0.50, 0.00), 0.00, dst_data48k(7, 0.00, -0.50), dst_data48k(8, -0.50, -1.00), -1.00, dst_data48k(10, -1.00, -0.50), dst_data48k(11, -0.50, 0.00), 0.00, 0.00],
                vec![0.00, dst_data48k(1, 0.00, -0.50), dst_data48k(2, -0.50, -1.00), -1.00, dst_data48k(4, -1.00, -0.50), dst_data48k(5, -0.50, 0.00), 0.00, dst_data48k(7, 0.00, 0.50), dst_data48k(8, 0.50, 1.00), 1.00, dst_data48k(10, 1.00, 0.50), dst_data48k(11, 0.50, 0.00), 0.00, 0.00],
            ];
            assert_eq!(new_channel_data_vec, channel_data_48000_vec);

            let new_channel_data_vec = convert_sampling_rate_for_channel_data_vec(&channel_data_32000_vec, 32000, 22050).unwrap();
            #[rustfmt::skip]
            let channel_data_22500_vec: Vec<Vec<f64>> = vec![
                vec![0.00, dst_data22k(1, 0.50, 1.00),  dst_data22k(2, 1.00, 0.50), dst_data22k(3, 0.00, -0.50), dst_data22k(4, -0.50, -1.00), dst_data22k(5, -0.50, 0.00), dst_data22k(6, 0.00, 0.00)],
                vec![0.00, dst_data22k(1, -0.50, -1.00),  dst_data22k(2, -1.00, -0.50), dst_data22k(3, 0.00, 0.50), dst_data22k(4, 0.50, 1.00), dst_data22k(5, 0.50, 0.00), dst_data22k(6, 0.00, 0.00)],
            ];
            assert_eq!(new_channel_data_vec, channel_data_22500_vec);
        }

        {
            #[rustfmt::skip]
            let data_channel_32000_vec: Vec<Vec<f64>> = vec![
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

            let new_channel_data_vec = convert_sampling_rate_for_data_channel_vec(&data_channel_32000_vec, 32000, 16000).unwrap();
            #[rustfmt::skip]
            let channel_data_16000_vec: Vec<Vec<f64>> = vec![
                vec![0.00, 0.00],
                vec![1.00, -1.00],
                vec![0.00, 0.00],
                vec![-1.00, 1.00],
                vec![0.00, 0.00],
            ];
            assert_eq!(new_channel_data_vec, channel_data_16000_vec);
            let new_channel_data_vec = convert_sampling_rate_for_data_channel_vec(&data_channel_32000_vec, 32000, 48000).unwrap();
            #[rustfmt::skip]
            let channel_data_48000_vec: Vec<Vec<f64>> = vec![
                vec![0.00, 0.00],
                vec![dst_data48k(1, 0.00, 0.50), dst_data48k(1, 0.00, -0.50)],
                vec![dst_data48k(2, 0.50, 1.00), dst_data48k(2, -0.50, -1.00)],
                vec![1.00, -1.00],
                vec![dst_data48k(4, 1.00, 0.50), dst_data48k(4, -1.00, -0.50)],
                vec![dst_data48k(5, 0.50, 0.00), dst_data48k(5, -0.50, 0.00)],
                vec![0.00, 0.00],
                vec![dst_data48k(7, 0.00, -0.50), dst_data48k(7, 0.00, 0.50)],
                vec![dst_data48k(8, -0.50, -1.00), dst_data48k(8, 0.50, 1.00)],
                vec![-1.00, 1.00],
                vec![dst_data48k(10, -1.00, -0.50), dst_data48k(10, 1.00, 0.50)],
                vec![dst_data48k(11, -0.50, 0.00), dst_data48k(11, 0.50, 0.00)],
                vec![0.00, 0.00],
                vec![0.00, 0.00],
            ];
            assert_eq!(new_channel_data_vec, channel_data_48000_vec);

            let new_channel_data_vec = convert_sampling_rate_for_data_channel_vec(&data_channel_32000_vec, 32000, 22050).unwrap();
            #[rustfmt::skip]
            let channel_data_22500_vec: Vec<Vec<f64>> = vec![
                vec![0.00, 0.00],
                vec![dst_data22k(1, 0.50, 1.00), dst_data22k(1, -0.50, -1.00)],
                vec![dst_data22k(2, 1.00, 0.50), dst_data22k(2, -1.00, -0.50)],
                vec![dst_data22k(3, 0.00, -0.50), dst_data22k(3, 0.00, 0.50)],
                vec![dst_data22k(4, -0.50, -1.00), dst_data22k(4, 0.50, 1.00)],
                vec![dst_data22k(5, -0.50, 0.00), dst_data22k(5, 0.50, 0.00)],
                vec![dst_data22k(6, 0.00, 0.00), dst_data22k(6, 0.00, 0.00)],
            ];
            assert_eq!(new_channel_data_vec, channel_data_22500_vec);
        }
    }

    #[test]
    fn size_check_test() {
        // 12 = "RIFF" + RIFF Size + "WAVE"
        // 8 = junk chunk_id + body_size
        // 24 = "fmt" chunk size
        // 8 = data chunk_id + body_size
        // 1 = audio data
        let junk_chunk = SubChunk {
            chunk_id: [b'J', b'U', b'N', b'K'],
            bytes_data_vec: vec![0x00; 0xffffffff - 12 - 8 - 24 - 8 - 1],
        };

        let wave_format = WaveFormat {
            id: 1,
            channel: 1,
            sampling_rate: 8000,
            bits: 8,
        };

        let mut channel_data_vec: Vec<Vec<f64>> = vec![vec![0.0]];

        let mut wav_file = WavFile::new();
        wav_file.update_sub_chunk(junk_chunk).unwrap();
        // Ok
        wav_file
            .update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)
            .unwrap();
        wav_file.save_as(Path::new("./large_files.wav")).unwrap();
        remove_file(Path::new("./large_files.wav")).unwrap();
        // Err
        channel_data_vec[0].push(0.0);
        let result = wav_file.update_audio_for_channel_data_vec(&wave_format, &channel_data_vec);
        match result {
            Ok(_) => {
                panic!();
            }
            Err(err) => {
                if let Some(wav_err) = err.downcast_ref::<WavF64VecError>() {
                    if wav_err.err_kind == WavF64VecErrorKind::SubChunkSizeTooLarge {
                        if wav_err.op_additional_message.as_ref().unwrap() != "data" {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            }
        }

        // Err
        wav_file.sub_chunks[0].bytes_data_vec.push(0);
        let result = wav_file.save_as(Path::new("./too_large_files.wav"));
        match result {
            Ok(_) => {
                panic!();
            }
            Err(err) => {
                if let Some(wav_err) = err.downcast_ref::<WavF64VecError>() {
                    if wav_err.err_kind == WavF64VecErrorKind::SubChunkSizeTooLarge {
                        if wav_err.op_additional_message.is_some() {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            }
        }

        // Err
        for _ in 0..23 {
            wav_file.sub_chunks[0].bytes_data_vec.push(0);
        }
        let result = wav_file.update_audio_for_channel_data_vec(&wave_format, &channel_data_vec);
        match result {
            Ok(_) => {
                panic!();
            }
            Err(err) => {
                if let Some(wav_err) = err.downcast_ref::<WavF64VecError>() {
                    if wav_err.err_kind == WavF64VecErrorKind::SubChunkSizeTooLarge {
                        if wav_err.op_additional_message.as_ref().unwrap() != "fmt " {
                            panic!();
                        }
                    } else {
                        panic!();
                    }
                } else {
                    panic!();
                }
            }
        }
    }

    #[test]
    fn sub_chunk_util_test() {
        let mut wav_file = WavFile::new();

        // update_sub_chunk
        let abcd_chunk = SubChunk {
            chunk_id: [b'a', b'b', b'c', b'd'],
            bytes_data_vec: vec![0x00, 0x01, 0x02, 0x03],
        };
        let efgh_chunk = SubChunk {
            chunk_id: [b'e', b'f', b'g', b'h'],
            bytes_data_vec: vec![0x04, 0x05, 0x06, 0x07],
        };
        let ijkl_chunk = SubChunk {
            chunk_id: [b'i', b'j', b'k', b'l'],
            bytes_data_vec: vec![0x08, 0x09, 0x0A, 0x0B],
        };

        wav_file.update_sub_chunk(abcd_chunk.clone()).unwrap();
        wav_file.update_sub_chunk(efgh_chunk.clone()).unwrap();
        wav_file.update_sub_chunk(ijkl_chunk.clone()).unwrap();

        assert_eq!(wav_file.sub_chunks[0], abcd_chunk);
        assert_eq!(wav_file.sub_chunks[1], efgh_chunk);
        assert_eq!(wav_file.sub_chunks[2], ijkl_chunk);

        assert_eq!(
            wav_file.get_sub_chunk_id_vec(),
            vec![abcd_chunk.chunk_id, efgh_chunk.chunk_id, ijkl_chunk.chunk_id]
        );

        // delete_sub_chunk
        assert_eq!(wav_file.delete_sub_chunk(efgh_chunk.chunk_id), true);
        assert_eq!(wav_file.delete_sub_chunk(efgh_chunk.chunk_id), false);
        assert_eq!(wav_file.sub_chunks[0], abcd_chunk);
        assert_eq!(wav_file.sub_chunks[1], ijkl_chunk);

        // get_sub_chunk_idx
        assert_eq!(wav_file.get_sub_chunk_idx(ijkl_chunk.chunk_id).unwrap(), 1);
        assert_eq!(wav_file.get_sub_chunk_idx(efgh_chunk.chunk_id), None);

        // wave format
        assert_eq!(wav_file.get_format().unwrap(), None);
        let wave_format = WaveFormat {
            id: 1,
            channel: 1,
            sampling_rate: 8000,
            bits: 8,
        };
        let channel_data_vec: Vec<Vec<f64>> = vec![vec![0.00]];
        wav_file
            .update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)
            .unwrap();
        assert_eq!(wav_file.get_format().unwrap().unwrap(), wave_format);
    }

    fn create_test_file(
        id: usize,
        channel: usize,
        sampling_rate: usize,
        bits: usize,
        channel_data_vec: &Vec<Vec<f64>>,
    ) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };
        let mut wav_file = WavFile::new();
        wav_file
            .update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)
            .unwrap();
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
        channel_data_vec: &Vec<Vec<f64>>,
    ) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };
        let mut wav_file = WavFile::new();
        wav_file
            .update_audio_for_channel_data_vec(&wave_format, &channel_data_vec)
            .unwrap();
        wav_file.save_as(Path::new("./test_channel_vec.wav")).unwrap();
        PathBuf::from(&"./test_channel_vec.wav")
    }

    fn create_test_file_from_data_vec(
        id: usize,
        channel: usize,
        sampling_rate: usize,
        bits: usize,
        data_channel_vec: &Vec<Vec<f64>>,
    ) -> PathBuf {
        let wave_format = WaveFormat {
            id: id,
            channel: channel,
            sampling_rate: sampling_rate,
            bits: bits,
        };

        let mut wav_file = WavFile::new();
        wav_file
            .update_audio_for_data_channel_vec(&wave_format, &data_channel_vec)
            .unwrap();
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
            assert_eq!(buf, vec_of_file);
        }

        // sub chunk check
        let wav_file = WavFile::open(Path::new(file_path)).unwrap();

        let mut fmt_checked = false;
        let mut data_checked = false;
        for sub_chunk in &wav_file.sub_chunks {
            if sub_chunk.chunk_id == [b'f', b'm', b't', b' '] {
                fmt_checked = true;
                assert_eq!(sub_chunk.bytes_data_vec, vec_of_fmt_chunk);
            }
            if sub_chunk.chunk_id == [b'd', b'a', b't', b'a'] {
                data_checked = true;
                assert_eq!(sub_chunk.bytes_data_vec, vec_of_data_chunk);
            }
        }
        if !fmt_checked || !data_checked {
            panic!();
        }

        let (_, channel_data_vec) = wav_file.get_audio_for_channel_data_vec().unwrap();
        assert_eq!(channel_data_vec, *expected_channel_vec);
    }
}
