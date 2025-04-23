use crate::EguiSvg;
use egui::epaint::{ClippedShape, ColorMode, PathStroke};
use egui::{Color32, FontFamily, Shape, Stroke};
use std::collections::HashMap;

/// HashMap of svg element keys.
type Attributes = HashMap<&'static str, String>;

impl EguiSvg {
	/// Generates the svg from the given shapes.
	pub fn generate_svg(&self, shapes: Vec<ClippedShape>) -> String {
		let mut svg = String::with_capacity(shapes.len() * 32); // rough estimate
		svg.push_str(&format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\">", self.resolution.0, self.resolution.1));
		let mut attrs = Attributes::new();

		shapes_to_svg(shapes.into_iter().map(|x| x.shape).collect(), &mut attrs, &mut svg);

		svg.push_str("</svg>");
		svg
	}
}

/// Recursive function that applies a vector of egui `Shape`s on the given `svg` String.
fn shapes_to_svg(shapes: Vec<Shape>, attrs: &mut Attributes, svg: &mut String) {
	for shape in shapes {
		debug_assert!(attrs.is_empty());
		match shape {
			Shape::Noop => {}
			Shape::Vec(shapes) => shapes_to_svg(shapes, attrs, svg),
			Shape::Circle(circle) => {
				attrs.insert("r", circle.radius.to_string());
				attrs.insert("cx", circle.center.x.to_string());
				attrs.insert("cy", circle.center.y.to_string());

				attrs.insert("fill", circle.fill.to_hex());
				insert_stroke(circle.stroke, attrs);

				apply_attrs("circle", svg, attrs, None);
			}
			Shape::Rect(rect) => {
				let stroke_empty = rect.stroke.is_empty();

				if !rect.rect.is_positive() || (rect.fill.is_additive() && stroke_empty) {
					// skip invisible rects
					continue;
				}

				let snap_to_half;

				if !stroke_empty {
					insert_stroke(rect.stroke, attrs);

					if rect.stroke.width as u32 % 2 == 1 {
						// we need to position the rect at a .5 step to achieve pixel-perfect rendering
						snap_to_half = true;
					} else {
						// same as above but position at .0 step
						snap_to_half = false;
					}
				} else {
					snap_to_half = false;
				}

				let mut x = rect.rect.max.x - rect.rect.min.x;
				let mut y = rect.rect.max.y - rect.rect.min.y;

				if !stroke_empty {
					if snap_to_half {
						x = x.round() - 1.0;
						y = y.round() - 1.0;
					} else {
						x = snap_forward_half(x);
						y = snap_forward_half(y);
					}
				}

				attrs.insert("width", x.to_string());
				attrs.insert("height", y.to_string());

				if rect.rect.min.x != 0.0 {
					let x = if snap_to_half { snap_forward_half(rect.rect.min.x) } else { rect.rect.min.x.round() };
					attrs.insert("x", x.to_string());
				}

				if rect.rect.min.y != 0.0 {
					let y = if snap_to_half { snap_forward_half(rect.rect.min.y) } else { rect.rect.min.y.round() };
					attrs.insert("y", y.to_string());
				}

				if rect.fill != Color32::BLACK {
					attrs.insert("fill", rect.fill.to_hex());
				}

				if rect.corner_radius.nw > 0 {
					attrs.insert("rx", rect.corner_radius.nw.to_string());
				}

				apply_attrs("rect", svg, attrs, None);
			}
			Shape::Text(text) => {
				// currently no section support and only static row heights
				let first_section = &text.galley.job.sections[0];
				for row in &text.galley.rows {
					let line = row.glyphs.iter().map(|x| x.chr).collect::<String>();

					attrs.insert("x", (text.pos.x + row.rect.min.x).to_string());
					attrs.insert("y", (text.pos.y + row.rect.max.y - 3.0).to_string());
					attrs.insert("font-size", first_section.format.font_id.size.to_string());
					attrs.insert("font-family",
						if first_section.format.font_id.family == FontFamily::Proportional { "p" }
						else { "m" }.into()
					);

					/* figure out font color */ {
						let font_color = if first_section.format.color != Color32::PLACEHOLDER {
							first_section.format.color
						} else if text.fallback_color != Color32::PLACEHOLDER {
							text.fallback_color
						} else {
							println!("couldnt find font color");
							Color32::PLACEHOLDER
						};

						attrs.insert("fill", font_color.to_hex());
					}

					apply_attrs("text", svg, attrs, Some(&line));
				}
			}
			Shape::LineSegment { points, stroke } => {
				attrs.insert("x1", points[0].x.to_string());
				attrs.insert("y1", snap_forward_half(points[0].y).to_string());
				attrs.insert("x2", points[1].x.to_string());
				attrs.insert("y2", snap_forward_half(points[1].y).to_string());
				attrs.insert("stroke", stroke.color.to_hex());
				attrs.insert("stroke-width", stroke.width.to_string());

				apply_attrs("line", svg, attrs, None);
			}
			Shape::Path(path) => {
				let mut pathdata = String::with_capacity(path.points.len() * 6); // estimate

				if let Some((first, points)) = path.points.split_first() {
					// move to absolute
					pathdata.push_str(&format!("M{} {} ", first.x, first.y));

					// line to absolute
					for point in points {
						pathdata.push_str(&format!("{} {} ", point.x, point.y));
					}

					if path.closed {
						// close path absolute
						pathdata.push('Z');
					}
				}

				attrs.insert("d", pathdata);

				if !path.stroke.is_empty() {
					insert_pathstroke(&path.stroke, attrs);
				}

				if path.fill != Color32::BLACK {
					attrs.insert("fill", path.fill.to_hex());
				}

				apply_attrs("path", svg, attrs, None);
			}
			Shape::Mesh(mesh) => {
				// textures and UVs are not supported
				mesh.indices.iter()
					.map(|x| *x as usize)
					.collect::<Vec<_>>()
					.chunks_exact(3)
					.for_each(|tri| {
						let points = tri.iter().map(|v| {
							let v = mesh.vertices[*v];
							format!("{:.3},{:.3}", v.pos.x, v.pos.y)
						}).collect::<Vec<_>>().join(" ");

						attrs.insert("points", points);

						// use color of first vert
						let color = mesh.vertices[tri[0]].color;
						if color != Color32::BLACK {
							attrs.insert("fill", color.to_hex());
						}

						// used to avoid seams in polygons
						attrs.insert("shape-rendering", "crispEdges".to_string());

						apply_attrs("polygon", svg, attrs, None);
					});
			}
			_ => {
				println!("unsupported shape:\n{shape:?}");
				continue;
			}
		}
	}
}

