use std::borrow::Cow;
use espflash::connection::Port;
use espflash::connection::reset::{ResetAfterOperation, ResetBeforeOperation};
use espflash::elf::RomSegment;
use espflash::flasher::Flasher;
use espflash::targets::Chip;
use espflash::targets::Chip::Esp32c6;
use serde::Deserialize;
use serialport::{COMPort, SerialPortBuilder, SerialPortInfo, SerialPortType, UsbPortInfo};

pub fn menu() -> Result<(), Box<dyn std::error::Error>> {
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
            "2" =>
                tokio::runtime::Runtime::new().unwrap().block_on(install())?,
            "3" => update(),
            "4" => settings(),
            "5" => reset(),
            "6" => break,
            _ => println!("無効な選択"),
        }
    }
    println!("さようなら！");
    Ok(())
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

#[derive(Deserialize, Debug)]
struct Firmware {
    version: String,
    url: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Devices {
    devices: Vec<Firmware>,
    last_updated: String,
}

async fn install() -> Result<(), Box<dyn std::error::Error>> {
    println!("ファームウェアのインストールには、インターネット接続が必要です。");

    println!("まず、マイコンをPCに接続してください。");
    println!("接続できましたか？ (y/n)");
    let mut connected = String::new();
    std::io::stdin().read_line(&mut connected)?;
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
        return Ok(());
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
            return Ok(());
        }
    }

    // マイコンデバイスを確認
    let devices = serialport::available_ports().expect("デバイスの取得に失敗しました。");

    let len = devices.len();

    if len == 0 {
        println!("デバイスが見つかりませんでした。");
        println!("デバイスが見つからない場合は、ドライバーのインストールが必要かもしれません。");
        println!("ドライバーのインストール方法は、公式ウェブサイトのドキュメントをご覧ください。");
        println!("https://ogsp.okayugroup.com/");
        return Ok(());
    }
    let mut selected_device : SerialPortInfo;
    if len > 1 {
        println!("複数のデバイスが見つかりました。");
        println!("どのデバイスを使用しますか？");
        for (i, device) in devices.iter().enumerate() {
            println!("{}: {}", i + 1, device.port_name);
        }
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;
        let choice = choice.trim().parse::<usize>()?;
        selected_device = devices[choice - 1].clone();
    } else {
        selected_device = devices[0].clone();
    }

    let mut usb_port_info: Option<UsbPortInfo> = None;
    match selected_device.clone().port_type {
        SerialPortType::UsbPort(info) => {
            usb_port_info = Some(info);
        }
        _ => {}
    }

    if usb_port_info.is_none() {
        println!("デバイスは検知されましたが、USBデバイスではありません。");
        println!("USBデバイスを使用してください。");
        return Ok(());
    }

    println!("デバイス: {}", selected_device.port_name);


    println!("ファームウェアが対応しているデバイスのリストを取得しています...");

    let client = reqwest::Client::new();
    let res = client.get("https://api.okayugroup.com/ogsp/firmware/devices.json")
        .send()
        .await?;

    let body = res.text().await?;
    let devices: Devices = serde_json::from_str(&body).expect("デバイスのリストの取得に失敗しました。");
    println!("デバイスのリストを取得しました。");

    println!("インストールするデバイスを選択してください。");
    for (i, device) in devices.devices.iter().enumerate() {
        println!("{}: {}", i + 1, device.name);
    }

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice)?;
    let choice = choice.trim().parse::<usize>()?;
    let device = &devices.devices[choice - 1];

    println!("ファームウェア: {}", device.name);

    println!("ファームウェアをダウンロードしています...");

    let url = &device.url.replace("<version>", &device.version);  // versionをlatestに置換
    println!("URL: {}", url);
    let res = client.get(url)
        .send().await.expect("ファームウェアのダウンロードに失敗しました。");

    let bytes = res.bytes().await?;
    let bytes_array: Vec<u8> = bytes.to_vec();

    println!("ファームウェアをダウンロードしました。");

    // ファームウェアの書き込み
    println!("ファームウェアを書き込んでいます...");
    let serial = serialport::new(&selected_device.port_name, 115200).timeout(std::time::Duration::from_secs(1));
    let com = COMPort::open(&serial)?;
    let mut flasher = Flasher::connect(com, usb_port_info.unwrap(), None, false, false, false, Some(Esp32c6), ResetAfterOperation::HardReset, ResetBeforeOperation::DefaultReset)?;
    let rom = RomSegment {
        addr: 0,
        data: Cow::from(bytes_array),
    };
    flasher.write_bins_to_flash(&[rom], None)?;

    println!("ファームウェアの書き込みが完了しました。");

    Ok(())

}


fn update() {}
fn settings() {}
fn reset() {}