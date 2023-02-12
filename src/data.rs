
use ggez::Context;
use ggez::GameResult;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use ggez::graphics;
use ggez::mint::{Vector2, Point2};
use crate::assets::{Assets};
#[derive(Debug)]
pub struct Structure {
    mesh: Mesh,
    pub rec: Rect,
    pos: Point2<f32>,
    color: graphics::Color
}

impl Structure {
    const SPEED:f32 = 500.0;
    pub fn new_rect(ctx: &Context,rect: &mut Rect,scale: f32)->Self
    {
        rect.w = rect.w+scale;
        rect.h = rect.h+scale;
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),*rect,graphics::Color::YELLOW).unwrap();
        Structure{mesh,rec:*rect,pos:rect.point(),color:graphics::Color::YELLOW}
    }
    pub fn new_color_size(ctx: &Context,pos: Point2<f32>,color:graphics::Color,h:f32,w:f32) -> Self
    {
        let rec = graphics::Rect::new(pos.x,pos.y,w,h);
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),rec,color).unwrap();
        Structure{mesh,rec,pos,color}
       
    }
    pub fn new(ctx: &Context,pos: Point2<f32>) -> Self
    {
        let rec = graphics::Rect::new(pos.x,pos.y,200.0,250.0);
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),rec,graphics::Color::RED).unwrap();
        Structure{mesh,rec,pos,color:graphics::Color::RED}
    }
    pub fn update(&mut self,pos: Point2<f32>,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32) { 
            self.rec.x += Self::SPEED * seconds * amount_x;
            self.rec.y += Self::SPEED * seconds * amount_y;
            self.pos.x = self.rec.x;
            self.pos.y = self.rec.y;
            //self.rec.move_to(Point2{x:self.rec.x-pos.x,y:self.rec.y-pos.y});
            self.mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.rec,self.color).unwrap();
           
       }
    pub fn draw(&mut self,ctx: &Context, canvas: &mut graphics::Canvas) {
            let dp = graphics::DrawParam::default();//.rotation(1.0);
            canvas.draw(&self.mesh,dp);
            let mut mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.left_side(),graphics::Color::GREEN).unwrap();
         //  canvas.draw(&mesh,dp);
         //   mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.right_side(),graphics::Color::GREEN).unwrap();
         //   canvas.draw(&mesh,dp);
         //  mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.top_side(),graphics::Color::GREEN).unwrap();
          //   canvas.draw(&mesh,dp);
         //   mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.bottom_side(),graphics::Color::GREEN).unwrap();
          // canvas.draw(&mesh,dp);
        }
    pub fn left_side(&self)->Rect
    {
        graphics::Rect::new(self.rec.point().x-10.0,self.rec.point().y+10.0,10.0,self.rec.h-20.0)
    }
    pub fn top_side(&self)->Rect
    {
        graphics::Rect::new(self.rec.point().x,self.rec.point().y,self.rec.w,10.0)
    }
    pub fn bottom_side(&self)->Rect
    {
        graphics::Rect::new(self.rec.point().x,self.rec.point().y+self.rec.h-10.0,self.rec.w,10.0)
    }
    pub fn right_side(&self)->Rect
    {
        graphics::Rect::new(self.rec.point().x+self.rec.w,self.rec.point().y+10.0,10.0,self.rec.h-20.0)
    }
    
}
#[derive(Debug)]
pub enum WeaponType {
    SPistol,
    Rifle,
    Shotgun,
}
#[derive(Debug)]
pub struct Weapon {
    pub w_type: WeaponType,
    pub damage: f32,
    pub recoil: Point2<f32>,
    pub fire_rate: f32,
    pub pos : Point2<f32>,
    pub rect: Rect,
    pub pick_box: Mesh,
    pub ammo: u32,
    pub max_ammo: (u32,u32,u32),
    default_recoil: Point2<f32>,
}

