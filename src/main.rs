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

        let accent = if self.timer.phase == Phase::Focus {
            Color32::from_rgb(47, 111, 226)
        } else {
            Color32::from_rgb(35, 157, 117)
        };
        let muted = Color32::from_rgb(112, 122, 139);
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Color32::from_rgb(246, 248, 252)))
            .show(ui, |ui| {
                ui.add_space(18.0);
                ui.vertical_centered(|ui| {
                    ui.heading(
                        RichText::new("Focus Timer")
                            .size(28.0)
                            .color(Color32::from_rgb(30, 42, 60)),
                    );
                    ui.label(
                        RichText::new("Make room for what matters.")
                            .size(14.0)
                            .color(muted),
                    );
                    ui.add_space(24.0);
                    egui::Frame::new()
                        .fill(Color32::WHITE)
                        .corner_radius(24)
                        .inner_margin(24)
                        .show(ui, |ui| {
                            ui.set_width(370.0);
                            ui.vertical_centered(|ui| {
                                ui.horizontal(|ui| {
                                    for (phase, label) in
                                        [(Phase::Focus, "Focus"), (Phase::Break, "Break")]
                                    {
                                        let selected = self.timer.phase == phase;
                                        let button =
                                            egui::Button::new(
                                                RichText::new(label).size(15.0).color(
                                                    if selected { Color32::WHITE } else { muted },
                                                ),
                                            )
                                            .fill(if selected {
                                                accent
                                            } else {
                                                Color32::from_rgb(239, 242, 247)
                                            })
                                            .corner_radius(12);
                                        if ui.add_sized([105.0, 34.0], button).clicked() {
                                            self.timer.select_phase(phase);
                                        }
                                    }
                                });
                                ui.add_space(16.0);
                                let (rect, _) = ui.allocate_exact_size(
                                    egui::vec2(230.0, 230.0),
                                    egui::Sense::hover(),
                                );
                                let center = rect.center();
                                let radius = 100.0;
                                ui.painter().circle_stroke(
                                    center,
                                    radius,
                                    egui::Stroke::new(12.0, Color32::from_rgb(231, 235, 242)),
                                );
                                let total = self.timer.duration_for(self.timer.phase);
                                let progress =
                                    1.0 - self.timer.remaining_secs as f32 / total as f32;
                                if progress > 0.0 {
                                    let steps = ((progress * 96.0).ceil() as usize).max(2);
                                    let points: Vec<_> = (0..=steps)
                                        .map(|i| {
                                            let angle = -std::f32::consts::FRAC_PI_2
                                                + std::f32::consts::TAU * progress * i as f32
                                                    / steps as f32;
                                            center + egui::vec2(angle.cos(), angle.sin()) * radius
                                        })
                                        .collect();
                                    ui.painter().add(egui::Shape::line(
                                        points,
                                        egui::Stroke::new(12.0, accent),
                                    ));
                                }
                                ui.painter().text(
                                    center + egui::vec2(0.0, -9.0),
                                    egui::Align2::CENTER_CENTER,
                                    self.timer.clock_text(),
                                    egui::FontId::proportional(55.0),
                                    Color32::from_rgb(30, 42, 60),
                                );
                                ui.painter().text(
                                    center + egui::vec2(0.0, 39.0),
                                    egui::Align2::CENTER_CENTER,
                                    if self.timer.running {
                                        "IN PROGRESS"
                                    } else {
                                        "READY WHEN YOU ARE"
                                    },
                                    egui::FontId::proportional(12.0),
                                    muted,
                                );
                                ui.add_space(12.0);
                                ui.horizontal(|ui| {
                                    let action = if self.timer.running {
                                        "Pause"
                                    } else {
                                        "Start focus"
                                    };
                                    let primary = egui::Button::new(
                                        RichText::new(action).size(16.0).color(Color32::WHITE),
                                    )
                                    .fill(accent)
                                    .corner_radius(14);
                                    if ui.add_sized([190.0, 44.0], primary).clicked() {
                                        self.timer.running = !self.timer.running;
                                        self.last_tick = Instant::now();
                                    }
                                    if ui
                                        .add_sized(
                                            [80.0, 44.0],
                                            egui::Button::new("Reset").corner_radius(14),
                                        )
                                        .clicked()
                                    {
                                        self.timer.reset();
                                    }
                                });
                                ui.add_space(16.0);
                                ui.label(
                                    RichText::new(format!(
                                        "{} focus sessions completed",
                                        self.timer.completed_sessions
                                    ))
                                    .size(13.0)
                                    .color(muted),
                                );
                            });
                        });
                    ui.add_space(20.0);
                    ui.label(
                        RichText::new("SESSION LENGTH")
                            .size(12.0)
                            .strong()
                            .color(muted),
                    );
                    ui.add_space(8.0);
                    ui.set_width(370.0);
                    let focus_changed = ui
                        .add(
                            egui::Slider::new(&mut self.timer.focus_minutes, 1..=120)
                                .text("Focus min"),
                        )
                        .changed();
                    let break_changed = ui
                        .add(
                            egui::Slider::new(&mut self.timer.break_minutes, 1..=60)
                                .text("Break min"),
                        )
                        .changed();
                    if !self.timer.running
                        && ((focus_changed && self.timer.phase == Phase::Focus)
                            || (break_changed && self.timer.phase == Phase::Break))
                    {
                        self.timer.reset();
                    }
                });
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([460.0, 660.0])
            .with_min_inner_size([430.0, 620.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Focus Timer",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::light());
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
