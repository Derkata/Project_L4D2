use ggez::{Context, GameResult};
use crate::data::{Weapon,WeaponType};
use ggez::graphics;
use ggez::mint::{Vector2, Point2};
use ggez::input::mouse::position;
use crate::assets::{Assets};
use libm::cos;
use libm::sin;
use libm::atan2;
use ggez::graphics::Rect;
use ggez::graphics::Mesh;
use ggez::graphics::DrawMode;
use crate::data::Structure;
#[derive(Debug)]
pub enum PlayerState {
    Normal,
    Shooting,
}

#[derive(Debug)]
pub struct Player {
    pub state: PlayerState,
    pub pos: Point2<f32>,
    pub time_until_next_shot: f32,
    pub angle: f32,
    pub x_vel: f32,
    pub y_vel: f32,
    pub dis_mov: Point2<f32>,
    pub weapon: Weapon,
    pub f_pos: Point2<f32>,
    pub square: Rect,

    
}

impl Player {
    pub const SHOT_TIMEOUT: f32 = 0.5;
    pub fn new(pos: Point2<f32>,ctx: &Context) -> Self {
        const SPEED: f32 = 500.0;
        let angle = atan2(pos.y.into(),pos.x.into()) as f32;
        Player {
            state: PlayerState::Normal,
            pos,
            time_until_next_shot: Self::SHOT_TIMEOUT,
            angle,
            x_vel: SPEED ,
            y_vel: SPEED ,
            dis_mov: Point2{x:0.0,y:0.0},
            weapon: Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx),
            f_pos: pos,
            square: graphics::Rect::new(pos.x-25.0,pos.y-25.0,50.0,50.0),
        }
    }

    pub fn update(&mut self, amount_x: f32, amount_y: f32, seconds: f32,mouse: Point2<f32>) {
        self.f_pos.x += self.x_vel * seconds * amount_x;
        self.f_pos.y += self.x_vel * seconds * amount_y;
    
      // println!("{:?}",self.f_pos);
       self.angle = atan2((-mouse.y+self.pos.y).into(),(self.pos.x-mouse.x).into()) as f32;
       //self.angle = atan2((self.pos.y-mouse.y).into(),(self.pos.x-mouse.x).into()) as f32;
        //self.angle *= 10.0;
    }

    pub fn draw(&self,ctx: &Context , canvas: &mut graphics::Canvas, assets: &Assets) {
        match self.state {
            PlayerState::Normal => {
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).
                    scale(Vector2 { x: 0.65, y: 0.65 }).
                    offset(Point2 { x: 0.5, y: 0.5 }).rotation(self.angle+182.0_f32.to_radians());
                match self.weapon.w_type
                {
                    WeaponType::SPistol =>  canvas.draw(&assets.player_mac10_stat, draw_params),
                    WeaponType::Rifle =>    canvas.draw(&assets.p_r_s, draw_params),
                    WeaponType::Shotgun =>  canvas.draw(&assets.p_s_s, draw_params),
                }
                
            },

            PlayerState::Shooting => {
                let draw_params = graphics::DrawParam::default().
                    dest(self.pos).scale(Vector2 { x: 0.65, y: 0.65 }).
                    offset(Point2 { x: 0.5, y: 0.5 }).rotation(self.angle+182.0_f32.to_radians());
                    match self.weapon.w_type
                    {
                        WeaponType::SPistol =>  canvas.draw(&assets.player_mac10_shoot, draw_params),
                        WeaponType::Rifle =>    canvas.draw(&assets.p_r_sh, draw_params),
                        WeaponType::Shotgun =>  canvas.draw(&assets.p_s_sh, draw_params),
                    }
            },
        }
       // let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.square,graphics::Color::BLUE).unwrap();
       // canvas.draw(&mesh,graphics::DrawParam::default());
    }
}


#[derive(Debug, Clone)]
pub struct Shot {
    pub pos: Point2<f32>,
    pub is_alive: bool,
    angle: f32,
    pub velocity: Vector2<f32>,
    pub mouse: Point2<f32>,
    old_dist: Point2<f32>,
}

impl Shot {
    pub fn new(pos: Point2<f32>,mouse: Point2<f32>) -> Self {
        let angle = atan2((pos.y-mouse.y).into(),(pos.x-mouse.x).into()) as f32;
        Shot {
            pos,
            is_alive: true,
            angle,
            velocity: Vector2 { x: cos(angle.into()) as f32 * 1500.0, y: sin(angle.into()) as f32  *  1500.0 },  //1500
            mouse,
            old_dist: Point2{x:1.0,y:1.0},

        }
    }