/// Inserts the attributes to mimic [`Stroke`].
fn insert_stroke(stroke: Stroke, attrs: &mut Attributes) {
	attrs.insert("stroke", stroke.color.to_hex());
	attrs.insert("stroke-width", stroke.width.to_string());
}

/// Inserts the attributes to mimic [`PathStroke`].
///
/// Does not support [`ColorMode::UV`] and falls back to [`StrokeKind::Middle`].
fn insert_pathstroke(stroke: &PathStroke, attrs: &mut Attributes) {
	if let ColorMode::Solid(color) = stroke.color {
		attrs.insert("stroke", color.to_hex());
	} else {
		return;
	}

	attrs.insert("stroke-width", stroke.width.to_string());
}

/// Drains the `attrs` and writes them directly into the `svg` inside a new `elem` element.
fn apply_attrs(elem: &str, svg: &mut String, attrs: &mut Attributes, content: Option<&str>) {
	svg.push('<');
	svg.push_str(elem);

	for (key, value) in attrs.drain() {
		svg.push(' ');
		svg.push_str(key);
		svg.push_str("=\"");
		svg.push_str(&value);
		svg.push('"');
	}

	if let Some(content) = content {
		svg.push('>');
		svg.push_str(content);
		svg.push_str("</");
		svg.push_str(elem);
		svg.push('>');
	} else {
		svg.push_str("/>");
	}
}

/// Used to fix blurriness.
fn snap_forward_half(value: f32) -> f32 {
	let base = value.floor();
	if value < base + 0.5 {
		base + 0.5
	} else {
		base + 1.5
	}
}
