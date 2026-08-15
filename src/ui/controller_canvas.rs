use iced::widget::canvas::{self, Frame, Path, Stroke};
use iced::{Color, Point, Rectangle, Renderer, Size, Theme, mouse};

use rkg_utils::input_data::{DPadButton, StickInput};

const OUTLINE_COLOR: Color = Color::WHITE;
const OUTLINE_BORDER_COLOR: Color = Color::BLACK;
const FILL_COLOR: Color = Color::TRANSPARENT;
const ACTIVE_COLOR: Color = Color::WHITE;
const DOT_COLOR: Color = Color::WHITE;

const OUTLINE_WIDTH: f32 = 5.0;
// Wider than `OUTLINE_WIDTH` and stroked first, so it peeks out on both
// sides of the white line as a thin black edge.
const OUTLINE_BORDER_WIDTH: f32 = OUTLINE_WIDTH + 6.0;

fn stroke_outlined(frame: &mut Frame, path: &Path) {
    frame.stroke(
        path,
        Stroke::default()
            .with_color(OUTLINE_BORDER_COLOR)
            .with_width(OUTLINE_BORDER_WIDTH),
    );
    frame.stroke(
        path,
        Stroke::default()
            .with_color(OUTLINE_COLOR)
            .with_width(OUTLINE_WIDTH),
    );
}

pub struct StickCanvas {
    pub stick: StickInput,
}

impl<Message> canvas::Program<Message> for StickCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let size = bounds.width.min(bounds.height) - OUTLINE_BORDER_WIDTH - 4.0;

        
        let radius = size * 0.34;
        let dot_radius = size * 0.21;
        let travel = size * 0.27;

        let octagon = octagon_path(center, radius);
        frame.fill(&octagon, FILL_COLOR);
        stroke_outlined(&mut frame, &octagon);

        let nx = (self.stick.x() as f32 - 7.0) / 7.0;
        let ny = (self.stick.y() as f32 - 7.0) / 7.0;
        let dot_center = Point::new(center.x + nx * travel, center.y - ny * travel);

        let dot = Path::circle(dot_center, dot_radius);
        frame.fill(&dot, DOT_COLOR);
        stroke_outlined(&mut frame, &dot);

        vec![frame.into_geometry()]
    }
}

fn octagon_path(center: Point, radius: f32) -> Path {
    Path::new(|p| {
        for i in 0..8 {
            let angle = (45.0 * i as f32 - 90.0).to_radians();
            let point = Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            if i == 0 {
                p.move_to(point);
            } else {
                p.line_to(point);
            }
        }
        p.close();
    })
}

pub struct DPadCanvas {
    pub dpad: DPadButton,
}

impl<Message> canvas::Program<Message> for DPadCanvas {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = frame.center();
        let reach = bounds.width.min(bounds.height) / 2.0 - 6.0;
        let half_width = reach * 0.27;

        let cross = cross_path(center, reach, half_width);
        frame.fill(&cross, FILL_COLOR);

        if let Some((top_left, size)) = active_arm_rect(center, reach, half_width, self.dpad) {
            frame.fill_rectangle(top_left, size, ACTIVE_COLOR);
        }

        stroke_outlined(&mut frame, &cross);

        vec![frame.into_geometry()]
    }
}

fn cross_path(center: Point, reach: f32, half_width: f32) -> Path {
    let (cx, cy) = (center.x, center.y);
    let points = [
        Point::new(cx - half_width, cy - reach),
        Point::new(cx + half_width, cy - reach),
        Point::new(cx + half_width, cy - half_width),
        Point::new(cx + reach, cy - half_width),
        Point::new(cx + reach, cy + half_width),
        Point::new(cx + half_width, cy + half_width),
        Point::new(cx + half_width, cy + reach),
        Point::new(cx - half_width, cy + reach),
        Point::new(cx - half_width, cy + half_width),
        Point::new(cx - reach, cy + half_width),
        Point::new(cx - reach, cy - half_width),
        Point::new(cx - half_width, cy - half_width),
    ];

    let corner_radius = half_width * 0.4;
    rounded_polygon_path(&points, corner_radius)
}

fn point_toward(from: Point, to: Point, distance: f32) -> Point {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let len = dx.hypot(dy);
    if len < f32::EPSILON {
        from
    } else {
        Point::new(from.x + dx / len * distance, from.y + dy / len * distance)
    }
}


fn rounded_polygon_path(points: &[Point], radius: f32) -> Path {
    let n = points.len();

    Path::new(|p| {
        let point_before = |i: usize| point_toward(points[i], points[(i + n - 1) % n], radius);
        let point_after = |i: usize| point_toward(points[i], points[(i + 1) % n], radius);

        p.move_to(point_before(0));
        for (i, &point) in points.iter().enumerate() {
            p.arc_to(point, point_after(i), radius);
            let next = (i + 1) % n;
            if next != 0 {
                p.line_to(point_before(next));
            }
        }
        p.close();
    })
}

fn active_arm_rect(
    center: Point,
    reach: f32,
    half_width: f32,
    dpad: DPadButton,
) -> Option<(Point, Size)> {
    let along_arm = Size::new(half_width * 2.0, reach - half_width);
    let across_arm = Size::new(reach - half_width, half_width * 2.0);

    match dpad {
        DPadButton::Up => Some((
            Point::new(center.x - half_width, center.y - reach),
            along_arm,
        )),
        DPadButton::Down => Some((
            Point::new(center.x - half_width, center.y + half_width),
            along_arm,
        )),
        DPadButton::Left => Some((
            Point::new(center.x - reach, center.y - half_width),
            across_arm,
        )),
        DPadButton::Right => Some((
            Point::new(center.x + half_width, center.y - half_width),
            across_arm,
        )),
        DPadButton::None | DPadButton::Unknown => None,
    }
}
