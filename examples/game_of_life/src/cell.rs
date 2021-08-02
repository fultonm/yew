#[derive(Clone, Copy, PartialEq)]
pub enum State {
    Alive,
    Dead,
}

#[derive(Clone, Copy)]
pub struct Cellule {
    pub state: State,
    pub render: bool,
}

impl Cellule {
    pub fn new_dead() -> Self {
        Self {
            state: State::Dead,
            render: false,
        }
    }

    pub fn set_alive(&mut self) {
        self.state = State::Alive;
        self.set_render_flag(true);
    }

    pub fn set_dead(&mut self) {
        self.state = State::Dead;
        self.set_render_flag(true);
    }

    pub fn is_alive(self) -> bool {
        self.state == State::Alive
    }

    pub fn toggle(&mut self) {
        if self.is_alive() {
            self.set_dead()
        } else {
            self.set_alive()
        }
    }

    pub fn set_render_flag(&mut self, render: bool) {
        self.render = render;
    }

    pub fn should_render(self) -> bool {
        return self.render;
    }

    pub fn count_alive_neighbors(neighbors: &[Self]) -> usize {
        neighbors.iter().filter(|n| n.is_alive()).count()
    }

    pub fn alone(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) < 2
    }

    pub fn overpopulated(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) > 3
    }

    pub fn can_be_revived(neighbors: &[Self]) -> bool {
        Self::count_alive_neighbors(neighbors) == 3
    }
}
