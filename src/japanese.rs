pub fn menu() {
    println!("こんにちは！");
    println!("おかゆグループ地震計プロジェクトに興味を持っていただきありがとうございます。");

    loop {
        println!("何をしますか？");
        println!("1. おかゆグループ地震計プロジェクトについて");
        println!("2. ファームウェアのインストール");
        println!("3. ファームウェアのアップデート");
        println!("4. 地震計の設定画面を表示");
        println!("5. ファームウェアと設定をリセット");
        println!("6. 終了");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).unwrap();
        let choice = choice.trim();
        match choice {
            "1" => about(),
            "2" => install(),
            "3" => update(),
            "4" => settings(),
            "5" => reset(),
            "6" => break,
            _ => println!("無効な選択"),
        }
    }
    println!("さようなら！");
}

fn about() {
    println!("おかゆグループ地震計プロジェクトは、全国に震度計を設置し、地震の揺れを検知するオープンソースかつ非営利のプロジェクトです。");
    println!("このプロジェクトは、とある中学生の自由研究がきっかけで始まりました。ぜひ皆さんも震度計を家に設置して、揺れを可視化してみましょう！");
    println!("詳細は https://ogsp.okayugroup.com/ をご覧ください。");
    println!();
    println!("震度計は、安価で購入できるマイコンを使用して、簡単に作成できます。");
    println!("震度計の作成方法については、公式ウェブサイトに詳細が載っていますが、ここでは簡単に説明します。");
    println!("費用は約3000円程度で、部品は秋月電子通商などで購入できます。");
    println!("マイコンには、現在ESP32を推奨しています。");
    println!("はんだ付けが必要ですが、注意事項を守れば、初心者でも作成できます。");
    println!("震度計の作成方法については、公式ウェブサイトをご覧ください。");
    println!("[ドキュメント作成中]");
    println!();

    std::thread::sleep(std::time::Duration::from_millis(500));
}

fn install() {
    println!("ファームウェアのインストールには、インターネット接続が必要です。");

    println!("まず、マイコンをPCに接続してください。");
    println!("接続できましたか？ (y/n)");
    let mut connected = String::new();
    std::io::stdin().read_line(&mut connected).unwrap();
    let connected = connected.trim();

    if connected == "n" {
        println!("もしよければ、接続できなかった理由を教えてください。(空白の場合はスキップします。)");
        let mut reason = String::new();
        std::io::stdin().read_line(&mut reason).unwrap();
        let reason = reason.trim();
        if reason != "" {
            println!("お答えいただきありがとうございます。");
            println!("よくある理由は、公式ウェブサイトのドキュメントに記載しています。");
            println!("https://ogsp.okayugroup.com/");
        } else {
            println!("理由がわからない場合は、公式ウェブサイトのドキュメントをご覧ください。");
            println!("https://ogsp.okayugroup.com/");
        }
        return
    }

    println!("環境を確認しています...");

    // PCのOSを確認
    let os = std::env::consts::OS;
    println!("OS: {}", os);

    // インターネット接続を確認 (Googleに接続できるか)
    let connected = std::net::TcpStream::connect("google.com:80");
    match connected {
        Ok(_) => println!("インターネット接続: 成功"),
        Err(e) => {
            println!("インターネット接続: 失敗");
            // エラーの内容が、未接続なのかDNSエラーなどなのかを判定
            if e.kind() == std::io::ErrorKind::ConnectionRefused {
                println!("インターネット接続が拒否されました。");
                println!("接続先のサーバーが応答していないか、接続先のポートが閉じられている可能性があります。");
                println!("インターネット接続が拒否された場合は、接続先のサーバーの状態を確認してください。");
            } else {
                println!("インターネット接続に失敗しました。");
                println!("インターネット接続ができない場合は、接続先のサーバーの状態を確認してください。");
            }
            return
        }
    }

    // マイコンデバイスを確認
    let devices = serialport::available_ports().expect("デバイスの取得に失敗しました。");
    if devices.len() == 0 {
        println!("デバイスが見つかりませんでした。");
        println!("デバイスが見つからない場合は、ドライバーのインストールが必要かもしれません。");
        println!("ドライバーのインストール方法は、公式ウェブサイトのドキュメントをご覧ください。");
        println!("https://ogsp.okayugroup.com/");
        return
    }

    for port in devices {
        match port.port_type {
            serialport::SerialPortType::UsbPort(info) => {
                println!("Port: {:?}", port.port_name);
                println!("  VID: {:04x}", info.vid);
                println!("  PID: {:04x}", info.pid);
                println!("  Manufacturer: {:?}", info.manufacturer);
                println!("  Serial Number: {:?}", info.serial_number);
            }
            _ => println!("Port: {:?} is not a USB port", port.port_name),
        }
    }

    // todo ESP32を探す

    // todo ファームウェアのダウンロード

    // todo ファームウェアの書き込み

}


fn update() {}
fn settings() {}
fn reset() {}