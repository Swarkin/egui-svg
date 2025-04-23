#![warn(missing_docs)]

//! # egui-svg
//!
//! a crate that allows you to export a static, noninteractive svg from the output of egui.
//!
//! see `README.md` for more information.

mod svg;

use egui::{Context, FontDefinitions, FullOutput, Pos2, RawInput, Rect};

/// The `EguiSvg` struct which provides methods to run egui and export an svg.
pub struct EguiSvg {
	ctx: Context,
	resolution: (u32, u32),
}

impl EguiSvg {
	/// Create a new EguiSvg instance.
	/// Does not need to be re-used, but doing so can be slightly more efficient.
	pub fn new(res_x: u32, res_y: u32) -> Self {
		let ctx = Context::default();
		ctx.style_mut(|s| s.animation_time = 0.0 );
		configure_fonts(&ctx);
		Self { ctx, resolution: (res_x, res_y) }
	}

	/// Runs `ctx.run` at least twice (egui does not output some elements on the very first frame!) and converts the result of the last call to an svg.
	/// This behavior currently cannot be disabled.
	pub fn run(&self, mut ui_func: impl FnMut(&Context) + Sized) -> String {
		let _ = self.ctx.run(self.raw_input(), |ctx| ui_func(ctx)); // run and discard first iteration
		let output = self.final_output(|ctx| ui_func(ctx));
		self.generate_svg(output.shapes)
	}

	/// Runs `ctx.run` until `platform_output.requested_discard` is false.
	fn final_output(&self, mut run_ui: impl FnMut(&Context) + Sized) -> FullOutput {
		loop {
			let output = self.ctx.run(self.raw_input(), |ctx| run_ui(ctx));
			if !output.platform_output.requested_discard() {
				return output;
			} else {
				println!("discarding: {:?}", output.platform_output.request_discard_reasons);
			}
		}
	}

	fn raw_input(&self) -> RawInput {
		RawInput {
			screen_rect: Some(Rect::from_points(&[Pos2::ZERO, Pos2::new(self.resolution.0 as f32, self.resolution.1 as f32)])),
			..Default::default()
		}
	}
}

/// Should be used to configure egui's fonts.
pub fn configure_fonts(ctx: &Context) {
	let mut fonts = FontDefinitions::empty();
	egui_svg_fonts::implement(&mut fonts);
	ctx.set_fonts(fonts);
}