    pub fn update(&mut self,pos: Point2<f32>,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32) {
        //self.angle = atan2((pos.y-self.mouse.y).into(),(pos.x-self.mouse.x).into()) as f32;
        //self.velocity = Vector2 { x: cos((self.angle).into()) as f32 * 1000.0, y: sin((self.angle).into()) as f32 * 1000.0};
        //println!("{}",self.angle.to_degrees());
        
        if  self.angle >= -90.0_f32.to_radians() && self.angle < 90.0_f32.to_radians()
        {
            if self.angle >= -90.0_f32.to_radians() && self.angle <= 0_f32.to_radians()
            {   
               // println!(">=-90");
                self.pos.x -=  self.velocity.x * seconds + self.velocity.y * seconds *0.5 * amount_x; //a;
            }
            else {
             //   println!("<-90");
                self.pos.x -=  self.velocity.x * seconds + self.velocity.y * seconds *0.5 * -amount_x;} //a;}
         
         self.pos.y -=  self.velocity.y * seconds + self.velocity.x * seconds *0.5 * -amount_y; //b;
        }
        else {
            if self.angle <= -90.0_f32.to_radians()
            {
               // println!("<=-90");
                self.pos.x -=  self.velocity.x * seconds + self.velocity.y * seconds *0.5 * amount_x; //a;
            }
            else { 
              //  println!(">-90");
                self.pos.x -=  self.velocity.x * seconds + self.velocity.y * seconds *0.5 * -amount_x; }//a;}
            self.pos.y -=  self.velocity.y * seconds + self.velocity.x * seconds *0.5 * amount_y; //b;
        }
       
        //println!("{:?}",self.mouse);
       //  println!("{:?}",self.pos);
       // self.pos.x = self.pos.x * player.pos.x;
        //self.pos.y = self.pos.x * player.pos.y;

    }

    pub fn draw(&mut self, canvas: &mut graphics::Canvas, assets: &Assets) {
        canvas.draw(&assets.shot_image, graphics::DrawParam::default().dest(self.pos));
    }
}

