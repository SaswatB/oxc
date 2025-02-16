// cargo run -p oxc-parser-runner
use eframe::egui;
use oxc_allocator::Allocator;
use oxc_parser::{ParseOptions, Parser};
use oxc_span::SourceType;

struct OxcParserApp {
    input: String,
    output: String,
    allocator: Allocator,
}

impl Default for OxcParserApp {
    fn default() -> Self {
        Self { input: String::from(""), output: String::new(), allocator: Allocator::default() }
    }
}

impl eframe::App for OxcParserApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::left_to_right(egui::Align::TOP)
                    .with_main_align(egui::Align::LEFT)
                    .with_main_justify(true),
                |ui| {
                    // Left side - Input with scrolling
                    let available_width = ui.available_width();
                    let left_width = available_width * 0.485;
                    let right_width = available_width * 0.485;

                    let input_response = ui.group(|ui| {
                        ui.set_width(left_width);
                        ui.set_min_height(ui.available_height());

                        egui::ScrollArea::vertical().id_source("input_scroll").show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.input)
                                    .desired_width(left_width)
                                    .code_editor(),
                            )
                        })
                    });

                    // Right side - Output with scrolling
                    ui.group(|ui| {
                        ui.set_width(right_width);
                        ui.set_min_height(ui.available_height());

                        egui::ScrollArea::vertical().id_source("output_scroll").show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.output)
                                    .desired_width(right_width)
                                    .code_editor(),
                            )
                        })
                    });

                    // Update output whenever input changes
                    if input_response.inner.inner.changed() {
                        let source_type = SourceType::default();
                        let ret = Parser::new(&self.allocator, &self.input, source_type)
                            .with_options(ParseOptions {
                                parse_regular_expression: true,
                                ..ParseOptions::default()
                            })
                            .parse();
                        self.output = format!("{:#?}", ret.source_file);
                    }
                },
            );
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        window_builder: Some(Box::new(|builder| {
            builder.with_inner_size(egui::Vec2::new(1200.0, 800.0))
        })),
        ..Default::default()
    };

    eframe::run_native("OXC Parser GUI", options, Box::new(|_cc| Box::new(OxcParserApp::default())))
}
