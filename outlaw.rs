extern crate sdl2;
extern crate lazy_static;
use sdl2::video::*;
use sdl2::render::*;
use std::time::Duration;
use sdl2::event::Event;
use sdl2::pixels::Color;
use sdl2::keyboard::KeyboardState;
use sdl2::keyboard::Scancode;
use lazy_static::lazy_static;
const frametime : u32 = 10;
const steptime : u32 = 20;
const bullettime : u32 = 10;
const player_speed : f64 = 2.0;
const bullet_speed : f64 = 1.5;
//const swaptime : u32 = 500;
const sprnum : usize = 5;
const wall_size : i32 = 16;
const wall_size_float : f64 = 16.0;
const wall_distance_from_top : f64 = 96.0;
const LEFT : usize = 0;
const RIGHT : usize = 1;
const size_of_dot:i32=8;
const size_of_dot_float:f64=8.0;

/*flip the bits of a UByte*/
/*11010001 -> 10001011*/
fn flipbits(b:u8)->u8{
    let mut bcontain=b;
    let mut result:u8=0;
    for i in 0..8{
        result<<=1;
        if (bcontain & 1 ==1){
            result+=1;

        }
        bcontain>>=1;
    }
    return result;
}

/*flip an entire array of ubytes*/
fn flip(ua:  [u8;16])-> [u8;16]{
    let mut result :  [u8;16]=[0;16];	
    let mut cnt=0;
    for b in ua {
        result[cnt]=flipbits(b);
        cnt+=1;
    }
    return result;


}

/*
The graphics are stored in a 1bpp
UByte Array. All the graphics (except the bullets)
are exactly 8 dots wide. so each line is exactly 1 byte.
This is also kinda how it works on an original Atari 2600.
except only 1 line can be in "VRAM" at a time.
 */
static naught: [u8;12]=[
    0b01111110,
    0b11111111,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11111111,
    0b01111110
];
static one: [u8;12]=[
    0b00111100,
    0b00111100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00001100,
    0b00000000
];
static two: [u8;12]=[
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111111,
    0b11111111,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111111,
    0b11111111
];
static three: [u8;12]=[
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111111,
    0b11111111
];
static  four: [u8;12]=[
    0b00011000,
    0b00111000,
    0b01111000,
    0b11011000,
    0b10011000,
    0b11111111,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000
];
static five: [u8;12]=[
    0b11111111,
    0b11111111,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b11111111,
    0b11111111
];
static six: [u8;12]=[
    0b11111111,
    0b11111111,
    0b11000000,
    0b11000000,
    0b11000000,
    0b11111111,
    0b11111111,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11111111,
    0b11111111
];
static seven: [u8;12]=[
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000110,
    0b00001100,
    0b11111111,
    0b11111111,
    0b00110000,
    0b00110000,
    0b01100000,
    0b01100000,
    0b11000000
];
static eight: [u8;12]=[
    0b11111111,
    0b11111111,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11111111,
    0b11111111,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11111111,
    0b11111111
];
static nine: [u8;12]=[
    0b11111111,
    0b11111111,
    0b11000011,
    0b11000011,
    0b11000011,
    0b11111111,
    0b11111111,
    0b00000011,
    0b00000011,
    0b00000011,
    0b00000011,
    0b00000011
];
static ten: [u8;12]=[
    0b11011111,
    0b11011111,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011011,
    0b11011111,
    0b11011111
];
static digits:[[u8;12];11]=[naught,one,two,three,four,five,six,seven,eight,nine,ten];

