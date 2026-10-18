#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, Color32, RichText};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Phase {
    Focus,
    Break,
}

#[derive(Debug)]
struct Timer {
    phase: Phase,
    remaining_secs: u64,
    focus_minutes: u64,
    break_minutes: u64,
    running: bool,
    completed_sessions: u32,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            phase: Phase::Focus,
            remaining_secs: 25 * 60,
            focus_minutes: 25,
            break_minutes: 5,
            running: false,
            completed_sessions: 0,
        }
    }
}

impl Timer {
    fn duration_for(&self, phase: Phase) -> u64 {
        match phase {
            Phase::Focus => self.focus_minutes * 60,
            Phase::Break => self.break_minutes * 60,
        }
    }
    fn reset(&mut self) {
        self.running = false;
        self.remaining_secs = self.duration_for(self.phase);
    }
    fn select_phase(&mut self, phase: Phase) {
        self.phase = phase;
        self.reset();
    }
    fn advance(&mut self, seconds: u64) {
        if !self.running || seconds == 0 {
            return;
        }
        if seconds < self.remaining_secs {
            self.remaining_secs -= seconds;
            return;
        }
        if self.phase == Phase::Focus {
            self.completed_sessions += 1;
            self.phase = Phase::Break;
        } else {
            self.phase = Phase::Focus;
        }
        self.reset();
    }
    fn clock_text(&self) -> String {
        format!(
            "{:02}:{:02}",
            self.remaining_secs / 60,
            self.remaining_secs % 60
        )
    }
}

struct FocusApp {
    timer: Timer,
    last_tick: Instant,
}

impl Default for FocusApp {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            last_tick: Instant::now(),
        }
    }
}

impl eframe::App for FocusApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        if self.timer.running {
            let elapsed = now.duration_since(self.last_tick).as_secs();
            if elapsed > 0 {
                self.timer.advance(elapsed);
                self.last_tick += Duration::from_secs(elapsed);
            }
            ui.ctx().request_repaint_after(Duration::from_millis(100));
        } else {
            self.last_tick = now;
        }

        let ink = Color32::from_rgb(30, 42, 59);
        let muted = Color32::from_rgb(113, 125, 144);
        let accent = match self.timer.phase {
            Phase::Focus => Color32::from_rgb(68, 107, 225),
            Phase::Break => Color32::from_rgb(38, 157, 124),
        };
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(Color32::from_rgb(248, 249, 252))
                    .inner_margin(16),
            )
            .show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(5.0);
                    ui.label(RichText::new("focus.").size(28.0).strong().color(ink));
                    ui.label(
                        RichText::new("A little space to do your best work")
                            .size(13.0)
                            .color(muted),
                    );
                    ui.add_space(16.0);

                    egui::Frame::new()
                        .fill(Color32::from_rgb(234, 238, 245))
                        .corner_radius(13)
                        .inner_margin(3)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                for (phase, label) in
                                    [(Phase::Focus, "Focus"), (Phase::Break, "Break")]
                                {
                                    let selected = self.timer.phase == phase;
                                    let button = egui::Button::new(
                                        RichText::new(label).size(14.0).color(if selected {
                                            ink
                                        } else {
                                            muted
                                        }),
                                    )
                                    .fill(if selected {
                                        Color32::WHITE
                                    } else {
                                        Color32::TRANSPARENT
                                    })
                                    .corner_radius(10);
                                    if ui.add_sized([138.0, 34.0], button).clicked() {
                                        self.timer.select_phase(phase);
                                    }
                                }
                            });
                        });

                    ui.add_space(10.0);
                    let (rect, _) =
                        ui.allocate_exact_size(egui::vec2(190.0, 190.0), egui::Sense::hover());
                    let center = rect.center();
                    let radius = 78.0;
                    ui.painter().circle_stroke(
                        center,
                        radius,
                        egui::Stroke::new(9.0, Color32::from_rgb(226, 231, 240)),
                    );
                    let total = self.timer.duration_for(self.timer.phase);
                    let remaining = self.timer.remaining_secs as f32 / total as f32;
                    if remaining > 0.0 {
                        let steps = ((remaining * 96.0).ceil() as usize).max(2);
                        let points: Vec<_> = (0..=steps)
                            .map(|i| {
                                let angle = -std::f32::consts::FRAC_PI_2
                                    + std::f32::consts::TAU * remaining * i as f32 / steps as f32;
                                center + egui::vec2(angle.cos(), angle.sin()) * radius
                            })
                            .collect();
                        ui.painter()
                            .add(egui::Shape::line(points, egui::Stroke::new(9.0, accent)));
                    }
                    ui.painter().text(
                        center + egui::vec2(0.0, -23.0),
                        egui::Align2::CENTER_CENTER,
                        if self.timer.phase == Phase::Focus {
                            "FOCUS"
                        } else {
                            "BREAK"
                        },
                        egui::FontId::proportional(12.0),
                        muted,
                    );
                    ui.painter().text(
                        center + egui::vec2(0.0, 9.0),
                        egui::Align2::CENTER_CENTER,
                        self.timer.clock_text(),
                        egui::FontId::proportional(48.0),
                        ink,
                    );
                    ui.painter().text(
                        center + egui::vec2(0.0, 47.0),
                        egui::Align2::CENTER_CENTER,
                        if self.timer.running {
                            "Stay with it"
                        } else {
                            "Ready when you are"
                        },
                        egui::FontId::proportional(12.0),
                        muted,
                    );

                    ui.add_space(8.0);
                    let action = if self.timer.running {
                        "Pause timer"
                    } else {
                        "Start timer"
                    };
                    let primary = egui::Button::new(
                        RichText::new(action)
                            .size(15.0)
                            .strong()
                            .color(Color32::WHITE),
                    )
                    .fill(accent)
                    .corner_radius(13);
                    if ui.add_sized([286.0, 44.0], primary).clicked() {
                        self.timer.running = !self.timer.running;
                        self.last_tick = Instant::now();
                    }
                    if ui
                        .add(
                            egui::Button::new(RichText::new("Reset").size(13.0).color(muted))
                                .fill(Color32::TRANSPARENT),
                        )
                        .clicked()
                    {
                        self.timer.reset();
                    }

                    ui.add_space(10.0);
                    egui::Frame::new()
                        .fill(Color32::WHITE)
                        .corner_radius(16)
                        .inner_margin(12)
                        .show(ui, |ui| {
                            ui.set_width(300.0);
                            ui.with_layout(egui::Layout::top_down(egui::Align::Min), |ui| {
                                ui.label(
                                    RichText::new("DURATIONS").size(11.0).strong().color(muted),
                                );
                                ui.add_space(4.0);
                                let focus_changed = duration_stepper(
                                    ui,
                                    "Focus",
                                    &mut self.timer.focus_minutes,
                                    120,
                                    !self.timer.running,
                                );
                                let break_changed = duration_stepper(
                                    ui,
                                    "Break",
                                    &mut self.timer.break_minutes,
                                    60,
                                    !self.timer.running,
                                );
                                if !self.timer.running
                                    && ((focus_changed && self.timer.phase == Phase::Focus)
                                        || (break_changed && self.timer.phase == Phase::Break))
                                {
                                    self.timer.reset();
                                }
                            });
                        });
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(format!(
                            "{} sessions completed",
                            self.timer.completed_sessions
                        ))
                        .size(12.0)
                        .color(muted),
                    );
                });
            });
    }
}

