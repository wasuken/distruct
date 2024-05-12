use rand::Rng;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;
use terminal::{Action, Clear, Color};

fn main() {
    // JSONファイルを読み込む
    let file = File::open("data.json").expect("Failed to open data.json");
    let reader = BufReader::new(file);
    let data: Vec<String> = serde_json::from_reader(reader).expect("Failed to parse JSON");

    // ターミナルのインスタンスを作成
    let terminal = terminal::stdout();

    // 無限ループ
    loop {
        // ランダムな文を選択
        let index = rand::thread_rng().gen_range(0..data.len());
        let text = &data[index];
        terminal
            .batch(Action::ClearTerminal(Clear::All))
            .expect("error: clear");
        terminal
            .batch(Action::MoveCursorTo(0, 0))
            .expect("error: clear");
        terminal
            .batch(Action::SetForegroundColor(Color::Green))
            .expect("error: set foreground color");
        print!("{}", &text);
        terminal
            .batch(Action::ResetColor)
            .expect("error: reset color");

        terminal.flush_batch().expect("error: flush batch");

        // 2秒待機
        thread::sleep(Duration::from_millis(800));
        terminal.act(Action::ScrollDown(1)).expect("error: scroll");
    }
}
