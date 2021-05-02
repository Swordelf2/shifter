//! All svgs are assumed to have a seperate layer called `data`, which contains
//! groups of paths such as `collision` and `player_start_pos`

use svg::node::element::path::{Command, Data, Position};
use svg::node::element::tag;
use svg::node::element::tag::Type;
use svg::parser::Event;

use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use bevy::math::Vec2;
use bevy::reflect::TypeUuid;
use bevy::utils::BoxedFuture;

use crate::config::conversion::SVG_TO_UNITS;
use crate::util::shape::{CircleShape, PolyShape, Shape};

/// Asset type, which holds additional svg data for an object
#[derive(Default, Debug, TypeUuid)]
#[uuid = "737b3336-aa6f-11eb-bcbc-0242ac130002"]
pub struct SvgData {
    size: Vec2,
    groups: Vec<(String, Vec<Shape>)>,
}

#[derive(Default)]
pub struct SvgDataLoader;

const LABEL_ATTR: &'static str = "inkscape:label";
const DATA_LABEL: &'static str = "data";

enum State {
    OutsideData,
    InsideData,
    InsideGroup,
}

impl AssetLoader for SvgDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            // Parse the svg
            let parser =
                svg::read(std::str::from_utf8(bytes).unwrap()).unwrap();
            let mut size: Option<Vec2> = None;
            let mut groups: Vec<(String, Vec<Shape>)> = Vec::new();
            let mut state = State::OutsideData;
            let mut cur_group_name: Option<String> = None;
            let mut cur_group: Option<Vec<Shape>> = None;
            for event in parser {
                match event {
                    Event::Tag(tag::SVG, Type::Start, attr) => {
                        size = Some(Vec2::new(
                            attr.get("width").unwrap().parse().unwrap(),
                            attr.get("height").unwrap().parse().unwrap(),
                        ));
                    }
                    Event::Tag(tag::Group, Type::Start, attr) => {
                        if let Some(group_label) = attr.get(LABEL_ATTR) {
                            match state {
                                State::OutsideData => {
                                    if **group_label == *DATA_LABEL {
                                        state = State::InsideData;
                                    }
                                }
                                State::InsideData => {
                                    // Start new group
                                    cur_group_name =
                                        Some(String::from(&**group_label));
                                    cur_group = Some(Vec::new());
                                    state = State::InsideGroup;
                                }
                                State::InsideGroup => {
                                    panic!("A group inside a path group");
                                }
                            }
                        }
                    }
                    Event::Tag(tag::Group, Type::End, _) => {
                        match state {
                            State::OutsideData => {}
                            State::InsideData => {
                                state = State::OutsideData;
                            }
                            State::InsideGroup => {
                                // Finish the current group
                                groups.push((
                                    cur_group_name.take().unwrap(),
                                    cur_group.take().unwrap(),
                                ));
                                state = State::InsideData;
                            }
                        }
                    }
                    Event::Tag(tag::Path, _, attr) => {
                        match state {
                            State::OutsideData => {}
                            State::InsideData => {
                                panic!("path in data without a group");
                            }
                            State::InsideGroup => {
                                // Add the path to the current group
                                cur_group.as_mut().unwrap().push(
                                    path_to_shape(
                                        Data::parse(attr.get("d").unwrap())
                                            .unwrap(),
                                        size.unwrap(),
                                    ),
                                );
                            }
                        }
                    }
                    Event::Tag(tag::Circle, _, attr) => match state {
                        State::OutsideData => {}
                        State::InsideData => {
                            panic!("circle in data without a group");
                        }
                        State::InsideGroup => {
                            cur_group.as_mut().unwrap().push(Shape::Circle(
                                CircleShape {
                                    radius: attr
                                        .get("r")
                                        .unwrap()
                                        .parse::<f32>()
                                        .unwrap()
                                        * SVG_TO_UNITS,
                                    center: to_centered(
                                        dbg!(Vec2::new(
                                            attr.get("cx")
                                                .unwrap()
                                                .parse()
                                                .unwrap(),
                                            attr.get("cy")
                                                .unwrap()
                                                .parse()
                                                .unwrap(),
                                        )),
                                        size.unwrap(),
                                    ),
                                },
                            ));
                        }
                    },
                    _ => {}
                }
            }

            load_context.set_default_asset(LoadedAsset::new(SvgData {
                size: size.unwrap(),
                groups,
            }));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["svg"]
    }
}

/// The points in the svg are given in the top left coordinate system,
/// so all of them will need to be transformed into the centered
/// coordinate system.  
/// Also, they need to be scaled with [SVG_TO_UNITS] first.
///
/// `size` is the size of the canvas
#[inline]
fn to_centered(mut point: Vec2, size: Vec2) -> Vec2 {
    point *= SVG_TO_UNITS;

    Vec2::new(point.x - size.x * 0.5, size.y * 0.5 - point.y)
}

fn path_to_shape(data: Data, size: Vec2) -> Shape {
    let mut poly_shape = PolyShape::default();
    let mut last_point: Option<Vec2> = None;
    for command in data.iter() {
        match command {
            Command::Move(Position::Absolute, parameters) => {
                last_point = Some(Vec2::new(parameters[0], parameters[1]));
            }
            Command::Line(Position::Absolute, parameters) => {
                last_point = Some(Vec2::new(parameters[0], parameters[1]));
            }
            Command::Line(Position::Relative, parameters) => {
                last_point = Some(Vec2::new(
                    last_point.unwrap().x + parameters[0],
                    last_point.unwrap().y + parameters[1],
                ));
            }
            Command::HorizontalLine(Position::Absolute, parameters) => {
                last_point =
                    Some(Vec2::new(parameters[0], last_point.unwrap().y));
            }
            Command::HorizontalLine(Position::Relative, parameters) => {
                last_point = Some(Vec2::new(
                    last_point.unwrap().x + parameters[0],
                    last_point.unwrap().y,
                ));
            }
            Command::VerticalLine(Position::Absolute, parameters) => {
                last_point =
                    Some(Vec2::new(last_point.unwrap().x, parameters[0]));
            }
            Command::VerticalLine(Position::Relative, parameters) => {
                last_point = Some(Vec2::new(
                    last_point.unwrap().x,
                    last_point.unwrap().y + parameters[0],
                ));
            }
            Command::Close => last_point = None,
            _ => panic!("Unsupported path command"),
        }
        if let Some(last_point) = last_point {
            poly_shape.points.push(to_centered(last_point, size));
        }
    }

    Shape::Poly(poly_shape)
}
