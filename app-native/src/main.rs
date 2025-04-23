//! Used to test the app on native to compare the visual result.

#[cfg(feature = "demo")]
use eframe::egui::{CentralPanel, Frame};

use eframe::egui::{Context, FontDefinitions, Vec2, ViewportBuilder};
use eframe::NativeOptions;

fn main() -> eframe::Result {
	let mut init = true;

	eframe::run_simple_native(
		"egui-svg",
		NativeOptions {
			viewport: ViewportBuilder::default()
				.with_inner_size(Vec2::splat(1000.0)),
			..Default::default()
		},
		move |ctx, _| {
			if init {
				// Make sure to use the same fonts for the svg and for egui.
				let mut fonts = FontDefinitions::empty();
				egui_svg_fonts::implement(&mut fonts);
				ctx.set_fonts(fonts);

				init = false;
			}

			run(ctx);
		},
	)
}


/// Used to test rendering of elements during development.
#[cfg(not(feature = "demo"))]
pub fn run(_ctx: &Context) {}

/// Used to test rendering of the egui demo project.
#[cfg(feature = "demo")]
pub fn run(ctx: &Context) {
	CentralPanel::default().frame(Frame::NONE).show(ctx, |_ui| {
		egui_demo_lib::DemoWindows::default().ui(ctx);
	});
}