/*This is a container for the canvas and context, and is in charge for drawing the screen*/
pub struct MyCanvasWindow{
	canvas : WindowCanvas,
	event_pump  : sdl2::EventPump,
	timer_subsystem : sdl2::TimerSubsystem,
	prevframe: u32
}
impl MyCanvasWindow{
    /*this draws the scoreboard digits*/
    fn draw_hits<'a>(&mut self,mut sherrifs:[Sherrif<'a>;2])->[Sherrif<'a>;2]{
        let mut sherrif1=sherrifs[0];
        let mut sherrif2=sherrifs[1];
        /*draw Player 1's score*/
        {
            let hits = sherrif2.hits as usize;
            self.canvas.set_draw_color(sherrif1.color);
			/* index the 1 byte lines*/
            for i in 0..12 {
				/* loop grabs bits starting from the most significant and draws them on screen*/
                let mut b = digits[hits][i];
                for j in 0..8 {
                    if (b >= 0x80) {
                        self.canvas.fill_rect(sdl2::rect::Rect::new(16 + (j as i32) * size_of_dot, (i as i32) * size_of_dot, size_of_dot as u32, size_of_dot as u32));
                    }
                    b = b<<1;
                }
            }
        }
        /*draw player 2's score*/
        {
            let hits = sherrif1.hits as usize;
            self.canvas.set_draw_color(sherrif2.color);
			/* index the 1 byte lines*/
            for i in 0..12 {
				/* loop grabs bits starting from the most significant and draws them on screen*/
                let mut b = digits[hits][i];
                for j in 0..8 {
                    if (b >= 0x80) {
                        self.canvas.fill_rect(sdl2::rect::Rect::new(800 - 64 - 16 + (j as i32) * size_of_dot, (i as i32) * size_of_dot, size_of_dot as u32, size_of_dot as u32));
                    }
                    b = b <<1;
                }
            }
        }
        return [sherrif1,sherrif2];

    }
    /*draws the screen once*/
	pub fn draw<'a>(&mut self, all_sprites: &[&dyn Sprite;sprnum],mut sherrifs:[Sherrif<'a>;2] )->[Sherrif<'a>;2]{
		self.canvas.set_draw_color( Color::RGB(240,240,231));
		self.canvas.clear();
		self.canvas.set_draw_color( Color::RGB(140,120,100));
		let wall_top_rect=sdl2::rect::Rect::new(0,wall_distance_from_top as i32,800,wall_size as u32);
		let wall_bottom_rect=sdl2::rect::Rect::new(0,600-wall_size as i32,800,wall_size as u32);
		self.canvas.fill_rect(wall_top_rect);
		self.canvas.fill_rect(wall_bottom_rect);
        self.canvas.set_draw_color( Color::RGB(0xaa,0xaa,0xbb));
        let scoreboard_rect=sdl2::rect::Rect::new(0,0,800,wall_distance_from_top as u32);   
		self.canvas.fill_rect(scoreboard_rect);
		for sprite in all_sprites{
			sprite.drawOn(self);
		}
		let return_value:[Sherrif<'a>;2]= self.draw_hits(sherrifs);
		self.canvas.present();
		return return_value;
	}
	/*This function manages framerate*/
	pub fn drawOnTime<'a>(&mut self, all_sprites: &[&dyn Sprite;sprnum],mut sherrifs:[Sherrif<'a>;2] )->[Sherrif<'a>;2]{
		if( self.timer_subsystem.ticks() >= self.prevframe+frametime){
			let return_value:[Sherrif<'a>;2]=self.draw(all_sprites,sherrifs);
			self.prevframe=self.timer_subsystem.ticks();
			return return_value;
		}
		return sherrifs;
	}
}

static sherrif_left_image_data: [u8;16]=[
    0b00011000,
    0b00111100,
    0b00011000,
    0b00011000,
    0b00111100,
    0b00111100,
    0b01011010,
    0b10011001,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00101000,
    0b00101000,
    0b00101000,
    0b01000100,
    0b01000100
];


