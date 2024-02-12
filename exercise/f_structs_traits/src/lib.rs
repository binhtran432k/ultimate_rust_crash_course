pub trait Bite {
    fn bite(&mut self);
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
pub struct Carrot {
    percent_left: f32,
}

impl Carrot {
    pub fn new(percent_left: f32) -> Self {
        Self { percent_left }
    }
}

impl Bite for Carrot {
    fn bite(&mut self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}

#[derive(Debug)]
pub struct Grapes {
    amount_left: i32,
}

impl Grapes {
    pub fn new(amount_left: i32) -> Self {
        Self { amount_left }
    }
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.amount_left -= 1;
    }
}

pub fn bunny_nibbles<T: Bite>(item: &mut T) {
    item.bite();
    item.bite();
    item.bite();
}
