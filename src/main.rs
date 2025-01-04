use eframe::{
    egui::{
        vec2, CentralPanel, ComboBox, Context, Frame, Margin, Slider, Theme, Ui, ViewportBuilder,
    },
    run_native, App as EApp, CreationContext, Frame as Eframe, NativeOptions,
};

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

struct App {
    start_temp: TempVal,
    start_temp_unit: TempUnit,
    final_temp_unit: TempUnit,
}

impl App {
    const NAME: &str = "Temperature Converter";
    const WIDTH: f32 = 500.;
    const HEIGHT: f32 = 200.;

    fn new(creation_context: &CreationContext) -> Self {
        creation_context.egui_ctx.set_theme(Theme::Light);
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
            Box::new(|creation_context: &CreationContext<'_>| {
                Ok(Box::new(Self::new(creation_context)))
            }),
        )
        .unwrap();
    }
}

impl EApp for App {
    fn update(&mut self, context: &Context, _frame: &mut Eframe) {
        CentralPanel::default().show(context, |ui: &mut Ui| {
            Frame::none()
                .inner_margin(Margin::same(5.))
                .show(ui, |ui: &mut Ui| {
                    ui.add_space(20.);
                    ui.vertical_centered(|ui: &mut Ui| {
                        ui.heading(Self::NAME);
                    });
                    ui.add_space(20.);
                    ui.label("Select a temperature unit to convert from:");
                    ui.add_space(20.);
                    ui.columns(2, |columns: &mut [Ui]| {
                        columns[0].vertical_centered(|ui: &mut Ui| {
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
                        columns[1].vertical_centered(|ui: &mut Ui| {
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
                    ui.label(format!(
                        "Result: {}{}",
                        Self::conv_temp(self.start_temp_unit, self.start_temp, self.final_temp_unit),
                        self.final_temp_unit.sym()
                    ));
                });
        });
    }
}

fn main() {
    App::run();
}