pub struct Enemy {
    pub pos: Point2<f32>,
    pub health : f32, 
    pub dps: f32,
    pub hit_box: Rect,
    pub mesh: Mesh,
    pub is_alive: bool,
    pub velocity: f32,
    pub angle: f32,
    pub mid_point: Point2<f32>,
    pub z: u8,
    pub color: graphics::Color

}
impl Clone for Enemy {
    fn clone(&self) -> Enemy {
       Enemy{ 
        pos: self.pos,
        health: self.health,
        dps:self.dps,
        hit_box:graphics::Rect::new(self.pos.x,self.pos.y,self.health* 0.5,self.health *0.5),
        mesh:self.mesh.clone(),
        is_alive:self.is_alive,
        velocity:self.velocity,
        angle:self.angle,
        mid_point:self.mid_point,
        z:self.z,
        color: self.color
    }
       }
}
impl Enemy {
    const SPEED:f32 = 500.0;
    pub fn new(ctx: &Context,pos: Point2<f32>,health: f32,dps:f32)->Self
    {
        let rect = graphics::Rect::new(pos.x,pos.y,50.0+health* 0.05,50.0+health *0.05);
        let mut color=graphics::Color::GREEN;
        if dps>0.1
        {
            color = graphics::Color::RED;
        }
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),rect,color).unwrap();
        let mid = Point2{x:pos.x+rect.w*0.5,y:pos.y+rect.h*0.5};
        Enemy {pos,health,dps,hit_box: rect,mesh,is_alive:true,velocity:2.3,angle:0.0,mid_point: mid,z:0,color}
    }
    pub fn draw(&mut self,ctx: &Context, canvas: &mut graphics::Canvas, assets: &Assets)
    {
        canvas.draw(&self.mesh, graphics::DrawParam::default().rotation(self.angle+182.0_f32.to_radians()*0.0));
    }

    pub fn update(&mut self,pos: Point2<f32>,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32,rand_mov:Point2<f32>) {
        self.hit_box.x +=  Self::SPEED * seconds * (amount_x);
        self.hit_box.y +=  Self::SPEED * seconds * (amount_y);
        if self.hit_box.x < pos.x - 20.0 //- rand_mov.x*10.0
        {
            self.hit_box.x+=self.velocity;
           
        }
        if self.hit_box.y < pos.y - 20.0 //- rand_mov.y*10.0
        {
            self.hit_box.y+=self.velocity;
        }
        if self.hit_box.x > pos.x - 10.0 //+ rand_mov.x*10.0
        {
            self.hit_box.x-=self.velocity;
        }
        if self.hit_box.y > pos.y - 10.0 //+ rand_mov.y*10.0
        {
            self.hit_box.y-=self.velocity;
        }
        self.hit_box.x+=rand_mov.x;
        self.hit_box.y+=rand_mov.y;
        self.pos.x = self.hit_box.x;
        self.pos.y = self.hit_box.y;
      //  self.angle = atan2((-pos.y+self.pos.y).into(),(self.pos.x-pos.x).into()) as f32;
        //self.hit_box.move_to(self.pos);
        //println!("{:?}",self.rect.point());
        self.mid_point = Point2{x:self.hit_box.x+self.hit_box.w*0.5,y:self.hit_box.y+self.hit_box.h*0.5};
        self.mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.hit_box,self.color).unwrap();
   }
   pub fn struct_hit(&mut self,pos: Point2<f32>,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32,rect: &Structure)
   {
   // println!("rect.x: {} -- rect.x+w {},mid {}",rect.x,rect.x+rect.w,self.mid_point.x);
        if  self.hit_box.overlaps(&rect.top_side())
        {

            if Self::euclid_dist(pos,Point2{x:rect.rec.x,y:rect.rec.y})> Self::euclid_dist(pos,Point2{x:rect.rec.x+rect.rec.w,y:rect.rec.y})
            {
                self.z=1;
                self.hit_box.x += Self::SPEED*seconds*amount_x + Self::SPEED * seconds ;
            }
            else  {self.z=2;self.hit_box.x +=  Self::SPEED*seconds*amount_x - Self::SPEED * seconds ;}
          //  self.hit_box.y+=Self::SPEED * seconds * amount_y ;
           
        }
        else 
        if  self.hit_box.overlaps(&rect.bottom_side())
        {
            if Self::euclid_dist(pos,Point2{x:rect.rec.x,y:rect.rec.y+rect.rec.h}) > Self::euclid_dist(pos,Point2{x:rect.rec.x+rect.rec.w,y:rect.rec.y+rect.rec.h})
            {   self.z=3;
                self.hit_box.x -=  Self::SPEED*seconds*amount_x + Self::SPEED * seconds ;
            }
            else  {self.z=4;self.hit_box.x +=  Self::SPEED*seconds*amount_x - Self::SPEED * seconds ;}
        }
        else 
        if self.hit_box.overlaps(&rect.left_side())
        {
            if Self::euclid_dist(pos,Point2{x:rect.rec.x,y:rect.rec.y+rect.rec.h}) > Self::euclid_dist(pos,Point2{x:rect.rec.x,y:rect.rec.y})
            {
                self.z=5;
                self.hit_box.y += Self::SPEED*seconds*amount_x + Self::SPEED * seconds ;
            }
            else  {self.z=6;self.hit_box.y +=  Self::SPEED*seconds*amount_x + Self::SPEED * seconds;}
        }
        else 
        if  self.hit_box.overlaps(&rect.right_side())
        {
            if Self::euclid_dist(pos,Point2{x:rect.rec.x,y:rect.rec.y+rect.rec.h}) < Self::euclid_dist(pos,Point2{x:rect.rec.x+rect.rec.w,y:rect.rec.y+rect.rec.h})
            {   
                self.z=7;
                self.hit_box.y +=  Self::SPEED*seconds*amount_y + Self::SPEED * seconds ;
            }
            else  {self.z=8;self.hit_box.y +=  Self::SPEED*seconds*amount_y - Self::SPEED * seconds ;}
        }
        else {self.hit_box.y+=Self::SPEED * seconds * (amount_y) ;   
              self.hit_box.x+=Self::SPEED * seconds * (amount_x) ;
              if self.z==1
              {self.hit_box.x+=Self::SPEED * seconds*0.2 ;}
              if self.z==2
              {self.hit_box.x-=-Self::SPEED * seconds*0.2 ;}
              if self.z==3
              {self.hit_box.x+=Self::SPEED * seconds*0.2;}
              if self.z==4
              {self.hit_box.x-=-Self::SPEED * seconds*0.2;}
              if self.z==5
              {self.hit_box.y+=Self::SPEED * seconds*0.2;}
              if self.z==6
              {self.hit_box.y-=-Self::SPEED * seconds*0.2;}
              if self.z==7
              {self.hit_box.y+=Self::SPEED * seconds*0.2;}
              if self.z==8
              {self.hit_box.y-=-Self::SPEED * seconds*0.2;}            
            }
        
       

        
        self.mid_point = Point2{x:self.hit_box.x+self.hit_box.w*0.5,y:self.hit_box.y+self.hit_box.h*0.5};
        self.mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.hit_box,self.color).unwrap();
        
   }
   pub fn euclid_dist(a:Point2<f32>,b:Point2<f32>)->f32
   {
        libm::sqrt((a.x-b.x).powf(2.0) as f64 + (a.y+b.y).powf(2.0) as f64) as f32
   }
}