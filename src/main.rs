use rand::Rng;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use terminal::{Action, Clear, Color, Retrieved, Value};

fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..20) {
        0 => Color::Reset,
        1 => Color::Black,
        2 => Color::DarkGrey,
        3 => Color::Red,
        4 => Color::DarkRed,
        5 => Color::Green,
        6 => Color::DarkGreen,
        7 => Color::Yellow,
        8 => Color::DarkYellow,
        9 => Color::Blue,
        10 => Color::DarkBlue,
        11 => Color::Magenta,
        12 => Color::DarkMagenta,
        13 => Color::Cyan,
        14 => Color::DarkCyan,
        15 => Color::White,
        16 => Color::Grey,
        17 => Color::Rgb(rng.gen(), rng.gen(), rng.gen()),
        _ => Color::AnsiValue(rng.gen()),
    }
}

fn main() {
    // JSONファイルを読み込む
    let file = File::open("data.json").expect("Failed to open data.json");
    let reader = BufReader::new(file);
    let data: Vec<String> = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // ターミナルのインスタンスを作成
    let terminal = terminal::stdout();
    let mut max_width = 0;
    let mut max_height = 0;

    if let Retrieved::TerminalSize(x, y) = terminal
        .get(Value::TerminalSize)
        .expect("error: terminal size.")
    {
        max_width = x;
        max_height = y;
    }

    // 無限ループ
    loop {
        // 画面初期化
        terminal
            .batch(Action::ClearTerminal(Clear::All))
            .expect("error: clear");
        // ランダムな文を選択
        let index = rand::thread_rng().gen_range(0..data.len());
        let text = &data[index];
        let x = rand::thread_rng().gen_range(0..max_width);
        // 20をtext.len()にするとおちる。
        let mut trun_by_text_len_width = max_height - 20;
        if trun_by_text_len_width < 0 {
            trun_by_text_len_width = 0;
        }
        let y = rand::thread_rng().gen_range(0..trun_by_text_len_width);
        // ランダムな位置を指定
        terminal
            .batch(Action::MoveCursorTo(x, y))
            .expect("error: cursor to");
        // ランダムな色指定
        terminal
            .batch(Action::SetForegroundColor(random_color()))
            .expect("error: set foreground color");
        print!("{} ({}:{})", &text, x, y);
        terminal
            .batch(Action::ResetColor)
            .expect("error: reset color");

        terminal.flush_batch().expect("error: flush batch");

        // 2秒待機
        thread::sleep(Duration::from_millis(800));
        terminal.act(Action::ScrollDown(1)).expect("error: scroll");
    }
}
