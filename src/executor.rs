use crate::cli::Args;
use crate::sound::play_beep;
use encoding_rs::SHIFT_JIS;
use std::io::{BufRead, BufReader, Read};
use std::process::{Command, Stdio};
use std::time::Instant;

#[allow(dead_code)]
pub fn run_ping(args: &Args) -> Result<String, String> {
    if args.continuous {
        return Err(
            "Continuous (-t) mode is only supported in realtime mode. Use the realtime command."
                .to_string(),
        );
    }
    let output = Command::new("ping")
        .arg(&args.target)
        .arg("-n")
        .arg(args.count.to_string())
        .arg("-w")
        .arg(args.timeout.to_string())
        .output(); //←これで終わるとping終了まで待つ

    match output {
        Ok(output) => {
            if output.status.success() {
                let (decoded_output, _encoding, _malformed) = SHIFT_JIS.decode(&output.stdout);
                Ok(decoded_output.into_owned())
            } else {
                let (decoded_stderr, _encoding, _malformed) = SHIFT_JIS.decode(&output.stderr);
                Err(format!(
                    "Pingコマンドがエラーで終了しました:\n{}",
                    decoded_stderr.into_owned()
                ))
            }
        }
        Err(e) => Err(format!("Pingコマンドの実行に失敗しました: {}", e)),
    }
}

pub fn run_ping_realtime(args: &Args) -> Result<String, String> {
    let mut command = Command::new("ping");
    // always set target
    command.arg(&args.target);

    // Continuous mode (-t) on Windows is used without a count; if continuous is set, add -t
    if args.continuous {
        command.arg("-t");
    } else {
        command.arg("-n").arg(args.count.to_string());
    }

    // timeout and pipes
    command
        .arg("-w")
        .arg(args.timeout.to_string())
        //↓realtime
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = match command.spawn() {
        Ok(c) => c,
        Err(e) => return Err(format!("Pingコマンドの実行に失敗しました： {}", e)),
    };

    // stdout パイプからリアルタイムに読み取る（バイト単位で読み、Shift-JIS でデコードする）
    let stdout = child
        .stdout
        .take()
        .expect("stdoutパイプにアクセスできませんでした");
    let mut reader = BufReader::new(stdout);

    let mut full_output = String::new();
    let mut buf: Vec<u8> = Vec::new();

    // ビープ用のデバウンス用タイムスタンプ
    let mut last_beep = Instant::now() - std::time::Duration::from_secs(1);

    // read_until を使って改行ごとにバイト列を取得し、Shift-JIS としてデコードする
    loop {
        buf.clear();
        match reader.read_until(b'\n', &mut buf) {
            Ok(0) => break, // EOF
            Ok(_) => {
                // buf は改行を含む可能性がある。decode に渡すのはバイト列。
                let (decoded_line, _encoding, malformed) = SHIFT_JIS.decode(&buf);

                if malformed {
                    eprintln!("出力に不正なバイト列が含まれていたため置換文字が使われました");
                }

                // borrow して表示・蓄積（無駄なコピーを避ける）
                let output_line: &str = decoded_line.as_ref();
                print!("{}", output_line); // 既に末尾に改行が含まれるため println! ではなく print! を使う

                // ビープ機能: デフォルトでビープを行う。ビープを無効にするには `--nonbeep` を指定します
                if !args.nonbeep {
                    // 成功/失敗の判定パターン（英語・日本語両対応）
                    let success_patterns = [
                        "Reply from",
                        "TTL=",
                        "バイト数 =",
                        "時間 =",
                    ];
                    let failure_patterns = [
                        "Request timed out",
                        "要求がタイムアウト",
                        "宛先ホストに到達できません",
                        "送信先のホストに到達できません",
                        "一般エラー",
                        "一般エラー。",
                        "Destination host unreachable",
                        "Destination net unreachable",
                    ];

                    let mut matched_success = false;
                    for p in success_patterns {
                        if output_line.contains(p) {
                            matched_success = true;
                            break;
                        }
                    }

                    let mut matched_failure = false;
                    for p in failure_patterns {
                        if output_line.contains(p) {
                            matched_failure = true;
                            break;
                        }
                    }

                    // 最短間隔 150ms
                    let min_interval = std::time::Duration::from_millis(150);
                    if last_beep.elapsed() >= min_interval {
                        if matched_success {
                            play_beep(true);
                            last_beep = Instant::now();
                        } else if matched_failure {
                            play_beep(false);
                            last_beep = Instant::now();
                        }
                    }
                }

                full_output.push_str(output_line);
                // read_until が改行を含めて返すため、full_output に改行が既に含まれていることが多いが
                // 万一末尾に改行が無ければ追加する
                if !full_output.ends_with('\n') {
                    full_output.push('\n');
                }
            }
            Err(e) => {
                eprintln!("出力を読み取る際にエラーが発生しました: {}", e);
                break;
            }
        }
    }

    // プロセスが終了するのを待つ
    let status = child.wait().expect("子プロセスが待機中に失敗しました");

    if status.success() {
        Ok(full_output)
    } else {
        // 標準エラー出力の処理 (今回は標準出力のみでping結果が完結することが多いが、念のため)
        let mut stderr = child
            .stderr
            .take()
            .expect("標準エラー出力パイプにアクセスできませんでした");
        let mut err_buf: Vec<u8> = Vec::new();
        match stderr.read_to_end(&mut err_buf) {
            Ok(_) => {
                let (decoded_stderr_cow, _encoding, malformed) = SHIFT_JIS.decode(&err_buf);
                if malformed {
                    eprintln!(
                        "標準エラー出力に不正なバイト列が含まれていました（置換文字が使用されました）"
                    );
                }

                // 所有する String にしてから表示用に扱う
                let decoded_stderr = decoded_stderr_cow.into_owned();

                // stderr が空の場合は、これまでに蓄積した stdout (full_output) を代替表示する
                let display_err = if decoded_stderr.trim().is_empty() {
                    if full_output.trim().is_empty() {
                        "<no output>".to_string()
                    } else {
                        full_output.clone()
                    }
                } else {
                    decoded_stderr
                };

                Err(format!(
                    "Pingコマンドがエラーで終了しました。:\n{}",
                    display_err
                ))
            }
            Err(e) => Err(format!("標準エラーの読み取りに失敗しました: {}", e)),
        }
    }
}