static sherrif_left_walk_image_data: [u8;16]=[
    0b00011000,
    0b00111100,
    0b00011000,
    0b00011000,
    0b00111100,
    0b00111100,
    0b01011010,
    0b10011001,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00101000,
    0b00101000,
    0b00010000,
    0b00101000,
    0b01000100
];
static sherrif_left_shoot_down_image_data:[u8;16] = [
	0b00011000,
	0b00111100,
	0b00011000,
	0b00011000,
	0b00010000,
	0b00111000,
	0b00011110,
	0b00011010,
	0b00011001,
	0b00011000,
	0b00011000,
	0b00011000,
	0b00011000,
	0b00011000,
	0b00000100,
	0b01111110,
];
static sherrif_left_shoot_straight_image_data:[u8;16] = [
    0b00011000,
    0b00111100,
    0b00011000,
    0b00011000,
    0b00010011,
    0b00111110,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00000100,
    0b01111110
];
static sherrif_left_shoot_up_image_data :[u8;16] = [
    0b00011000,
    0b00111100,
    0b00011001,
    0b00011010,
    0b00010110,
    0b00111000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00011000,
    0b00000100,
    0b01111110
];
static sherrif_left_hit  :[u8;16]=[
    0b00000000,
    0b01100000,
    0b11110000,
    0b11110000,
    0b01100000,
    0b01100000,
    0b01100000,
    0b01100000,
    0b11100000,
    0b11100000,
    0b11100000,
    0b11100000,
    0b11100000,
    0b10000000,
    0b10000000,
    0b11111111
];
static obstacle_image_data  :[u8;16] =[
    0b00001001,
    0b00001010,
    0b10001010,
    0b01001100,
    0b00101100,
    0b00011000,
    0b00001001,
    0b00001110,
    0b00001000,
    0b00001000,
    0b10001000,
    0b01001000,
    0b00101000,
    0b00011000,
    0b00011000,
    0b00011000
];

/*Image data for both sides*/
lazy_static!{
	static ref sherrif_image_data:[[u8;16];2]=[sherrif_left_image_data,flip(sherrif_left_image_data)];
	static ref sherrif_walk_image_data:[[u8;16];2]=[sherrif_left_walk_image_data,flip(sherrif_left_walk_image_data)];
	static ref sherrif_shoot_straight_image_data:[[u8;16];2]=[sherrif_left_shoot_straight_image_data,flip(sherrif_left_shoot_straight_image_data)];
	static ref sherrif_shoot_down_image_data:[[u8;16];2]=[sherrif_left_shoot_down_image_data,flip(sherrif_left_shoot_down_image_data)];
	static ref sherrif_shoot_up_image_data:[[u8;16];2]=[sherrif_left_shoot_up_image_data,flip(sherrif_left_shoot_up_image_data)];
	static ref sherrif_hit:[[u8;16];2]=[sherrif_left_hit,flip(sherrif_left_hit)];

}

