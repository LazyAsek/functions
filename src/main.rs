

use sdl2::{self, event::Event, keyboard::Keycode, pixels::Color, rect::Point, render::Canvas, video::Window, EventPump, Sdl, VideoSubsystem};
fn main() {

    let sld_context : Sdl = sdl2::init().unwrap();
    let video_subsystem : VideoSubsystem = sld_context.video().unwrap();

    const WIDTH: u32 = 1080;
    const HEIGHT : u32 = 720;

    let window : Window = video_subsystem.window("functions", WIDTH,HEIGHT)
    .position_centered()
    .borderless()
    .build()
    .unwrap();

    let mut canvas : Canvas<Window> = window.into_canvas().build().unwrap();

    let mut event_pump : EventPump = sld_context.event_pump().unwrap();

    'running : loop {
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit { .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>{
                    break 'running;
                },
    
                _ => {
    
                }
            }
        }

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas = draw_axis(canvas,&WIDTH,&HEIGHT,2);


        canvas.present();
    }

}

fn draw_axis (mut canvas:Canvas<Window>,width : &u32, height : &u32 ,line_width: u32) -> Canvas<Window> {
    canvas.set_draw_color(Color::RGB(40,40,40));
    let padding_vertical: u32 = width/10;
    let padding_upright : u32 = height/10;

    let mut vertical :[u32;4] = [padding_vertical,height/2,width-padding_vertical,height/2];
    let mut upright : [u32;4] = [width/2,padding_upright,width/2,height-padding_vertical];

    vertical[1]-= line_width/2;
    vertical[3]-= line_width/2;
    upright[0]-= line_width/2;
    upright[2]-= line_width/2;

    for _ in 0..line_width{
        vertical[1] += 1;
        vertical[3] += 1;
        upright[0] +=1;
        upright[2] += 1;
        canvas.draw_line(Point::new(vertical[0].try_into().unwrap(), vertical[1].try_into().unwrap()),Point::new(vertical[2].try_into().unwrap(), vertical[3].try_into().unwrap())).expect("draw line failed");
        canvas.draw_line(Point::new(upright[0].try_into().unwrap(), upright[1].try_into().unwrap()),Point::new(upright[2].try_into().unwrap(), upright[3].try_into().unwrap())).expect("draw line failed");
    }


    return canvas;
}