fn duration_stepper(
    ui: &mut egui::Ui,
    label: &str,
    value: &mut u64,
    max: u64,
    enabled: bool,
) -> bool {
    let mut changed = false;
    ui.horizontal(|ui| {
        ui.add_sized(
            [138.0, 28.0],
            egui::Label::new(RichText::new(label).size(14.0)),
        );
        if ui
            .add_enabled(
                enabled && *value > 1,
                egui::Button::new("−").min_size(egui::vec2(28.0, 28.0)),
            )
            .clicked()
        {
            *value -= 1;
            changed = true;
        }
        ui.add_sized([54.0, 28.0], egui::Label::new(format!("{} min", *value)));
        if ui
            .add_enabled(
                enabled && *value < max,
                egui::Button::new("+").min_size(egui::vec2(28.0, 28.0)),
            )
            .clicked()
        {
            *value += 1;
            changed = true;
        }
    });
    changed
}

fn app_icon() -> egui::IconData {
    let mut rgba = Vec::with_capacity(64 * 64 * 4);
    for y in 0..64 {
        for x in 0..64 {
            let dx = (x as f32 - 31.5).abs() - 18.0;
            let dy = (y as f32 - 31.5).abs() - 18.0;
            let outside = egui::vec2(dx.max(0.0), dy.max(0.0)).length();
            let square_distance = outside + dx.max(dy).min(0.0) - 10.0;
            let circle_distance = ((x as f32 - 31.5).hypot(y as f32 - 31.5) - 15.0).abs();
            let hand = (x as f32 - 31.5).abs() < 1.8 && (21.0..=32.0).contains(&(y as f32));
            let inside = square_distance <= 0.0;
            let (red, green, blue, alpha) = if !inside {
                (0, 0, 0, 0)
            } else if circle_distance < 2.2 || hand {
                (255, 255, 255, 255)
            } else {
                (68, 107, 225, 255)
            };
            rgba.extend_from_slice(&[red, green, blue, alpha]);
        }
    }
    egui::IconData {
        rgba,
        width: 64,
        height: 64,
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([420.0, 590.0])
            .with_min_inner_size([400.0, 570.0])
            .with_icon(app_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "Focus Timer",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
            if let Ok(font_bytes) = std::fs::read(r"C:\Windows\Fonts\segoeui.ttf") {
                let mut fonts = egui::FontDefinitions::default();
                fonts.font_data.insert(
                    "Segoe UI".to_owned(),
                    egui::FontData::from_owned(font_bytes).into(),
                );
                fonts
                    .families
                    .entry(egui::FontFamily::Proportional)
                    .or_default()
                    .insert(0, "Segoe UI".to_owned());
                cc.egui_ctx.set_fonts(fonts);
            }
            Ok(Box::new(FocusApp::default()))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn timer_counts_down_only_when_running() {
        let mut timer = Timer::default();
        timer.advance(10);
        assert_eq!(timer.remaining_secs, 1500);
        timer.running = true;
        timer.advance(10);
        assert_eq!(timer.remaining_secs, 1490);
    }
    #[test]
    fn completed_focus_session_starts_paused_break() {
        let mut timer = Timer {
            running: true,
            ..Timer::default()
        };
        timer.advance(1500);
        assert_eq!(timer.phase, Phase::Break);
        assert_eq!(timer.remaining_secs, 300);
        assert_eq!(timer.completed_sessions, 1);
        assert!(!timer.running);
    }
    #[test]
    fn completed_break_returns_to_focus_without_incrementing_count() {
        let mut timer = Timer::default();
        timer.select_phase(Phase::Break);
        timer.running = true;
        timer.advance(300);
        assert_eq!(timer.phase, Phase::Focus);
        assert_eq!(timer.remaining_secs, 1500);
        assert_eq!(timer.completed_sessions, 0);
    }
    #[test]
    fn reset_uses_selected_duration() {
        let mut timer = Timer {
            focus_minutes: 40,
            ..Timer::default()
        };
        timer.reset();
        assert_eq!(timer.clock_text(), "40:00");
    }
}
