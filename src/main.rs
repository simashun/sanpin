use std::net::Ipv4Addr;
use std::ptr::null_mut;
use std::mem;

use windows_sys::Win32::Foundation::{HANDLE, NO_ERROR};
use windows_sys::Win32::NetworkManagement::IpHelper::{
    IcmpCreateFile, IcmpCloseHandle, IcmpSendEcho,
    ICMP_ECHO_REPLY, IP_OPTION_INFORMATION
};

// --- 定数と設定 ---
// 送信するデータサイズ (32バイトが一般的)
const DATA_SIZE: usize = 32;
// エコー応答構造体とデータを格納するためのバッファサイズ
// 一般的な安全なサイズ: sizeof(ICMP_ECHO_REPLY) + DATA_SIZE
const REPLY_BUFFER_SIZE: usize = mem::size_of::<ICMP_ECHO_REPLY>() + DATA_SIZE;
// タイムアウト時間 (ミリ秒)
const TIMEOUT_MS: u32 = 4000;

fn main() -> Result<(), String> {
    let target_ip_str = "8.8.8.8"; // テスト用のGoogle Public DNS
    let target_ip: Ipv4Addr = target_ip_str.parse().map_err(|_| "無効なIPアドレス形式")?;

    println!("{} へ ping を実行しています...", target_ip_str);

    // 1. ICMPハンドルを作成
    let icmp_handle: HANDLE = unsafe { IcmpCreateFile() };
    if icmp_handle == 0 {
        return Err("IcmpCreateFileの呼び出しに失敗しました。".to_string());
    }

    // 2. データを準備
    // ICMPパケットに含める送信データ (全て0の32バイト)
    let send_buffer: [u8; DATA_SIZE] = [0; DATA_SIZE];
    
    // 応答データを受け取るためのバッファ
    // ICMP_ECHO_REPLY構造体 + 送信データサイズを格納できる大きさが必要
    let mut reply_buffer_raw = vec![0u8; REPLY_BUFFER_SIZE];
    
    // IPオプション情報 (TTLなどを設定しない場合は全て0)
    let ip_options = IP_OPTION_INFORMATION {
        Ttl: 128, // Time To Live (TTL)
        Tos: 0,
        Flags: 0,
        OptionsSize: 0,
        OptionsData: null_mut(),
    };

    // ターゲットIPアドレスをビッグエンディアンのu32で取得し、ネットワークバイト順のu32に変換 (WinAPIはホストバイト順を期待)
    let ip_bytes = target_ip.octets();
    let target_ip_u32 = u32::from_ne_bytes(ip_bytes);

    // 3. IcmpSendEchoを呼び出し
    let num_replies = unsafe {
        IcmpSendEcho(
            icmp_handle,                                // ICMPハンドル
            target_ip_u32,                              // ターゲットIPアドレス (u32)
            send_buffer.as_ptr() as *const _,           // 送信データバッファのポインタ
            DATA_SIZE as u16,                           // 送信データサイズ
            &ip_options,                                // IPオプションのポインタ
            reply_buffer_raw.as_mut_ptr() as *mut _,    // 応答バッファのポインタ
            REPLY_BUFFER_SIZE as u32,                   // 応答バッファサイズ
            TIMEOUT_MS,                                 // タイムアウト (ms)
        )
    };

    // 4. ICMPハンドルを閉じる (重要)
    unsafe { IcmpCloseHandle(icmp_handle) };

    // 5. 結果の処理
    if num_replies == 0 {
        // エラーコードを確認 (GetLastErrorはIcmpSendEchoでは利用できないことが多い)
        // IP_STATUSをチェックするために、応答バッファの先頭をICMP_ECHO_REPLYとして解釈する
        let reply = reply_buffer_raw.as_ptr() as *const ICMP_ECHO_REPLY;
        let status = unsafe { (*reply).Status };

        match status {
            30_054 => println!("要求がタイムアウトしました。"), // IP_REQ_TIMED_OUT
            30_051 => println!("宛先ホストに到達できません。"), // IP_DEST_HOST_UNREACHABLE
            _ => println!("応答がありません。Status: {}", status),
        }
    } else {
        // 成功: 応答バッファの先頭にあるICMP_ECHO_REPLY構造体を読み取る
        let reply = reply_buffer_raw.as_ptr() as *const ICMP_ECHO_REPLY;
        let round_trip_time = unsafe { (*reply).RoundTripTime };

        println!("応答: {} から", target_ip_str);
        println!("バイト: {}", unsafe { (*reply).DataSize });
        println!("時間: {}ms", round_trip_time);
        println!("TTL: {}", unsafe { (*reply).Options.Ttl });
    }

    Ok(())
}
