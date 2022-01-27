use std::fmt::format;

use eframe::{
    egui,
    egui::{CentralPanel, CtxRef, FontData, FontDefinitions, FontFamily, ScrollArea, Vec2},
    epi::App,
    run_native, NativeOptions,
};

struct Headlines {
    articles: Vec<NewsCardData>,
}

impl Headlines {
    fn new() -> Headlines {
        let items = (0..20).map(|i| NewsCardData {
            title: format!("标题 {i}"),
            desc: format!("描述 {i}"),
            url: format!("https://example.com/{i}"),
        });

        Headlines {
            articles: Vec::from_iter(items),
        }
    }

    // fn configure_fonts(&self, ctx: &CtxRef) {
    //     let mut font_def = FontDefinitions::default();
    //     font_def.font_data.insert(
    //         "NotoSansSc".to_owned(),
    //         FontData::from_static(include_bytes!("../resources/font.otf")),
    //     );

    //     font_def
    //         .family_and_size
    //         .insert(egui::TextStyle::Heading, (FontFamily::Proportional, 35.0));

    //         font_def
    //         .family_and_size
    //         .insert(egui::TextStyle::Body, (FontFamily::Proportional, 20.0));

    //     font_def
    //         .fonts_for_family
    //         .get_mut(&FontFamily::Proportional)
    //         .unwrap()
    //         .insert(0, "NotoSansSc".to_owned());

    //     ctx.set_fonts(font_def);
    // }
}
struct NewsCardData {
    title: String,
    desc: String,
    url: String,
}

impl App for Headlines {
    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &eframe::epi::Frame,
        _storage: Option<&dyn eframe::epi::Storage>,
    ) {
        // self.configure_fonts(&_ctx)
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for a in &self.articles {
                    ui.label(&a.title);
                    ui.label(&a.desc);
                    ui.label(&a.url);
                }
            });
        });
    }

    fn name(&self) -> &str {
        "第一个 APP"
    }
}

fn main() {
    let app = Headlines::new();
    let mut win_option = NativeOptions::default();
    win_option.initial_window_size = Some(Vec2::new(300.0, 120.0));
    run_native(Box::new(app), win_option)
}
