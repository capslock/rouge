use bracket_lib::terminal::Rect;

/// Specifies the layout of [`Widget`]s in a container.
#[derive(Debug)]
pub struct Layout {
    x: AlignX,
    y: AlignY,
}

impl Layout {
    /// Create a new layout with the given alignments.
    pub fn new(x: AlignX, y: AlignY) -> Self {
        Self { x, y }
    }

    /// Allocates some space in `containing_rect` according to the given
    /// alignments. `x` and `y` specify how much space should be allocated in
    /// their respective dimensions.
    pub fn allocate_aligned(&self, containing_rect: Rect, x: i32, y: i32) -> Rect {
        let mut rect = Rect::default();
        match self.x {
            AlignX::Left => {
                rect.x1 = containing_rect.x1;
                rect.x2 = containing_rect.x1 + x;
            }
            AlignX::Right => {
                rect.x1 = containing_rect.x2 - x;
                rect.x2 = containing_rect.x2;
            }
            AlignX::Center => {
                let center = containing_rect.center();
                rect.x1 = center.x - x / 2;
                rect.x2 = center.x + x / 2 + x % 2;
            }
        }
        match self.y {
            AlignY::Top => {
                rect.y1 = containing_rect.y1;
                rect.y2 = containing_rect.y1 + y;
            }
            AlignY::Bottom => {
                rect.y1 = containing_rect.y2 - y;
                rect.y2 = containing_rect.y2;
            }
            AlignY::Center => {
                let center = containing_rect.center();
                rect.y1 = center.y - y / 2;
                rect.y2 = center.y + y / 2 + y % 2;
            }
        }
        rect
    }
}

/// Alignment in the `X` dimension.
#[allow(unused)]
#[derive(Debug)]
pub enum AlignX {
    Left,
    Right,
    Center,
}

/// Alignment in the `Y` dimension.
#[allow(unused)]
#[derive(Debug)]
pub enum AlignY {
    Top,
    Bottom,
    Center,
}
