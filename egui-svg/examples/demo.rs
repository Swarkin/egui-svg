use egui_svg::EguiSvg;

fn main() {
	let svg = EguiSvg::new(1000, 1000).run(|ctx| {
		egui_demo_lib::DemoWindows::default().ui(ctx);
	});

	if option_env!("DRY_RUN").is_none() {
		std::fs::write("egui-svg/assets/egui.svg", svg).unwrap();
	} else {
		println!("{svg}");
	}
}
