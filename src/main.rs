use sfml::{
//  graphics::Texture,
  graphics::{RenderWindow,RenderTarget,Color,Text,Font,Texture, Sprite, Rect,Transformable},
  window::{ContextSettings,Event,Style,joystick}
};

fn main() {
  let mode:(u32,u32) = (800,600);
  let mut win = RenderWindow::new(
    mode,
    "Drift Minigame",
    Style::CLOSE,
    &ContextSettings::default()
  );
  let font = unsafe { Font::from_memory(include_bytes!("fonts/BIZ-UDGothicR.ttc"))}.unwrap();
  let mut background = Texture::new().unwrap();
  match background.load_from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/img/area.png"),Rect::new(0,0,256,256)){
    Result::Err(t) => {
      println!("ERROR: area.png was not able to be loaded.\n\t{}",t);
      return
    },
    Result::Ok(_)=>{}
  };
  let mut thumb_stick = Texture::new().unwrap();
  match thumb_stick.load_from_file(concat!(env!("CARGO_MANIFEST_DIR"), "/src/img/thumbstick.png"),Rect::new(0,0,64,64)){
    Result::Err(t) => {
      println!("ERROR: thumbstick.png was not able to be loaded.\n\t{}",t);
      return
    },
    Result::Ok(_)=>{}
  }
  let mut sp_background = Sprite::new();
  let mut sp_thumb_stick = Sprite::new();
  sp_background.set_texture(&background,false);
  sp_thumb_stick.set_texture(&thumb_stick,false);
  sp_background.scale((0.5,0.5));
  sp_thumb_stick.set_position((32.0,32.0));


  let mut points = 0;
  let mut frames = 0;
  while win.is_open(){
    while let Some(ev) = win.poll_event(){
      match ev{
        Event::Closed => win.close(),
        _=>{}
      }
    }
    win.clear(Color::rgb(0,50,80));
    let fontsize = 16;
    let mut message = "Score: ".to_owned();
    message.push_str(&points.to_string());
    let mut txt = Text::new(&message, &font, fontsize);
    txt.set_position((0.0,130.0));
    frames += 1;
    if frames>=1000 {
      points += 1;
      frames = 0;
    }
    if ! joystick::is_connected(0) {
      let mut txt2 = Text::new("PLEASE CONNECT A CONTROLLER!",&font, fontsize*2);
      txt2.set_position((130.0,0.0));
      txt2.set_fill_color(Color::RED);
      win.draw(&txt2);
    } else {
      let x = joystick::axis_position(0, joystick::Axis::X)*0.5;// constant to scale inputs to image size
      let y = joystick::axis_position(0, joystick::Axis::Y)*0.5;
      sp_thumb_stick.set_position((32.0+x,32.0+y));

      for i in 0..joystick::button_count(0){
        if joystick::is_button_pressed(0, i){
          println!("button {} pressed",i);
        }
      }
    }
    win.draw(&txt);
    win.draw(&sp_background);
    win.draw(&sp_thumb_stick);
    win.display();
  }
}
