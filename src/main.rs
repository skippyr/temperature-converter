use eframe::{
    egui::{
        vec2, CentralPanel, ComboBox, Context, FontData, FontDefinitions, FontFamily, FontId,
        Frame, Margin, RichText, Slider, TextStyle, Theme, ViewportBuilder,
    },
    run_native, App as EApp, CreationContext, Frame as Eframe, NativeOptions,
};
use std::sync::Arc;

type TempVal = i16;

#[derive(PartialEq, Copy, Clone, Eq, Debug)]
enum TempUnit {
    Celsius,
    Fahnherit,
    Kelvin,
}

impl TempUnit {
    fn all() -> [TempUnit; 3] {
        [TempUnit::Celsius, TempUnit::Fahnherit, TempUnit::Kelvin]
    }

    fn name(&self) -> &'static str {
        match self {
            TempUnit::Celsius => "Celsius",
            TempUnit::Fahnherit => "Fahnherit",
            TempUnit::Kelvin => "Kelvin",
        }
    }

    fn sym(&self) -> &'static str {
        match self {
            TempUnit::Celsius => "°C",
            TempUnit::Fahnherit => "°F",
            TempUnit::Kelvin => "K",
        }
    }
}

struct AppTextStyles;

impl AppTextStyles {
    fn body_bold() -> TextStyle {
        TextStyle::Name("body_bold".into())
    }
}

struct App {
    start_temp: TempVal,
    start_temp_unit: TempUnit,
    final_temp_unit: TempUnit,
}

impl App {
    const NAME: &str = "Temperature Converter";
    const WIDTH: f32 = 500.;
    const HEIGHT: f32 = 210.;

    fn new(creation_ctx: &CreationContext) -> Self {
        let mut fonts = FontDefinitions::empty();
        fonts.font_data.insert(
            String::from("inter"),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/inter/regular.ttf"
            ))),
        );
        fonts.font_data.insert(
            String::from("inter_bold"),
            Arc::new(FontData::from_static(include_bytes!(
                "../assets/inter/bold.ttf"
            ))),
        );
        fonts
            .families
            .entry(FontFamily::Proportional)
            .or_default()
            .push(String::from("inter"));
        fonts
            .families
            .entry(FontFamily::Name("inter_bold".into()))
            .or_default()
            .push(String::from("inter_bold"));
        creation_ctx.egui_ctx.set_theme(Theme::Light);
        creation_ctx.egui_ctx.set_fonts(fonts);
        creation_ctx.egui_ctx.style_mut(|s| {
            s.text_styles.entry(TextStyle::Heading).or_default().family =
                FontFamily::Name("inter_bold".into());
            s.text_styles.entry(TextStyle::Body).or_default().size = 14.;
            *s.text_styles.entry(AppTextStyles::body_bold()).or_default() = FontId {
                family: FontFamily::Name("inter_bold".into()),
                ..*s.text_styles.entry(TextStyle::Body).or_default()
            };
        });
        Self {
            start_temp: 0,
            start_temp_unit: TempUnit::Celsius,
            final_temp_unit: TempUnit::Fahnherit,
        }
    }

    fn conv_temp(start_unit: TempUnit, start_temp: TempVal, final_temp_unit: TempUnit) -> TempVal {
        let k = Self::to_kelvin(start_temp, start_unit);
        match final_temp_unit {
            TempUnit::Celsius => k - 273,
            TempUnit::Fahnherit => k * 9 / 5 - 459,
            TempUnit::Kelvin => k,
        }
    }

    fn to_kelvin(temp: TempVal, unit: TempUnit) -> TempVal {
        match unit {
            TempUnit::Celsius => temp + 273,
            TempUnit::Fahnherit => (temp - 32) * 5 / 9 + 273,
            TempUnit::Kelvin => temp,
        }
    }

    fn native_options() -> NativeOptions {
        NativeOptions {
            viewport: ViewportBuilder::default()
                .with_resizable(false)
                .with_inner_size(vec2(Self::WIDTH, Self::HEIGHT)),
            ..Default::default()
        }
    }

    fn run() {
        run_native(
            Self::NAME,
            Self::native_options(),
            Box::new(|creation_ctx| Ok(Box::new(Self::new(creation_ctx)))),
        )
        .unwrap();
    }
}

impl EApp for App {
    fn update(&mut self, context: &Context, _frame: &mut Eframe) {
        CentralPanel::default().show(context, |ui| {
            Frame::none().inner_margin(Margin::same(5.)).show(ui, |ui| {
                ui.add_space(10.);
                ui.vertical_centered(|ui| {
                    ui.heading(Self::NAME);
                });
                ui.add_space(20.);
                ui.label("Select a temperature unit to convert from:");
                ui.add_space(20.);
                ui.columns(2, |cols| {
                    cols[0].vertical_centered(|ui| {
                        ui.spacing_mut().slider_width = 150.;
                        ui.add(
                            Slider::new(
                                &mut self.start_temp,
                                Self::conv_temp(TempUnit::Celsius, 0, self.start_temp_unit)
                                    ..=Self::conv_temp(
                                        TempUnit::Celsius,
                                        100,
                                        self.start_temp_unit,
                                    ),
                            )
                            .text(self.start_temp_unit.sym()),
                        );
                    });
                    cols[1].vertical_centered(|ui| {
                        ComboBox::from_label("Start Unit")
                            .selected_text(self.start_temp_unit.name())
                            .show_ui(ui, |ui| {
                                for u in TempUnit::all() {
                                    let mut t = TempUnit::Celsius;
                                    if ui.selectable_value(&mut t, u, u.name()).clicked() {
                                        self.start_temp = Self::conv_temp(
                                            self.start_temp_unit,
                                            self.start_temp,
                                            t,
                                        );
                                        self.start_temp_unit = t;
                                        if self.start_temp_unit == self.final_temp_unit {
                                            self.final_temp_unit = TempUnit::all()
                                                .into_iter()
                                                .find(|u| *u != self.start_temp_unit)
                                                .unwrap();
                                        }
                                    }
                                }
                            });
                        ComboBox::from_label("Final Unit")
                            .selected_text(self.final_temp_unit.name())
                            .show_ui(ui, |ui| {
                                for u in TempUnit::all()
                                    .into_iter()
                                    .filter(|u| *u != self.start_temp_unit)
                                {
                                    ui.selectable_value(&mut self.final_temp_unit, u, u.name());
                                }
                            })
                    });
                });
                ui.add_space(20.);
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing = vec2(1., 3.);
                    ui.label(RichText::new("Result: ").text_style(AppTextStyles::body_bold()));
                    ui.label(format!(
                        "{}{}",
                        Self::conv_temp(
                            self.start_temp_unit,
                            self.start_temp,
                            self.final_temp_unit
                        ),
                        self.final_temp_unit.sym()
                    ));
                });
            });
        });
    }
}

fn main() {
    App::run();
}
