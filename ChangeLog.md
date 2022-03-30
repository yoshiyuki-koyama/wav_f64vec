# ChangeLog
## v0.4.0
* Remove file extension(.wav) check
* Relax sampling rate condition.
## v0.3.0
* Rename methods of `Wavfile` structure.
    * `get_channel_vec_audio()` to `get_audio_for_channel_data_vec()`.
    * `get_data_vec_audio()` to `get_audio_for_data_channel_vec()`.
    * `update_channel_vec_audio()` to `update_audio_for_channel_data_vec()`.
    * `update_data_vec_audio()` to `update_audio_for_data_channel_vec()`.

* Add change sampling rate functions.
    * Add `convert_sampling_rate_for_channel_data_vec()`.
    * Add `convert_sampling_rate_for_data_channel_vec()`.
## v0.2.0
* `open()` of `WavFile` structure changed from a method to a associated function that makes `WavFile` strucure.
* Added some methods to `WavFile` structure.
* `body_size` field of `Subchunk` structure was removed.
* Some minor change.
## v0.1.0
* the first version.
