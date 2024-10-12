use rand::{thread_rng, Fill};
use sfml::{
    graphics::{
        Color, ConvexShape, Rect, RectangleShape, RenderTarget, RenderWindow, Shape, Transformable,
        View,
    },
    window::{Event, Key, Style},
};

type Item = u8;
const ITEM_COUNT: usize = 32;

#[derive(Debug)]
enum Step {
    Mark,
    Compare,
    SwapHappened,
    Next,
    Finished,
}

struct Sim {
    items: [Item; ITEM_COUNT],
    cursor: usize,
    step: Step,
    swapped_any: bool,
    pass: usize,
}

impl Sim {
    fn new_randomized() -> Self {
        let mut rng = thread_rng();
        let mut items = [0; ITEM_COUNT];
        items.try_fill(&mut rng).unwrap();
        Self {
            items,
            cursor: 0,
            step: Step::Mark,
            swapped_any: false,
            pass: 0,
        }
    }
    fn advance(&mut self) {
        match self.step {
            Step::Mark => self.step = Step::Compare,
            Step::Compare => {
                if self.items[self.cursor] > self.items[self.cursor + 1] {
                    self.items.swap(self.cursor, self.cursor + 1);
                    self.step = Step::SwapHappened;
                    self.swapped_any = true;
                } else {
                    self.step = Step::Next;
                }
            }
            Step::SwapHappened => {
                self.step = Step::Next;
            }
            Step::Next => {
                self.cursor += 1;
                if self.cursor >= self.items.len() - (1 + self.pass) {
                    if self.swapped_any {
                        self.cursor = 0;
                        self.swapped_any = false;
                        self.step = Step::Mark;
                        self.pass += 1;
                    } else {
                        self.step = Step::Finished;
                    }
                } else {
                    self.step = Step::Mark;
                }
            }
            Step::Finished => {}
        }
    }
    fn draw(&self, win: &mut RenderWindow) {
        let ws = win.size();
        let h_margin = 16.0;
        let v_margin = 64.0;
        let left = h_margin;
        let top = v_margin;
        let bottom = ws.y as f32 - v_margin;
        let right = ws.x as f32 - h_margin;
        let width = right - left;
        let gap_ratio = 0.25;
        let column_w = width / (ITEM_COUNT as f32 * (1.0 + gap_ratio));
        let gap = column_w * gap_ratio;
        let column_h = bottom - v_margin;
        let mut cursor_arrow = ConvexShape::new(3);
        let arrow_h = column_h / 24.0;
        cursor_arrow.set_point(0, (column_w / 2., 0.));
        cursor_arrow.set_point(1, (0., arrow_h));
        cursor_arrow.set_point(2, (column_w, arrow_h));
        let x = |i| left + (i as f32 * (column_w + gap));
        cursor_arrow.set_position((x(self.cursor), bottom + 8.0));
        for (i, &value) in self.items.iter().enumerate() {
            let ratio = value as f32 / Item::MAX as f32;
            let y_offset = column_h - column_h * ratio;
            let mut rs = RectangleShape::from_rect(Rect::new(
                x(i),
                top + y_offset,
                column_w,
                column_h * ratio,
            ));
            rs.set_outline_thickness(2.0);
            rs.set_fill_color(Color::rgb(128, 128, 128));
            let oc = match (&self.step, i as i32 - self.cursor as i32) {
                (Step::Mark, 0) => Color::YELLOW,
                (Step::Compare, 0) => Color::YELLOW,
                (Step::Compare, 1) => Color::GREEN,
                (Step::SwapHappened, 0) => Color::GREEN,
                (Step::SwapHappened, 1) => Color::YELLOW,
                _ => Color::TRANSPARENT,
            };
            rs.set_outline_color(oc);
            win.draw(&rs);
            if !matches!(self.step, Step::Finished) {
                win.draw(&cursor_arrow);
            }
        }
    }
}

fn main() {
    let mut window = RenderWindow::new(
        (800, 600),
        "Bubble sort visualization",
        Style::default(),
        &Default::default(),
    )
    .unwrap();
    window.set_vertical_sync_enabled(true);
    let mut sim = Sim::new_randomized();
    let mut pause = true;

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                Event::Resized { width, height } => {
                    let area = Rect::new(0., 0., width as f32, height as f32);
                    window.set_view(&View::from_rect(area));
                }
                Event::KeyPressed {
                    code: Key::Space, ..
                } => pause ^= true,
                Event::KeyPressed {
                    code: Key::Right, ..
                } if pause => sim.advance(),
                _ => {}
            }
        }
        if !pause {
            sim.advance();
        }
        window.clear(Color::BLACK);
        sim.draw(&mut window);
        window.display();
    }
}
