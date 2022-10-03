use derivative::Derivative;

use crate::grid::Grid;

pub enum Action {
    Paint(Vec<(usize, usize)>, [u8; 4]),
    Resize(usize, usize),
    Clear,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Paint(g, _) => write!(f, "paint {} cells", g.len()),
            Action::Resize(w, h) => write!(f, "resize to {}x{}", w, h),
            Action::Clear => write!(f, "clear"),
        }
    }
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct Undo {
    stack: Vec<(Grid, Action)>,
    pointer: usize,
}

impl Undo {
    pub fn new() -> Self { Self::default() }

    pub fn push(&mut self, act: Action, grid: &Grid) {
        // If the pointer is not at the end of the stack, remove all
        // the elements after the pointer
        if self.pointer != self.stack.len() {
            self.stack.truncate(self.pointer);
        }

        // Push the action to the stack
        self.stack.push((grid.clone(), act));

        // Increment the pointer
        self.pointer += 1;
    }

    pub fn undo(&mut self, grid: &mut Grid) -> Option<&Action> {
        // If there is no more undo, do nothing
        if self.pointer == 0 { return None; }

        self.pointer -= 1;

        // Perform the action
        let act = &self.stack[self.pointer];
        match act {
            (_, Action::Paint(coords, _)) => {
                // Get the previous grid state
                if self.pointer > 0 {
                    let prev_grid = &self.stack[self.pointer - 1].0;
                    grid.replace(prev_grid);
                } else {
                    // If there is no previous color, then just erase the cell
                    for (x, y) in coords {
                        grid.erase(*x, *y);
                    }
                }
            }
            (g, Action::Resize(_, _)) => {
                grid.resize(g.width, g.height);
            }
            (_, Action::Clear) => {
                if self.pointer == 0 { return None; }
                // If the action is clear, then just copy the previous grid
                *grid = self.stack[self.pointer - 1].0.clone();
            }
        }

        return Some(&act.1);
    }

    pub fn redo(&mut self, grid: &mut Grid) -> Option<&Action> {
        // If there is no more redo, do nothing
        if self.pointer == self.stack.len() { return None; }

        self.pointer += 1;

        // Perform the action
        let act = &self.stack[self.pointer - 1];
        match act {
            (_, Action::Paint(coords, color)) => {
                for (x, y) in coords {
                    grid.set(*x, *y, *color);
                }
            }
            (_, Action::Resize(w, h)) => {
                grid.resize(*w, *h);
            }
            (_, Action::Clear) => {
                grid.clear();
            }
        }

        return Some(&act.1);
    }

    pub fn clear(&mut self) { *self = Self::default(); }
}