pub struct Sherrif<'a>{
	color:sdl2::pixels::Color,
	x : f64,
	y : f64,
	w : f64,
	h : f64,
	curImageState : &'a [u8;16],
	prevstep : u32,
	prevswap : u32,
	iswalking : bool,
	swaptime:u32,
	isaiming:bool,
	vertical_movement:i32,
	side: usize,
	hits: i32,
	isshooting:bool,
	ishit:bool
}
trait Sprite{
	fn drawOn(&self, window:&mut MyCanvasWindow);
}
impl Sprite for Sherrif<'_>{
	fn drawOn(&self,window:&mut MyCanvasWindow){
		window.canvas.set_draw_color(self.color);
		for i in 0..self.h as i64{
		    let mut b:u8 = self.curImageState[i as usize];
		    
		    for j in 0..self.w as i64{
		        if (b>=0x80){
		        	let r=sdl2::rect::Rect::new(
		        		(self.x + (j as f64 * size_of_dot_float)) as i32, 
		        		(self.y + (i as f64 * size_of_dot_float)) as i32,
		        		size_of_dot as u32,
		        		size_of_dot as u32
		        	);
		            window.canvas.fill_rect(r);
	
	        }
		        b=(b<<1) as u8;

		    }
		}
	}

}
impl<'a> Sherrif<'_>{
	/*This moves the sherrif*/
	fn Move(&mut self,deltax:f64, deltay:f64,ticks:u32){
        if (self.isshooting){return;}
		/*manage redundant data*/
		if (deltay>0.0){
			self.vertical_movement=1;
		} else if (deltay<0.0){
			self.vertical_movement=-1;
			
		} else {
			self.vertical_movement=0;
			
		}
		if (self.isaiming){return;}
		/*Actually move the sherrif*/
		if ((deltax!=0.0 || deltay!=0.0)){
			
			self.iswalking=true;
            self.x+=deltax*player_speed;
            self.y+=deltay*player_speed;
		} else {
			self.iswalking=false;
		}
		

        /*check to see if the sherrif is trying to move off screen and prevent it*/
		if (self.y<wall_size as f64+wall_distance_from_top){
			self.y=wall_size as f64+wall_distance_from_top;
		}
		if (self.y>600.0-wall_size as f64-size_of_dot_float*self.h){
			self.y=600.0-wall_size as f64-self.trueheight();
		}
		if (self.x<(self.side as f64)*400.0){
			self.x=(self.side as f64)*400.0;
		}
		if (self.x>(((self.side+1) as f64)*400.0)-self.truewidth()){
			self.x=(((self.side+1) as f64)*400.0)-self.truewidth();
		}
		/*check to see if the sherrif is trying to move into the obstacle.*/
		if ((self.x + self.truewidth() > obstacle.x) && (self.x + self.truewidth() < obstacle.x + size_of_dot_float) && (self.y < obstacle.y + obstacle.trueheight()) && (self.y + self.trueheight() > obstacle.y)) {
			self.x = obstacle.x - self.truewidth();
		   	}
		   	if ((self.x > obstacle.x + obstacle.truewidth() - size_of_dot_float) && (self.x < obstacle.x + obstacle.truewidth()) && (self.y < obstacle.y + obstacle.trueheight()) && (self.y + self.trueheight() > obstacle.y)) {
			self.x = obstacle.x + obstacle.truewidth();
		   	}
		if ((self.y + self.trueheight() > obstacle.y) && (self.y + self.trueheight() < obstacle.y + size_of_dot_float) && (self.x < obstacle.x + obstacle.truewidth()) && (self.x + self.truewidth() > obstacle.x)) {
		        self.y = obstacle.y - self.trueheight();
		}
		if ((self.y > obstacle.y + obstacle.trueheight() - size_of_dot_float) && (self.y < obstacle.y + obstacle.trueheight()) &&
        (self.x < obstacle.x + obstacle.truewidth()) && (self.x + self.truewidth() > obstacle.x)) {
			self.y = obstacle.y + obstacle.trueheight();
        }
        self.prevstep=ticks;
		
	}
	fn truewidth(&self)->f64{return (self.w as f64)*(size_of_dot as f64);}
	fn trueheight(&self)->f64{return (self.h as f64)*(size_of_dot as f64);}
    /*This swaps the neutral and step positions if the sherrif is moving*/
	fn step_cycle_handler(&mut self,ticks:u32){
        if (self.isshooting) { return;}
		if (self.isaiming){
			if (self.vertical_movement>0){
				self.curImageState=&(sherrif_shoot_down_image_data[self.side]);

			} else if (self.vertical_movement<0){
				self.curImageState=&(sherrif_shoot_up_image_data[self.side]);
			
			} else {
				self.curImageState=&(sherrif_shoot_straight_image_data[self.side]);
			}
		} else {
			if (ticks>=self.prevswap+self.swaptime){
				self.prevswap=ticks;
				if (self.iswalking ){
					if self.curImageState==&(sherrif_image_data[self.side]){
					
						self.curImageState=&(sherrif_walk_image_data[self.side]);
					} else {
						self.curImageState=&(sherrif_image_data[self.side]);
					}
				} else {
					self.curImageState=&(sherrif_image_data[self.side]);
				}
			}
		}
	}
    /*This is called when a bullet hits the guy*/
    fn hit(&mut self) {
        self.hits += 1;
        self.curImageState = &sherrif_hit[self.side];
        self.ishit=true;
    }
	
}
impl Copy for Sherrif<'_>{}
impl<'a> Clone for Sherrif<'a>{
    fn clone(&self) -> Sherrif<'a>{
        return *self;
    }
}
/*This is a struct for the bush in the middle of the field*/
pub struct Obstacle{
	color : Color,
	x : f64,
	y : f64,
	w :u32,
	h :u32
}

