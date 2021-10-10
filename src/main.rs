#![allow(non_snake_case)]

extern crate piston_window;
extern crate time;

// use piston_window::*;
// use time::*;

// fn main()
// {
//      let mut window: PistonWindow = WindowSettings::new("Hello Piston!", [640, 480])
//         .exit_on_esc(true).build().unwrap();

//     let mut deltaTime : f32 = 0.0;
//     let mut lastFrameEndTime : f64 = precise_time_s();

//     let mut testSize : f32 = 100.0;

//     while let Some(event) = window.next()
//     {
//         testSize += 100.0 * deltaTime;

//         window.draw_2d(&event, |context, graphics|
//         {
//             clear([1.0; 4], graphics);
//             rectangle([1.0, 0.0, 0.0, 1.0], // red
//                       [0.0, 0.0, 100.0, testSize as f64],
//                       context.transform,
//                       graphics);
//         });

//         let frameEndTime : f64 = precise_time_s();
//         deltaTime = (frameEndTime - lastFrameEndTime) as f32;
//         lastFrameEndTime = frameEndTime;
//     }
// }

extern crate amethyst;
use amethyst::prelude::*;
use amethyst::renderer::{DisplayConfig, DrawSprite, Pipeline,
                       RenderBundle, Stage};

fn main() -> amethyst::Result<()>
{
let path = "./resources/display_config.ron";
let config = DisplayConfig::load(&path);

let pipe = Pipeline::build().with_stage(
    Stage::with_backbuffer()
        .clear_target([0.1, 0.1, 0.3, 1.0], 1.0)
        .with_pass(DrawSprite::new()),);

struct Pong;
impl<'a, 'b> SimpleState<'a,'b> for Pong { }

let game_data = GameDataBuilder::default()
    .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?;
let mut game = Application::new("./", Pong, game_data)?;

game.run();

Ok(())
}