use sycamore::motion::ScopeMotionExt;
use sycamore::prelude::*;

const STYLE: &str = r#"
html, body { margin: 0; display: flex; }
svg { width: 100vw; height: 100vh; }
body > div { position: absolute; top: 10px; left: 10px; }
"#;

const DISTANCE: f32 = 20.0;
const RADIUS: f32 = 1.0;
const ROTATION_SPEED: f32 = 0.001;

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    use std::f32::consts::PI;

    let rotation = ctx.create_signal(0f32);
    let (_running, start, _stop) = ctx.create_raf(|| {
        rotation.set((*rotation.get() + ROTATION_SPEED) % 1.0);
    });
    start();

    let num_points = ctx.create_signal(5usize);
    let increment = |_| num_points.set(*num_points.get() + 1);
    let decrement = |_| num_points.set((*num_points.get() - 1).max(2));

    let points = ctx.create_memo(|| {
        let num_points = *num_points.get();
        let mut points = Vec::with_capacity(num_points);
        let angle = 2. * PI / num_points as f32;
        for i in 0..num_points {
            let angle = i as f32 * angle;
            let x = DISTANCE * angle.cos();
            let y = DISTANCE * angle.sin();
            points.push((x, y));
        }
        points
    });

    let line = ctx.create_memo(|| {
        use std::fmt::Write;

        let points = points.get();
        let len = points.len();
        let hops = match len {
            4 => 1,
            6 => 2,
            _ => (len / 3) + 1,
        };
        let mut line = String::new();

        let mut start_idx = 0;
        let mut joined = 0;

        while joined < len {
            let start = points[start_idx];
            let mut curr = start_idx + hops;
            write!(line, "M {},{} L ", start.0, start.1).unwrap();
            while curr != start_idx {
                let (x, y) = points[curr];
                write!(line, "{},{} ", x, y).unwrap();
                curr = (curr + hops) % len;
                joined += 1;
            }
            write!(line, "{},{} ", start.0, start.1).unwrap();
            joined += 1;

            start_idx += 1;
        }
        line
    });

    view! { ctx,
        style { (STYLE) }

        div {
            p { "Points: " (num_points.get()) }
            button(on:click=increment) { "+" }
            button(on:click=decrement) { "-" }
        }

        svg(viewBox="-50 -50 100 100") {
            g(style=(format!("transform: rotate({rotation}turn)"))) {
                path(d=(line), fill="none", stroke="red", stroke-width="0.25") {}
                Indexed {
                    iterable: points,
                    view: move |ctx, (x, y)| view! { ctx,
                        circle(cx=(x), cy=(y), r=(RADIUS), fill="red") {}
                    },
                }
            }
        }
    }
}

fn main() {
    sycamore::render(|ctx| {
        view! { ctx, App() }
    });
}