impl Obstacle{
	fn truewidth(&self)->f64{return (self.w as f64)*(size_of_dot as f64);}
	fn trueheight(&self)->f64{return (self.h as f64)*(size_of_dot as f64);}
}

impl Sprite for Obstacle{
	fn drawOn(&self,window:&mut MyCanvasWindow){
		window.canvas.set_draw_color(self.color);
		for i in 0..self.h as i64{
		    let mut b:u8 = obstacle_image_data[i as usize];
		    
		    for j in 0..self.w as i64{
		        if (b>=0x80){
		        	let r=sdl2::rect::Rect::new(
		        		(self.x + ((j as f64) * size_of_dot_float)) as i32, 
		        		(self.y + ((i as f64) * size_of_dot_float)) as i32,
		        		size_of_dot as u32,
		        		size_of_dot as u32
		        	);
		            window.canvas.fill_rect(r);
	
	        }
		        b=(b<<1) as u8;

		    }
		}
	}
}
/*This function creates a Bullet instance*/
pub fn initialize_bullet(c:Color,side:usize, shoot_time:u32)->Bullet{
	let bullet=Bullet{
		color:c,
		x:-200.0,
		y:-200.0,
		deltax:0.0,
		deltay:0.0,
		last_move_time: shoot_time,
		side : side
	};
	return bullet;
}
/*This function creates an Obstacle instance*/
pub fn initialize_obstacle()->Obstacle{
	let c=Color::RGB(0x88,0x99,0x33);
	let obstacl=Obstacle{
		color:c,
		x:400.0-32.0,
		y:300.0+(wall_distance_from_top/2.0)-64.0,
		w:8,
		h:16

	};
	return obstacl;
}
lazy_static!{
	pub static ref obstacle:Obstacle = initialize_obstacle();
}
/*This struct is in charge of the bullet that is shot from the gun of the players*/
pub struct Bullet{
	x:f64,
	y:f64,
	color:Color,
	deltax:f64,
	deltay:f64,
	last_move_time:u32,
	side : usize
}

