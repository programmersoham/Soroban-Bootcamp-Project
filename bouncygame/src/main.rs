use std::fmt;
use std::fmt::{Display, Formatter,Error };
enum VertDir{
    Up,
    Down
}
enum HorizDir{
    Left,
    Right
}

struct Ball{
    x: i32,
    y: i32,
    vert_dir: VertDir,
    horiz_dir: HorizDir
}
struct  Frame{
    width: i32,
    height: i32
}
struct Game{
    frame: Frame,
    ball: Ball
}

impl Game {
    fn new()-> Game{
        Game{
            frame: Frame{width: 63, height: 31},
            ball: Ball{x: 44, y: 21, vert_dir: VertDir::Down, horiz_dir: HorizDir::Right}
        }
    }

    fn step (&mut self){

        self.ball.bounce(&self.frame);
        self.ball.mv();

        }
    }


impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        // if self.y == 0{
        //     self.vert_dir = VertDir::Down;
        // }
        // if self.y == frame.height - 1{
        //     self.vert_dir = VertDir::Up;
        // }
        if self.x <= 0 {
            self.horiz_dir = HorizDir::Right;
        } else if frame.width <= self.x {
            self.horiz_dir = HorizDir::Left;
        } else if self.y <= 0 {
            self.vert_dir = VertDir::Down;
        } else if frame.height <= self.y {
            self.vert_dir = VertDir::Up;
        } else {}
    }


    fn mv(&mut self) {
        if self.vert_dir == VertDir::Down {
            self.y += 1;
        } else {
            self.y -= 1;
        }
        if self.horiz_dir == HorizDir::Right {
            self.x += 1;
        } else {
            self.x -= 1;
        }
    }
    fn mv(&mut self) {
        match self.vert_dir {
            VertDir::Down => self.y += 1,
            VertDir::Up => self.y -= 1
        }
        match self.horiz_dir {
            HorizDir::Right => self.x += 1,
            HorizDir::Left => self.x -= 1
        }
    }
}
impl Display for Game{
    fn fmt: &mut ,
    write! (fmt
.64{ write!
in @.
for
in
for y:i32
:i32 in
for x
x as 132 self.ball.y y as i32{
if
self. ball. x
write! (fmtum'tl±);
@ {write!
if
else if x ! = y 31 {write! (fmt
else { write! (fmt
write! fmt
"\n");
write! (fmt ,
"\n")
                            }
fn main() {
    println!("Hello, world!");
                                = Game: : new();
let mut new_qame : Game
= std: : time: : Duration: :from_mittis( millis: 33) ;
let sleep_time : Duration
loop{
println! ( ,
new_qame) ;
new _ game. step();
std: : thread: : sleep (sleep _ time) ;
println! ( "{Y {b" ,
new_game . ball. x,
new_qame . ball. y) ;
}
}