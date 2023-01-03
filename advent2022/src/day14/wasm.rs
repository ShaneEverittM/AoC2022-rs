use derive_more::{Add, AddAssign, Sub};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use web_sys::ImageData;

use std::{fmt, iter};

const SOURCE: Point = Point { x: 500, y: 0 };
const AIR_COLOR: [u8; 3] = [0xFF, 0xFF, 0xFF];
const ROCK_COLOR: [u8; 3] = [0x80, 0x84, 0x87];
const SAND_COLOR: [u8; 3] = [0xC2, 0xB2, 0x80];
const CURRENT_COLOR: [u8; 3] = [0xF5, 0xCE, 0x31];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Add, AddAssign, Sub)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn parse(s: &str) -> Self {
        let mut tokens = s.split(',');
        let (x, y) = (tokens.next().unwrap(), tokens.next().unwrap());
        Self {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }

    fn signum(self) -> Self {
        Self {
            x: self.x.signum(),
            y: self.y.signum(),
        }
    }
}

#[derive(Debug)]
struct Polyline {
    points: Vec<Point>,
}

impl Polyline {
    fn parse(s: &str) -> Self {
        Self {
            points: s.split(" -> ").map(Point::parse).collect(),
        }
    }

    fn path_points(&self) -> impl Iterator<Item = Point> + '_ {
        iter::from_generator(|| {
            let mut points = self.points.iter().copied();
            let Some(mut a) = points.next() else { return };
            yield a;

            loop {
                let Some(b) = points.next() else { return };
                let delta = (b - a).signum();
                assert!((delta.x == 0) ^ (delta.y == 0));

                loop {
                    a += delta;
                    yield a;
                    if a == b {
                        break;
                    }
                }
            }
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Cell {
    Air,
    Rock,
    Sand,
}

#[wasm_bindgen]
pub struct Grid {
    origin: Point,
    width: usize,
    height: usize,
    cells: Vec<Cell>,
    settled: usize,
    grains: Vec<Point>,
    floor: bool,
}

impl Grid {
    pub fn parse(input: &str, floor: bool) -> Self {
        let mut polylines: Vec<_> = input.lines().map(Polyline::parse).collect();
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        // Find bounding coordinates
        let source = Point { x: 500, y: 0 };
        for point in polylines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(iter::once(&source))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        if floor {
            let floor_y = max_y + 2;
            min_x = 300;
            max_x = 700;
            max_y = floor_y;
            polylines.push(Polyline {
                points: vec![
                    Point {
                        x: min_x,
                        y: floor_y,
                    },
                    Point {
                        x: max_x,
                        y: floor_y,
                    },
                ],
            });
        }

        // Compute dimensions
        let origin = Point { x: min_x, y: min_y };
        let width: usize = (max_x - min_x + 1).try_into().unwrap();
        let height: usize = (max_y - min_y + 1).try_into().unwrap();

        // Make the grid
        let mut grid = Self {
            origin,
            width,
            height,
            cells: vec![Cell::Air; width * height],
            settled: 0,
            grains: vec![],
            floor,
        };

        // Place the rocks
        for point in polylines.iter().flat_map(|p| p.path_points()) {
            *grid.cell_mut(point).unwrap() = Cell::Rock;
        }

        grid
    }

    fn cell_index(&self, point: Point) -> Option<usize> {
        let Point { x, y } = point - self.origin;
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    fn cell(&self, point: Point) -> Option<&Cell> {
        Some(&self.cells[self.cell_index(point)?])
    }

    fn cell_mut(&mut self, point: Point) -> Option<&mut Cell> {
        let cell_index = self.cell_index(point)?;
        Some(&mut self.cells[cell_index])
    }

    pub fn num_settled(&self) -> usize {
        self.settled
    }

    pub fn step(&mut self) -> bool {
        let mut grains = std::mem::take(&mut self.grains);
        let _ = grains
            .drain_filter(|grain| {
                let straight_down = *grain + Point { x: 0, y: 1 };
                let down_left = *grain + Point { x: -1, y: 1 };
                let down_right = *grain + Point { x: 1, y: 1 };
                let options = [straight_down, down_left, down_right];

                if let Some(pos) = options
                    .into_iter()
                    .find(|pos| matches!(self.cell(*pos), Some(Cell::Air)))
                {
                    *grain = pos;
                    return false;
                }

                if options.into_iter().any(|pos| self.cell(pos).is_none()) {
                    return true;
                }

                self.settled += 1;
                *self.cell_mut(*grain).unwrap() = Cell::Sand;
                true
            })
            .count();
        self.grains = grains;
        self.grains.push(SOURCE);

        // Check done conditions
        if !self.floor {
            self.grains.iter().any(|p| p.y == (self.height - 1) as i32)
        } else {
            matches!(self.cell(Point { x: 500, y: 0 }).unwrap(), Cell::Sand)
        }
    }
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    pub fn new(floor: bool) -> Self {
        Self::parse(include_str!("../inputs/day14.txt"), floor)
    }

    #[wasm_bindgen]
    pub fn render(&self, canvas_id: &str) {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id(canvas_id).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        canvas.set_width(self.width as _);
        canvas.set_height(self.height as _);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        let mut pixels = vec![0u8; 4 * self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let base_index = 4 * (y * self.width + x);
                pixels[base_index] = 255;
                pixels[base_index + 1] = 20;
                pixels[base_index + 2] = 20;
                pixels[base_index + 3] = 255;
            }
        }

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let cell = self.cell(point).unwrap();
                let color = match cell {
                    Cell::Air => &AIR_COLOR,
                    Cell::Rock => &ROCK_COLOR,
                    Cell::Sand => &SAND_COLOR,
                };

                let base_index = 4 * (y * self.width + x);
                pixels[base_index] = color[0];
                pixels[base_index + 1] = color[1];
                pixels[base_index + 2] = color[2];
            }
        }

        for grain in self.grains.iter().copied() {
            let Point { x, y } = grain - self.origin;

            let color = &CURRENT_COLOR;
            let base_index = 4 * (y as usize * self.width + x as usize);
            pixels[base_index] = color[0];
            pixels[base_index + 1] = color[1];
            pixels[base_index + 2] = color[2];
        }

        context
            .put_image_data(
                &ImageData::new_with_u8_clamped_array(Clamped(&pixels[..]), self.width as _)
                    .unwrap(),
                0.0,
                0.0,
            )
            .unwrap();
    }

    #[wasm_bindgen(js_name = step)]
    pub fn js_step(&mut self) -> bool {
        self.step()
    }

    #[wasm_bindgen(js_name = num_settled)]
    pub fn js_num_settled(&self) -> usize {
        self.num_settled()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let cell = self.cell(point).unwrap();
                let c = match cell {
                    Cell::Air => '.',
                    Cell::Rock => '#',
                    Cell::Sand => 'o',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
