extern  crate sdl2;


use std::{thread::sleep, time::{Duration, Instant}};

use sdl2::{ event::Event, keyboard::Keycode, pixels::Color, rect::Point, video::Window, EventPump, Sdl, VideoSubsystem, render::Canvas};
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
    let mut next_time = Instant::now();

    const INTERVAL : u64 = 10;

    let change: Duration = std::time::Duration::from_secs(10);
    let mut start : Instant = std::time::Instant::now(); 

    let mut points : Vec<Point> =vec![];
    let mut cur: i32= 0;
    let mut i:f32 = 2.0 * std::f32::consts::PI;
    let mut variation : f32 = 0.01;

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
        
        if std::time::Instant::now() - start >=change{
            if cur != 3{
                cur+=1;
            }
            else {
                cur = 0;
            }
            start = std::time::Instant::now();
        }
        

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();
        canvas = draw_axis(canvas,WIDTH,HEIGHT,2);

        canvas.set_draw_color(Color::RGB(255,255,255));

        points.push(get_point((WIDTH/10).try_into().unwrap(), (HEIGHT/10).try_into().unwrap(), WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap(), cur, i));

        if i <= 0.0{
            variation = 0.01;
        }
        if i >= 2.0 * std::f32::consts::PI{
            i=0.0;
        }
        
        i += variation;

     
        points = move_points(points, (WIDTH/10).try_into().unwrap(),1);
        

        canvas = draw_points(canvas, &points);

        sleep(next_time-Instant::now());
        next_time+= Duration::from_millis(INTERVAL);

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

fn get_point (padding_vertical : i32 , padding_upright : i32 , width : i32,height : i32,cur:i32,i:f32) ->Point{

    let lenght_vertical: f32 = (width - 2*padding_vertical) as f32;
    let vertical_proportion: f32 = lenght_vertical / std::f32::consts::PI/4.0;
    let lenght_upright : f32 = (height - 2 * padding_upright) as f32;
    let upright_proportion : f32 = lenght_upright/ 4.0;
    let mut point : Point = draw_sin( width, height, vertical_proportion, upright_proportion, i);

        match cur {
            0 => point = draw_sin( width, height, vertical_proportion, upright_proportion, i),
            1 => point = draw_cos( width, height, vertical_proportion, upright_proportion, i),
            2 => point = draw_tanges( width, height, vertical_proportion, upright_proportion, i),
            3 => point = draw_cotanges( width, height, vertical_proportion, upright_proportion, i),
            _ => {}
        }
      
    return point;
} 

fn move_points(mut points:Vec<Point>,vertival_padding: i32,tempo:i32) -> Vec<Point>{
    let mut cut: usize  = 0;
    for i in 0..points.len(){
        points[i] = points[i].offset(-tempo,0);
        if points[i].x() <= vertival_padding{
            cut = i;
        }
    }

    points.drain(0..cut);

    return points;
}

fn draw_points(mut canvas:Canvas<Window>,points:& Vec<Point>) -> Canvas<Window>{
    for i in 0..points.len(){
        canvas.draw_point(points[i]).expect("draw point failed");
    }
    return canvas;
}

fn draw_sin(width : i32,height : i32,vertical_proportion : f32,upright_proportion:f32 ,i:f32) -> Point{
    return Point::new(width/2+ (2.0 * std::f32::consts::PI * vertical_proportion) as i32,height/2 + (i.sin()*upright_proportion) as i32 * -1);
}

fn draw_cos(width : i32,height : i32,vertical_proportion : f32,upright_proportion:f32 ,i:f32) -> Point{
    return Point::new(width/2+ (2.0 * std::f32::consts::PI * vertical_proportion) as i32,height/2 + (i.cos()*upright_proportion) as i32 * -1);
}

fn draw_tanges(width : i32,height : i32,vertical_proportion : f32,upright_proportion:f32 ,i:f32) -> Point{
    return Point::new(width/2+ (2.0 * std::f32::consts::PI * vertical_proportion) as i32,height/2 + (i.tan()*upright_proportion) as i32 * -1);
}

fn draw_cotanges(width : i32,height : i32,vertical_proportion : f32,upright_proportion:f32 ,i:f32) -> Point{
    return Point::new(width/2+ (2.0 * std::f32::consts::PI * vertical_proportion) as i32,height/2 + ((i.cos()/i.sin())*upright_proportion) as i32 * -1);
}

