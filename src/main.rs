use eframe::{self, egui};
use serde_json::Value;

static KEY: &str = "de2d8ebf69454da09ef530ff67fd2414";

fn main() {
    // env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([300.0, 430.0]),
        ..Default::default()
    };
    eframe::run_native(
        "蒿以死德威的",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(WeatherApp::new(cc))
        }),
    ).unwrap()
}

#[derive(Default, Debug)]
struct City {
    name: String,
    lat: String,
    lon: String,
}

#[derive(Default)]
struct WeatherApp {
    input_city: String,
    city: City,
}


// impl Default for WeatherApp {
//     fn default() -> Self {
//         Self {
//         }
//     }
// }

impl WeatherApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        Self {
            ..WeatherApp::default()
        }
    }
}

impl eframe::App for WeatherApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                let label = ui.label("城市：");
                ui.add(egui::TextEdit::singleline(&mut self.input_city).hint_text(" ").desired_width(180.0))
                    .labelled_by(label.id);
                if ui.button("查詢").clicked() {
                    let url = format!("http://api.openweathermap.org/geo/1.0/direct?q={}&appid={}", self.input_city, KEY);
                    let json_city = reqwest::blocking::get(&url).unwrap()
                        .text().unwrap();
                    let v: Value = serde_json::from_str(&json_city).unwrap();
                    if let Value::Array(v)  = v {
                        if let Value::Object(v) = &v[0] {
                            if let Value::String(s) = &v["local_names"]["zh"] {
                                self.city.name = s.clone();
                            }
                            if let Value::Number(n) = &v["lat"] {
                                self.city.lat = n.to_string();
                            }
                            if let Value::Number(n) = &v["lon"] {
                                self.city.lon = n.to_string();
                            }
                        }
                    }

                    let url = format!("https://api.openweathermap.org/data/2.5/weather?lat={}&lon={}&appid={}", 
                        self.city.lat, 
                        self.city.lon,
                        KEY);
                    let json_weather = reqwest::blocking::get(&url).unwrap()
                        .text().unwrap();
                    let v: Value = serde_json::from_str(&json_weather).unwrap();
                    
                    println!("{:?}", v);

                }
            });
            ui.label(format!("{:?}", self.city));
        });
    }
}

fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();

    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../necessary/fonts/jf-openhuninn-1.0.ttf"
        )),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}