use clap::Parser;

#[derive(Parser, Debug)] //derive = 派生するの意
#[command(author, version, about, long_about = None)]
pub struct Args {
    // コマンド引数を入れる構造体
    //Args 引数構造体
    #[arg(index = 1)]
    pub target: String,

    // 送信パケット
    #[arg(short = 'c', long, default_value_t = 4)]
    pub count: u32,

    // パケットのタイムアウト値(msec)
    #[arg(short = 'w', long, default_value_t = 4000)]
    pub timeout: u32,

    /// 指定するとビープを無効にします（デフォルト：ビープ有効）
    #[arg(short = 'n', long = "nonbeep", default_value_t = false)]
    pub nonbeep: bool,

    /// 継続的に ping を送信する (-t の動作)
    #[arg(short = 't', long = "continuous", default_value_t = false)]
    pub continuous: bool,
}
