use components::{AtHorizon, Light};

use crate::*;
use rand::{thread_rng, Rng};

pub fn ui(
    mut egui_ctx: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    player: Query<&Player>,
    mut params: ResMut<Params>,
    mut commands: Commands,
) {
    let fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .unwrap()
        .average()
        .unwrap_or_default();

    let params = params.as_mut();
    let ctx = egui_ctx.ctx_mut();
    egui::Window::new("Game state").show(ctx, |ui| {
        ui.collapsing(RichText::new("About").heading(), |ui| {
            ui.label(concat!("yada yada ", "", ""));
        });
        ui.heading("Controls");
        egui::Grid::new("controls")
            .num_columns(2)
            .striped(true)
            .show(ui, |ui| {
                ui.label("Move");
                ui.label("WASD / Arrows");
                ui.end_row();

                ui.label("(Q)uit");
                ui.label("Q");
                ui.end_row();
            });

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("FPS: ");
            ui.label(RichText::new(format!("{fps:.1}")).code());
        });
        ui.horizontal(|ui| {
            ui.label("Height");
            //ui.add(egui::Slider::new(&mut player.height, 64..=2048));
        });

        ui.heading("Player");
        let player = player.single();

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("Pos X: ");
            ui.label(RichText::new(format!("{}", player.x)).code());
        });
        ui.horizontal(|ui| {
            ui.label("Pos Y: ");
            ui.label(RichText::new(format!("{}", player.y)).code());
        });
        ui.horizontal(|ui| {
            ui.label("Direction: ");
            ui.label(RichText::new(format!("{}", player.direction)).code());
        });
        ui.checkbox(&mut params.sky_up_bright, "Sky up");
        ui.checkbox(&mut params.ground_up_bright, "Ground up");
        ui.horizontal(|ui| {
            ui.label("Horizon");
            ui.add(egui::Slider::new(&mut params.sky_horizon_ratio, 0.0..=1.0));
        });
        if ui.add(egui::Button::new("Click me")).clicked() {
            let mut rng = thread_rng();
            commands.spawn((
                Light {
                    color: Color::srgba_u8(
                        rng.gen_range(120..180),
                        rng.gen_range(120..180),
                        rng.gen_range(120..180),
                        rng.gen_range(80..250),
                    ),
                },
                AtHorizon {
                    angle: rng.gen_range((0.)..(2.0 * std::f32::consts::PI)) - std::f32::consts::PI,
                },
            ));
        }
    });
}
