use std::env;
use std::process::Command;
use std::sync::Arc;
use eframe::egui;
use egui::{FontData, FontDefinitions};

pub fn run() -> Result<(), eframe::Error> {
    // アプリケーションの設定
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Seismometer downloader", // ウィンドウタイトル
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new(_cc)))),
    )
}

struct MyApp {
    screen: u8, // 0: Home, 1: Settings, 2: About
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            screen: 0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // サイドパネルの作成
        egui::SidePanel::left("side_menu").show(ctx, |ui| {
            ui.vertical(|ui| {
                if ui.button("Home").clicked() {
                    self.screen = 0;
                }
                if ui.button("Settings").clicked() {
                    self.screen = 1;
                }
                if ui.button("About").clicked() {
                    self.screen = 2;
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.screen {
                0 => self.home(ui),
                1 => self.settings(ui),
                2 => self.about(ui),
                _ => {}
            }
        });
    }
}

impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // フォントの設定
        let mut fonts = FontDefinitions::default();
        let font_data = FontData::from_static(include_bytes!("../../fonts/NotoSansJP-Regular.ttf"));
        fonts.font_data.insert(
            "NotoSansJP-Regular".to_owned(),
            Arc::new(font_data),
        );

        fonts.families.insert(
            egui::FontFamily::Name(Arc::from("sans-serif".to_owned())),
            vec!["NotoSansJP-Regular".to_owned()],
        );

        fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
            .insert(0, "NotoSansJP-Regular".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Default::default()
    }

    fn home(&mut self, ui: &mut egui::Ui) {
        ui.heading("ホーム");
        ui.label("おかゆグループ地震計プロジェクト (OGSP) に興味を持っていただきありがとうございます。");
        if ui.link("OGSP 公式ウェブサイト").clicked {
            // リンクがクリックされたときの処理
            println!("OGSP 公式ウェブサイトにアクセスします。");
            open_url("https://ogsp.okayugroup.com/");
        }
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.heading("Settings");
        ui.label("Settings are not implemented yet.");
    }

    fn about(&mut self, ui: &mut egui::Ui) {
        ui.heading("About");
        ui.label("This is a simple downloader for seismometer firmware.");
        ui.label("This is an open-source project.");
    }
}

fn open_url(url: &str) {
    // システムに応じたブラウザの開き方を選択
    if let Err(e) = open_url_platform_specific(url) {
        eprintln!("Failed to open URL: {}", e);
    }
}

fn open_url_platform_specific(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .arg("/C")
            .arg(format!("start {}", url))
            .output()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .output()?;
    } else if cfg!(target_os = "linux") {
        let desktop_env = env::var("XDG_SESSION_DESKTOP").unwrap_or_default();
        if desktop_env == "GNOME" || desktop_env == "Unity" {
            Command::new("xdg-open").arg(url).output()?;
        } else {
            Command::new("gnome-open").arg(url).output()?;
        }
    }
    Ok(())
}