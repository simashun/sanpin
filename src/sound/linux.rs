use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, OutputCallbackInfo, Sample, SampleFormat, SizedSample, StreamConfig, I24,
};
use std::{io, error::Error, f32::consts::TAU, time::Duration, thread, ffi::c_int};

/* 
pub fn play_beep2(success: bool) {
    if success {
        play_beep(true)?;
    } else {
        play_beep(false)?;
    }
    Ok(())
}
*/
pub fn play_beep_once(frequency: f32, duration_ms: u32){
    if let Err(err) = play_beep_once_innter(frequency, duration_ms) {
        eprintln!("ビープ音の再生に失敗しました: {err}");
    }
}

fn play_beep_once_innter(frequency: f32, duration_ms: u32) -> Result<(), Box<dyn Error>> {
    if frequency == 0.0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput, "周波数は0より大きくなければなりません").into());
    }
    if duration_ms == 0 {
        return Ok(());
    }
        // Linux標準のオーディオホストを使用する。
    // 通常はALSA。
    let host = cpal::default_host();

    let device = host.default_output_device().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            "デフォルトの音声出力デバイスが見つかりません",
        )
    })?;

    let supported_config = device.default_output_config()?;
    let sample_format = supported_config.sample_format();
    let config: StreamConfig = supported_config.into();

    // ソフトウェア生成なので、ナイキスト周波数以上は正しく再生できない。
    if frequency as f32 >= config.sample_rate as f32 / 2.0 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!(
                "frequencyが高すぎます: {frequency}Hz \
                 （現在のサンプルレート: {}Hz）",
                config.sample_rate
            ),
        )
        .into());
    }

    match sample_format {
        SampleFormat::I8 => {
            run_beep::<i8>(&device, config, frequency, duration_ms)
        }
        SampleFormat::I16 => {
            run_beep::<i16>(&device, config, frequency, duration_ms)
        }
        SampleFormat::I24 => {
            run_beep::<I24>(&device, config, frequency, duration_ms)
        }
        SampleFormat::I32 => {
            run_beep::<i32>(&device, config, frequency, duration_ms)
        }
        SampleFormat::I64 => {
            run_beep::<i64>(&device, config, frequency, duration_ms)
        }
        SampleFormat::U8 => {
            run_beep::<u8>(&device, config, frequency, duration_ms)
        }
        SampleFormat::U16 => {
            run_beep::<u16>(&device, config, frequency, duration_ms)
        }
        SampleFormat::U32 => {
            run_beep::<u32>(&device, config, frequency, duration_ms)
        }
        SampleFormat::U64 => {
            run_beep::<u64>(&device, config, frequency, duration_ms)
        }
        SampleFormat::F32 => {
            run_beep::<f32>(&device, config, frequency, duration_ms)
        }
        SampleFormat::F64 => {
            run_beep::<f64>(&device, config, frequency, duration_ms)
        }
        unsupported => Err(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("未対応のサンプル形式です: {unsupported}"),
        )
        .into()),
    }
}

fn run_beep<T>(
    device: &Device,
    config: StreamConfig,
    frequency: f32,
    duration_ms: u32,
) -> Result<(), Box<dyn Error>>
where
    T: SizedSample + FromSample<f32>,
{
    let sample_rate = config.sample_rate as f32;
    let channels = config.channels as usize;
    let phase_step = TAU * frequency as f32 / sample_rate;

    let mut phase = 0.0_f32;

    // 最大音量では大きすぎるため20%に抑える。
    let mut next_sample = move || {
        let value = phase.sin() * 0.20;

        phase += phase_step;
        if phase >= TAU {
            phase -= TAU;
        }

        value
    };

    let stream = device.build_output_stream(
        config,
	 move |data: &mut [T], _: &OutputCallbackInfo| {
            write_data(data, channels, &mut next_sample);
        },
        |err| {
            eprintln!("音声ストリームエラー: {err}");
        },
        None,
    )?;

    stream.play()?;

    // streamが存在している間だけ音が再生される。
    thread::sleep(Duration::from_millis(duration_ms as u64));

    Ok(())
}

fn write_data<T>(
    output: &mut [T],
    channels: usize,
    next_sample: &mut dyn FnMut() -> f32,
) where
    T: Sample + FromSample<f32>,
{
    for frame in output.chunks_mut(channels) {
        let value = T::from_sample(next_sample());

        // ステレオなど、すべてのチャンネルへ同じ波形を出力する。
        for sample in frame {
            *sample = value;
        }
    }
}

pub fn play_beep(bool: bool) -> Result<(), Box<dyn Error>> {
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