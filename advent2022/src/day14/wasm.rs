use derive_more::{Add, AddAssign, Sub};
use wasm_bindgen::{prelude::*, JsCast};

use std::{
    fmt,
    iter::{self, once},
};

const SOURCE: Point = Point { x: 500, y: 0 };
const SAND_COLOR: &str = "#C2B280";
const ROCK_COLOR: &str = "#808487";
const AIR_COLOR: &str = "#FFFFFF";

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
}

#[wasm_bindgen]
impl Grid {
    #[wasm_bindgen(constructor)]
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self::parse(include_str!("../inputs/day14.txt"))
    }

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

        for y in 0..self.height {
            for x in 0..self.width {
                let point = Point {
                    x: x as _,
                    y: y as _,
                } + self.origin;
                let cell = self.cell(point).unwrap();
                let color = match cell {
                    Cell::Air => {
                        if self.grains.contains(&point) {
                            SAND_COLOR
                        } else {
                            AIR_COLOR
                        }
                    }
                    Cell::Rock => ROCK_COLOR,
                    Cell::Sand => SAND_COLOR,
                };
                context.set_fill_style(&JsValue::from_str(color));
                context.fill_rect(x as _, y as _, 1.0, 1.0);
            }
        }
    }

    pub fn parse(input: &str) -> Self {
        let polylines: Vec<_> = input.lines().map(Polyline::parse).collect();
        let (mut min_x, mut min_y, mut max_x, mut max_y) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        let source = Point { x: 500, y: 0 };
        for point in polylines
            .iter()
            .flat_map(|p| p.points.iter())
            .chain(once(&source))
        {
            min_x = min_x.min(point.x);
            min_y = min_y.min(point.y);
            max_x = max_x.max(point.x);
            max_y = max_y.max(point.y);
        }

        let origin = Point { x: min_x, y: min_y };
        let width: usize = (max_x - min_x + 1).try_into().unwrap();
        let height: usize = (max_y - min_y + 1).try_into().unwrap();
        let mut grid = Self {
            origin,
            width,
            height,
            cells: vec![Cell::Air; width * height],
            settled: 0,
            grains: vec![],
        };

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

    #[wasm_bindgen]
    pub fn step(&mut self) -> bool {
        let mut grains = std::mem::take(&mut self.grains);
        let _ = grains
            .drain_filter(|grain| {
                let straight_down = *grain + Point { x: 0, y: 1 };
                let down_left = *grain + Point { x: -1, y: 1 };
                let down_right = *grain + Point { x: 1, y: 1 };
                let options = [straight_down, down_left, down_right];

                // Can we move?
                if let Some(pos) = options
                    .into_iter()
                    .find(|pos| matches!(self.cell(*pos), Some(Cell::Air)))
                {
                    *grain = pos;
                    return false; // keep it
                }

                // If not, are we moving off-screen?
                if options.into_iter().any(|pos| self.cell(pos).is_none()) {
                    return true; // remove it
                }

                // If not, then we've settled
                self.settled += 1;
                *self.cell_mut(*grain).unwrap() = Cell::Sand;
                true // remove it
            })
            .count();
        self.grains = grains;
        self.grains.push(SOURCE);
        self.grains.iter().any(|p| p.y == (self.height - 1) as i32)
    }

    // #[wasm_bindgen(js_name = step)]
    // pub fn js_step(&mut self) -> bool {
    //     self.step()
    // }
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
