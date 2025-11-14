use std::thread;
use std::time::Duration;
// Use direct FFI to Kernel32::Beep to avoid depending on windows-sys symbols
#[link(name = "Kernel32")]
unsafe extern "system" {
    fn Beep(dwFreq: u32, dwDuration: u32) -> i32;
}

use std::ffi::c_int;

/// 指定した周波数（Hz）と持続時間（ms）でビープを鳴らす
pub fn play_beep_once(frequency: u32, duration_ms: u32) {
    unsafe {
        // Beep は BOOL を返す（非ゼロで成功）
        let _ret: c_int = Beep(frequency, duration_ms);
        // ignore return value for now
    }
}

/// 成功/失敗に応じて音を鳴らす。短時間の非同期再生を行う。
pub fn play_beep(success: bool) {
    // 成功: 短い高音（1000Hz）を1回。失敗: 同じ高音を短く2回鳴らす（ピピッ音）
    if success {
        thread::spawn(move || {
            play_beep_once(1000, 80);
        });
        return;
    }

    // 失敗時: 成功と同じ周波数で短い音を2回鳴らす
    thread::spawn(move || {
        play_beep_once(700, 80);
        thread::sleep(Duration::from_millis(120));
        play_beep_once(700, 80);
    });
}
