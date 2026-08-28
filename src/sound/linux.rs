use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    SampleFormat, StreamConfig,
};
use std::{error::Error, f32::consts::TAU, time::Duration};

fn play_notification() -> Result<(), Box<dyn Error>> {
    let host = cpal::host_from_id(cpal::HostId::PipeWire)?;
    let device = host.default_output_device().ok_or("PipeWireの出力デバイスがないでーす")?;

    let config = device
        .supported_output_configs()?
        .find(|c| c.sample_format() == SampleFormat::F32)
        .ok_or("このデバイスはF32出力に対応していません")?
        .with_max_sample_rate();

    let config: StreamConfig = config.into();
    let sample_rate = config.sample_rate as f32;
    let channels = config.channels as usize;

    let mut phase = 0.0_f32;

    let stream = device.build_output_stream(
        config,
        move |data: &mut [f32], _| {
            for frame in data.chunks_mut(channels) {
                let sample = (phase * TAU).sin() * 0.2;

                phase = (phase + 1000.0 / sample_rate) % 1.0;

                // 全チャンネルへ同じ音を出す
                frame.fill(sample);
            }
        },
        |err| eprintln!("音声ストリームエラー: {err}"),
        None,
    )?;

    stream.play()?;
    std::thread::sleep(Duration::from_secs(3));

    Ok(())
}