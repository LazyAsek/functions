

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
        canvas = draw_axis(canvas,WIDTH,HEIGHT,2);

        canvas.set_draw_color(Color::RGB(255,255,255));
        
        let mut i : f32 = -2.0 * std::f32::consts::PI;
        while i <= 2.0 * std::f32::consts::PI{
            canvas.draw_point(Point::new((i*100.0) as i32,(i.sin()*100.0) as i32)).expect("draw point failed");
            i+=0.01;
        } 
        


        canvas.present();
    }

}

fn draw_axis (mut canvas:Canvas<Window>,width : u32, height : u32 ,line_width: u32) -> Canvas<Window> {
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

    canvas = draw_vertical_lines(canvas,padding_vertical.try_into().unwrap(),(width-padding_vertical).try_into().unwrap(), (line_width/2).try_into().unwrap(), height.try_into().unwrap());
    canvas = draw_upright_lines(canvas, padding_upright.try_into().unwrap(), (height-padding_upright).try_into().unwrap(), (line_width/2).try_into().unwrap(), width.try_into().unwrap());

    return canvas;
}

fn draw_vertical_lines(mut canvas : Canvas<Window>,line_start : i32,line_end : i32,size: i32 , height : i32) -> Canvas<Window>{
    let mid = (line_start + line_end) /2;
    let space : i32 = (mid - line_start)/4;
    let mut cur :i32 ;
    let padding:i32 = (height/100).try_into().unwrap();


    for i in 0..9{
        let temp : i32 = i.try_into().unwrap();
        cur = space * temp;
        for _ in 0..size{
            canvas.draw_line(Point::new(line_start + cur, (height /2)-padding), Point::new(line_start + cur, (height /2)+padding)).expect("draw line failed");
            cur += 1;
        }
    }

    return canvas;
    }

fn draw_upright_lines(mut canvas : Canvas<Window>,line_start : i32,line_end : i32,size: i32,width : i32) -> Canvas<Window>{
    let mid = (line_start + line_end) /2;
    let space : i32 = (mid - line_start)/2;
    let mut cur :i32 ;
    let padding : i32 = width/100;

    for i in 1..4{
        cur = space * i;
        for _ in 0..size{
            canvas.draw_line(Point::new(width/2-padding, line_start+cur), Point::new(width/2+padding,line_start+cur)).expect("draw line upright failed");
            cur+=1;
            }
        }
    return canvas
}

