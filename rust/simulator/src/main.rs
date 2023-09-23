pub struct Example {
    something: u32,
    flag: bool,
}

pub trait ConditionalAction {
    fn action<F: FnOnce(&mut Example) -> bool, G: FnOnce(&mut Example) -> bool>(
        &mut self,
        f: F,
        g: G,
    ) -> bool;
}

impl ConditionalAction for Example {
    fn action<F: FnOnce(&mut Example) -> bool, G: FnOnce(&mut Example) -> bool>(
        &mut self,
        f: F,
        g: G,
    ) -> bool {
        if self.flag {
            f(self)
        } else {
            g(self)
        }
    }
}

fn main() {
    let mut example = Example {
        something: 10,
        flag: true,
    };

    example.action(
        |e| {
            e.something = 32;
            true
        },
        |e| {
            e.something = 42;
            true
        },
    );
}