impl Weapon {
    const SPEED:f32 = 500.0;
    pub fn new(w_type: WeaponType,pos: Point2<f32>,ctx: &Context) -> Self
    {
        let p = graphics::Rect::new(pos.x,pos.y,512.0*0.1,512.0*0.1);
        let r = graphics::Rect::new(pos.x,pos.y,1000.0*0.1,460.0*0.1);
        let s = graphics::Rect::new(pos.x,pos.y,315.0*0.3,250.0*0.1);
        let mp = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),p,graphics::Color::GREEN).unwrap();
        let mr = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),r,graphics::Color::GREEN).unwrap();
        let ms = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),s,graphics::Color::GREEN).unwrap();
        let max_ammo = (u32::MAX,100,12);
        match &w_type {
            WeaponType::SPistol => Weapon {w_type,damage: 11.0,recoil: Point2{x:-0.1,y:0.1},fire_rate:10.0,pos,pick_box:mp,rect:p,ammo:u32::MAX,
            default_recoil: Point2{x:-7.0,y:7.0},max_ammo},
            WeaponType::Rifle => Weapon {w_type,damage: 25.0,recoil: Point2{x:-5.0,y:5.0},fire_rate:20.0,pos,pick_box:mr,rect:r,ammo:100,
            default_recoil: Point2{x:-5.0,y:5.0},max_ammo},
            WeaponType::Shotgun => Weapon {w_type,damage: 25.0, recoil: Point2{x:-1.0,y:1.0},fire_rate:2.0,pos,pick_box:ms,rect:s,ammo:12,
            default_recoil: Point2{x:-1.0,y:1.0},max_ammo},
        }
    }
    pub fn bounds(&self,p: Point2<f32>)->bool
    {
        self.rect.contains(p)
    }
    pub fn update(&mut self,pos: Point2<f32>,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32)
    {
            self.pos.x +=  Self::SPEED * seconds * amount_x;
            self.pos.y +=  Self::SPEED * seconds * amount_y;
            self.rect.move_to(self.pos);
            //println!("{:?}",self.rect.point());
            self.pick_box = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.rect,graphics::Color::GREEN).unwrap();
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas,assets: &Assets)
    {
        let draw_params = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.1, y: 0.1 });
        let draw_params_s = graphics::DrawParam::default().dest(self.pos).scale(Vector2 { x: 0.3, y: 0.3 }).offset(Point2{x:0.0,y:0.3});
        match &self.w_type
        {
            WeaponType::SPistol => {canvas.draw(&assets.mpistol, draw_params);},//canvas.draw(&self.pick_box, graphics::DrawParam::default());
            WeaponType::Rifle =>   {canvas.draw(&assets.rifle, draw_params);},//canvas.draw(&self.pick_box, graphics::DrawParam::default());
            WeaponType::Shotgun => {canvas.draw(&assets.shotgun, draw_params_s);},//canvas.draw(&self.pick_box, graphics::DrawParam::default());
        }
  
    }
    pub fn shoot(&mut self)
    {
        if self.ammo > 0
        {
            self.ammo-=1;
        }
        if self.ammo == 0
        {
            if let WeaponType::SPistol = self.w_type
            {
                self.ammo = u32::MAX;
            }
        }
        if self.recoil.x < -50.0
        {
            return;
        }
        self.recoil.x-=1.0;
        self.recoil.y+=1.0;

    }
    pub fn cooldown(&mut self)
    {
       if self.recoil.x < self.default_recoil.x {self.recoil.x +=0.5;self.recoil.y -=0.5;}
    }

}

#[derive(Debug)]
pub struct PickUp
{
    pub amount: f32,
    pub is_health: bool,
    pub pick: Rect,
    pub mesh: Mesh,
}

impl PickUp
{
    pub fn new(ctx: &Context,pos:Point2<f32>,amount: f32,is_health:bool)->Self
    {   let pick = graphics::Rect::new(pos.x,pos.y,96.0*0.5756*amount*0.01,96.0*0.5756*amount*0.01);
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),pick,graphics::Color::GREEN).unwrap();
        PickUp{amount,is_health,pick,mesh}
    }
    pub fn update(&mut self,ctx: &Context,seconds: f32,amount_x:f32,amount_y:f32)
    {
            self.pick.x +=  500.0 * seconds * amount_x;
            self.pick.y +=  500.0 * seconds * amount_y;
            //println!("{:?}",self.rect.point());
            self.mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),self.pick,graphics::Color::GREEN).unwrap();
    }
    pub fn draw(&self, canvas: &mut graphics::Canvas,assets: &Assets)
    {
        let draw_params = graphics::DrawParam::default().dest(Point2{x:self.pick.x-20.0,y:self.pick.y-25.0}).scale(Vector2 {x:self.amount*0.01,y:self.amount*0.01});
        //let draw2 = graphics::DrawParam::default();
        if self.is_health
        {
          //  canvas.draw(&self.mesh,draw2);
            canvas.draw(&assets.health,draw_params);
            
        }
        else {
           // canvas.draw(&self.mesh,draw2);
            canvas.draw(&assets.ammo,draw_params);
             }
       
    }
}