use components::{AtHorizon, Bird, Colored, Fly, GlitchBlob, Height, Light, Positioned, Tree};

use crate::*;
use rand::{thread_rng, Rng};

pub fn ui(
    mut egui_ctx: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    mut player: Query<&mut Player>,
    mut params: ResMut<Params>,
    mut sky_blender: ResMut<SkyBlender>,
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
        ui.collapsing(RichText::new("Controls").heading(), |ui| {
            egui::Grid::new("controls")
                .num_columns(2)
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Move");
                    ui.label("WASD / Arrows");
                    ui.end_row();

                    ui.label("(Q)uit");
                    ui.label("Q");
                    ui.label("(F)ullscreen toggle");
                    ui.label("F");
                    ui.end_row();
                });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("FPS: ");
            ui.label(RichText::new(format!("{fps:.1}")).code());
        });

        ui.separator();

        ui.heading("Player");
        let mut player = player.single_mut();
        ui.horizontal(|ui| {
            ui.label("Height");
            ui.add(egui::Slider::new(&mut player.height, -128..=128));
        });

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
        ui.checkbox(&mut params.draw_poles, "Draw NSWE poles");

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Horizon");
            ui.add(egui::Slider::new(&mut params.sky_horizon_ratio, 0.0..=1.0));
        });

        ui.horizontal(|ui| {
            ui.label("Sky max bright");
            ui.add(egui::Slider::new(&mut params.sky_max_brightness, 0..=100));
        });

        if ui.add(egui::Button::new("Spawn light")).clicked() {
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
        if ui.add(egui::Button::new("Spawn giant")).clicked() {
            commands.spawn((
                Giant {
                    timer: Timer::new(Duration::from_secs(2), TimerMode::Repeating),
                    frame: 1,
                },
                Positioned { x: 10.0, y: 10.0 },
            ));
        }
        if ui.add(egui::Button::new("Spawn glitch blobs")).clicked() {
            spawn_glitch_blobs(commands, Vec2::new(player.x, player.y));
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Light Cone Y");
            ui.add(egui::Slider::new(&mut params.light_cone_off_y, 0..=100));
        });
        ui.horizontal(|ui| {
            ui.label("Light Cone X");
            ui.add(egui::Slider::new(&mut params.light_cone_off_x, -100..=100));
        });
        ui.horizontal(|ui| {
            ui.label("Light Cone Dist");
            ui.add(egui::Slider::new(
                &mut params.light_cone_max_dist,
                -100.0..=100.0,
            ));
        });

        let skyblender = sky_blender.as_mut();

        ui.heading("SkyBlender");
        ui.horizontal(|ui| {
            ui.label("SkyBlender height");
            ui.add(egui::Slider::new(&mut skyblender.height, -100..=100));
        });
        ui.horizontal(|ui| {
            ui.label("SkyBlender aura");
            ui.add(egui::Slider::new(&mut skyblender.extend, 10.0..=200.0));
        });
    });
}

fn spawn_glitch_blobs(mut commands: Commands, player_pos: Vec2) {
    commands.spawn((
        GlitchBlob {
            x: player_pos.x + 0.0,
            y: player_pos.y + 3.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 10.0 },
    ));
    //IsFalling));
    commands.spawn((
        GlitchBlob {
            x: player_pos.x + 0.0,
            y: player_pos.y + -3.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 10.0 },
        //    IsFalling));
    ));
    commands.spawn((
        GlitchBlob {
            x: player_pos.x + 30.0,
            y: player_pos.y + -30.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 8.0 },
    ));
    commands.spawn((
        GlitchBlob {
            x: player_pos.x - 30.0,
            y: player_pos.y + 30.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 8.0 },
    ));
    commands.spawn((
        GlitchBlob {
            x: player_pos.x + 0.0,
            y: player_pos.y + -30.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 6.0 },
    ));
    commands.spawn((
        GlitchBlob {
            x: player_pos.x + 0.0,
            y: player_pos.y + 30.0,
            color: Color::srgb_u8(240, 160, 160),
        },
        Height { height: 6.0 },
    ));
}
