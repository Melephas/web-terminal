use bracket_terminal::prelude::*;

bracket_terminal::add_wasm_support!();

const PADDING: usize = 1;
const WIDTH: usize = 120;
const HEIGHT: usize = 75;

#[derive(Default)]
struct State {
    display_lines: Vec<String>,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        for (y, line) in self.display_lines.iter().enumerate() {
            ctx.print(PADDING, (y + PADDING) as i32, &line);
        }

        match ctx.key {
            None => {}
            Some(key) => {
                self.display_lines.push(format!(
                    "{} pressed, shift: {}, control: {}, alt: {}",
                    key as i32, ctx.shift, ctx.control, ctx.alt
                ));
                while self.display_lines.len() > HEIGHT - (PADDING * 2) {
                    self.display_lines.remove(0);
                }
            }
        }
    }
}

fn main() -> BError {
    let mut context = BTermBuilder::simple(WIDTH, HEIGHT)?
        .with_title("Hello Minimal Bracket World")
        .build()?;

    context.post_screenburn = false;
    context.post_scanlines = false;

    let gs: State = State::default();
    main_loop(context, gs)
}