impl Bullet {
	fn isInScreen(&self)->bool{
        	if ((self.x>800.0+size_of_dot_float) || (self.y>600.0+size_of_dot_float) || (self.x< -size_of_dot_float) || (self.y< -size_of_dot_float)) {
	        	

        		return false;
        	}
	
        return true;
    }
   /*move the bullet according to its momentum, returns if a hit occured*/
    fn move_bullet<'a>(&mut self, mut sherrifs:[Sherrif<'a>;2])->[Sherrif<'a>;2]{
        //if (!world_active) {return}
        self.x+=self.deltax*bullet_speed;
		self.y+=self.deltay*bullet_speed;
        if ((self.y<=wall_size_float+wall_distance_from_top) || (self.y>=600.0-wall_size_float)){
            self.deltay=-self.deltay;
        }
        /* too lazy to write a better collision handler */
        let mut hit=false;
        /*check to see if the bullet hit the obstacle*/
        if ((self.x + size_of_dot_float > obstacle.x) && (self.x + size_of_dot_float< obstacle.x + size_of_dot_float) && (self.y < obstacle.y + obstacle.trueheight()) && (self.y + size_of_dot_float > obstacle.y)) {
            hit=true;
        }
        if ((self.x > obstacle.x + obstacle.truewidth() - size_of_dot_float) && (self.x < obstacle.x + obstacle.truewidth()) && (self.y < obstacle.y + obstacle.trueheight()) && (self.y + size_of_dot_float > obstacle.y)) {
            hit=true;
        }
        if ((self.y + size_of_dot_float > obstacle.y) && (self.y + size_of_dot_float < obstacle.y + size_of_dot_float) && (self.x < obstacle.x + obstacle.truewidth()) && (self.x + size_of_dot_float > obstacle.x)) {
            hit=true;
        }
        if ((self.y > obstacle.y + obstacle.trueheight() - size_of_dot_float) && (self.y < obstacle.y + obstacle.trueheight()) && (self.x < obstacle.x + obstacle.truewidth()) && (self.x + size_of_dot_float > obstacle.x)) {
            hit=true;
        }
		/*check to see if the bullet hit the enemy player*/
        let mut opposing_sherrif : Sherrif = sherrifs[1-self.side];
        if ((self.x + size_of_dot_float > opposing_sherrif.x) && (self.x + size_of_dot_float < opposing_sherrif.x + size_of_dot_float) && (self.y < opposing_sherrif.y + opposing_sherrif.trueheight()) && (self.y + size_of_dot_float > opposing_sherrif.y)) {
            hit=true;
            opposing_sherrif.hit();

        }
        if ((self.x > opposing_sherrif.x + opposing_sherrif.truewidth() - size_of_dot_float) && (self.x < opposing_sherrif.x + opposing_sherrif.truewidth()) && (self.y < opposing_sherrif.y + opposing_sherrif.trueheight()) && (self.y + size_of_dot_float > opposing_sherrif.y)) {
            hit=true;
            opposing_sherrif.hit();
        }
        if ((self.y + size_of_dot_float > opposing_sherrif.y) && (self.y + size_of_dot_float< opposing_sherrif.y + size_of_dot_float) && (self.x < opposing_sherrif.x + opposing_sherrif.truewidth()) && (self.x + size_of_dot_float> opposing_sherrif.x)) {
            hit=true;
            opposing_sherrif.hit();
        }
        if ((self.y > opposing_sherrif.y + opposing_sherrif.trueheight() - size_of_dot_float) && (self.y < opposing_sherrif.y + opposing_sherrif.trueheight()) && (self.x < opposing_sherrif.x + opposing_sherrif.truewidth()) && (self.x + size_of_dot_float > opposing_sherrif.x)) {
            hit=true;
            opposing_sherrif.hit();
        }
        
        sherrifs[1-self.side]=opposing_sherrif;
        /*move the bullet out of bounds if hit something*/
        if (hit){
            /*move the bullet out of bounds*/
            self.x=-200.0;
            self.y=-200.0;
            self.deltax=0.0;
            self.deltay=0.0;
            
            /*change sherrif to move state*/
            sherrifs[self.side].isshooting=false;


        }
        return sherrifs;
    }
    /*sets the position && momentum of the bullet*/
    fn shoot(&mut self,x:f64,y:f64,deltax:f64,deltay:f64, shoot_time:u32){
        
        self.x=x;
        self.y=y;
        self.deltax=deltax;
        self.deltay=deltay;
        self.last_move_time=shoot_time;
    }
}
impl Sprite for Bullet{
	fn drawOn(&self, window:&mut MyCanvasWindow){

		if (self.isInScreen()){
				window.canvas.set_draw_color(self.color);
				let r=sdl2::rect::Rect::new(
		        		self.x as i32 , 
		        		self.y as i32,
		        		size_of_dot as u32,
		        		size_of_dot as u32
		        	);
		            window.canvas.fill_rect(r);
		}
	}

}
/*This function creates a Player instance*/
pub fn initialize_sherrif(c:Color,posx:f64,posy:f64,creation_time:u32,side:usize)->Sherrif<'static>{
	let mut swaptime;
	if (side==LEFT){
		swaptime=400;
	} else {
		swaptime=380;
	}
	let sherrif = Sherrif{
		color:c,
		x:posx,
		y:posy,
		w:8.0,
		h:16.0,
		curImageState:&(sherrif_image_data[side]),
		prevstep: creation_time,
		prevswap:creation_time,
		iswalking:false,
		side:side,
		isaiming:false,
		swaptime: swaptime,
		vertical_movement : 0,
		hits: 0,
        isshooting:false,
        ishit:false
	};
	return sherrif;
}
/*This function initializes SDL and creates the window*/
pub fn initialize_window(context:sdl2::Sdl)->MyCanvasWindow{
	
	
	let video_subsystem=context.video().unwrap();
	let window_builder= video_subsystem.window("Outlaw", 800, 600);
	let sdl_window= window_builder.build().unwrap();
	let canvas_builder=sdl_window.into_canvas();
	let canvas=canvas_builder.build().unwrap();
	let event_pump = context.event_pump().unwrap();
	
	let timer_subsystem=context.timer().unwrap();
	let zeroth_frame=timer_subsystem.ticks();
	let mut window= MyCanvasWindow{
		canvas: canvas,
		event_pump: event_pump,
		timer_subsystem:timer_subsystem,
		prevframe : zeroth_frame
	};
	return window;
}

