/*
以下の4言語、4つの挨拶を対話的に表示するプログラムを作成せよ

| 　　 |  日本語   |     英語      | フランス語  |    ドイツ語      |
|:---:|:--------:|:------------: |:---------:|:--------------:|
| 朝　 | おはよう　 | Good moring  |  Bonjour  |  Guten Morgen   |
| 昼　 | こんにちは |    Hello     |  Bonjour  |    Guten Tag    |
| 夜　 | こんばんは | Good evening |  Bonsoir  |   Guten Abend   |
| 別れ | さようなら |   Good-by    | Au revoir | Auf Wiedersehen |

プログラムは、
1. 最初の入力で言語選択
1. 次の選択でシチュエーション選択
1. 最初に戻る、または終了

- ユーザインタフェースはご自由に！
- 拡張性を考慮すること
*/

use std::io;

fn get_id(title: String, disp: &Vec<String>, max: u32) -> u32 {
    let mut buf = String::new();
    loop {
        println!("select {}", title);
        for i in 0..max {
            print!("{} = {} ", disp[i as usize], i+1);
        }
        println!("end = 9 : ");
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        buf = buf.trim().to_string();
        let id: u32 = buf.parse().unwrap_or(0);
        if id > 0 && id <= max {
            return id;
        }
        if id == 9 {
            return id;
        }
    }
}

fn main() {
    let ja = vec!["おはよう".to_string(), "こんにちは".to_string(), "こんばんは".to_string(), "さようなら".to_string()];
    let en = vec!["Good moring".to_string(), "Hello".to_string(), "Good evening".to_string(), "Good-by".to_string()];
    let fr = vec!["Bonjour".to_string(), "Bonjour".to_string(), "Bonsoir".to_string(), "Au revoir".to_string()];
    let gr = vec!["Guten Morgen".to_string(), "Guten Tag".to_string(), "Guten Abend".to_string(), "Auf Wiedersehen".to_string()];

    let ja_name = "日本語".to_string();
    let en_name = "English".to_string();
    let fr_name = "Français".to_string();
    let gr_name = "Deutsch".to_string();

    let messages = vec![ja, en, fr, gr];
    let languages = vec![ja_name, en_name, fr_name, gr_name];

    let max_language = messages.len();
    let max_message = messages[0].len();
    loop {
        let language_id = get_id("language".to_string(), &languages, max_language as u32);
        if language_id == 9 {
            break;
        }
        let message_id = get_id("message".to_string(), &messages[(language_id - 1) as usize], max_message as u32);
        if message_id == 9 {
            break;
        }
        println!("{}", 
            messages[(language_id - 1) as usize][(message_id - 1) as usize]);
    }
}