pub fn main(){
	let context:sdl2::Sdl=sdl2::init().unwrap();
	/*Initialize all the sorta static mutables*/
	let mut mywindow : MyCanvasWindow=initialize_window(context);
	let mut ticks=mywindow.timer_subsystem.ticks();
	let mut sherrif1 = initialize_sherrif(Color::RGB(102,51,153),200.0,200.0,ticks,LEFT);
	let mut sherrif2 = initialize_sherrif(Color::RGB(51,153,102),600.0,400.0,ticks+200,RIGHT);
    let mut bullet1 = initialize_bullet(Color::RGB(102,51,153),LEFT,ticks);
    let mut bullet2 = initialize_bullet(Color::RGB(51,153,102),RIGHT,ticks);
    let bullets : [&Bullet;2] = [&bullet1,&bullet2];
    let mut world_last_deactivation=ticks as i32-1000;

	'mainloop: loop{
	    for event in mywindow.event_pump.poll_iter() {
			/*check for quit*/
		    match event {
		        Event::Quit {..} => {
		            break  'mainloop
		        },
		        _ => {}
		    }
		}
		let keyboard_state=mywindow.event_pump.keyboard_state();
		let mut ticks=mywindow.timer_subsystem.ticks();
        let mut sherrifs : [Sherrif;2] = [sherrif1,sherrif2];
        if (ticks as i32>=world_last_deactivation+1000){
			/*check win condition*/
            if (sherrif1.hits>=10 || sherrif2.hits>=10){
                    break 'mainloop;
            }
			/*move the sherrifs if one of the movement keys is pressed*/
            if (ticks>=sherrif1.prevstep+steptime){
                sherrif1.iswalking=false;sherrif1.Move(0.0,0.0,ticks);
                if (keyboard_state.is_scancode_pressed(Scancode::W)){
                    sherrif1.Move(0.0,-1.0,ticks);
                }
                if (keyboard_state.is_scancode_pressed(Scancode::S)){
                    sherrif1.Move(0.0,1.0,ticks);
                }
                if (keyboard_state.is_scancode_pressed(Scancode::A)){
                        sherrif1.Move(-1.0,0.0,ticks);
                }
                if (keyboard_state.is_scancode_pressed(Scancode::D)){
                    sherrif1.Move(1.0,0.0,ticks);
                }
                
            }
            if (ticks>=sherrif2.prevstep+steptime) { 
                sherrif2.iswalking=false;sherrif2.Move(0.0,0.0,ticks);
                if (keyboard_state.is_scancode_pressed(Scancode::I)){
                    sherrif2.Move(0.0,-1.0,ticks);

                }
                if (keyboard_state.is_scancode_pressed(Scancode::K)){
                    sherrif2.Move(0.0,1.0,ticks); 
                }
                if (keyboard_state.is_scancode_pressed(Scancode::J)){
                    sherrif2.Move(-1.0,0.0,ticks);
                }
                if (keyboard_state.is_scancode_pressed(Scancode::L)){
                    sherrif2.Move(1.0,0.0,ticks);
                }
                
                
            }
			/*start aiming if one of the shoot buttons is pressed*/
			/*too lazy to fix the indentation.*/
                if (keyboard_state.is_scancode_pressed(Scancode::Q)){
                    sherrif1.isaiming=true;
                } else {
					/*If released the bullet needs to be shot to the correct direction*/
                    if (sherrif1.isaiming && !sherrif1.isshooting){
                        bullet1.shoot(
                            sherrif1.x+sherrif1.truewidth(),
                            sherrif1.y + (sherrif1.trueheight() / 2.0)-((-sherrif2.vertical_movement+1)*3*size_of_dot) as f64,
                            3.0,
                            (sherrif1.vertical_movement*3) as f64,
                            ticks
                            
                        );
                        sherrif1.isshooting=true;
                        sherrif1.isaiming=false;
                    }
                }
            
                if (keyboard_state.is_scancode_pressed(Scancode::U)){
                    sherrif2.isaiming=true;
                } else {
                    if (sherrif2.isaiming && !sherrif2.isshooting){
                        bullet2.shoot(
                            sherrif2.x,
                            sherrif2.y + (sherrif2.trueheight() / 2.0)-((-sherrif2.vertical_movement+1)*3*size_of_dot) as f64,
                            -3.0,
                            (sherrif2.vertical_movement*3) as f64,
                            ticks
                            
                        );
                        sherrif2.isshooting=true;
                        sherrif2.isaiming=false;
                        
                    }
                }
                

            sherrif1.ishit=false;
            sherrif2.ishit=false;
            
			/*move the bullets that are already on screen*/
            if (ticks>=bullet1.last_move_time+bullettime){
                if (bullet1.isInScreen()){
                    sherrifs=bullet1.move_bullet(sherrifs);
                    bullet1.last_move_time=ticks;
                    sherrif1=sherrifs[0];
                    sherrif2=sherrifs[1];
                    
                    
                    sherrif1.isaiming=false;
                } else {
                    if (sherrif1.isshooting){
                        sherrif1.isshooting=false;
                    }
                } 
            }
            if (ticks>=bullet2.last_move_time+bullettime){
                if (bullet2.isInScreen()){
					/*
					The return value gets the modified sherrifs instead
					of a simple "is hit" because I
					need the sherrifs to be somewhat mutable
					the sherrifs are sent by value to prevent
					aliasing. 
					*/
                    sherrifs=bullet2.move_bullet(sherrifs);
                    bullet2.last_move_time=ticks;
                    sherrif1=sherrifs[0];
                    sherrif2=sherrifs[1];
                } else {
                    if (sherrif2.isshooting){
                        sherrif2.isshooting=false;
                    }
                }
            }
            sherrif1.step_cycle_handler(ticks);
            sherrif2.step_cycle_handler(ticks);
			/*pause if one of the sherrifs is hit this frame*/
            if (sherrif1.ishit || sherrif2.ishit){
                world_last_deactivation=ticks as i32;
            }

        }
        
		let all_sprites : [&dyn Sprite;sprnum] = [&sherrif1,&sherrif2,&*obstacle, &bullet1, &bullet2];
		mywindow.drawOnTime(&all_sprites,sherrifs);
		/*this is to prevent CPU power draw*/
		std::thread::sleep(Duration::new(0,100));
	}
	/*Thanks for looking at the source code*/
	println!("Thanks for playing!");
		
	
}
