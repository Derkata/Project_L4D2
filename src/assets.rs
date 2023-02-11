use ggez::audio::{self, SoundSource};
use ggez::graphics::{self, Drawable};
use crate::entities::{Player,PlayerState};
use ggez::{Context, GameResult};
use std::fmt::Debug;
use ggez::mint::{Vector2, Point2};
use ggez::input::mouse::position;
use libm::cos;
use libm::sin;
use libm::atan2;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::Rect;
use crate::data::{WeaponType};

pub struct Assets {
    pub player_mac10_stat:     graphics::Image,
    pub player_mac10_shoot:    graphics::Image,
    pub p_r_s:                 graphics::Image, //player  rifle static
    pub p_r_sh:                graphics::Image, //player  rifle shoot
    pub p_s_s:                 graphics::Image, 
    pub p_s_sh:                graphics::Image, 
    pub shot_image:            graphics::Image,
    pub crosshair_normal:      graphics::Image,
    pub crosshair_shoot:       graphics::Image,
    pub mpistol:               graphics::Image,
    pub rifle:                 graphics::Image,
    pub shotgun:               graphics::Image,
    pub health:                graphics::Image,
    pub ammo:                  graphics::Image,

    pub shot_sound_p: audio::Source,
    pub shot_sound_r: audio::Source,
    pub shot_sound_s: audio::Source,
}

impl Assets {
    pub fn new(ctx: &mut Context) -> GameResult<Assets> {
        let player_mac10_stat     = graphics::Image::from_path(ctx,"/player_mac10_stat.png")?;
        let player_mac10_shoot    = graphics::Image::from_path(ctx,"/player_mac10_shoot.png")?;
        let shot_image            = graphics::Image::from_path(ctx,"/shot.png")?;
        let crosshair_normal      = graphics::Image::from_path(ctx,"/crosshair.png")?;
        let crosshair_shoot       = graphics::Image::from_path(ctx,"/crosshair_shoot.png")?;
        let mpistol               = graphics::Image::from_path(ctx,"/mpistol.png")?;
        let rifle                 = graphics::Image::from_path(ctx,"/rifle.png")?;
        let shotgun               = graphics::Image::from_path(ctx,"/shotgun.png")?;
        let p_r_s                 = graphics::Image::from_path(ctx,"/player_rifle_stat.png")?;
        let p_r_sh                = graphics::Image::from_path(ctx,"/player_rifle_shoot.png")?;
        let p_s_s                 = graphics::Image::from_path(ctx,"/player_shotgun_stat.png")?;
        let p_s_sh                = graphics::Image::from_path(ctx,"/player_shotgun_shoot.png")?;
        let health                = graphics::Image::from_path(ctx,"/aid.png")?;
        let ammo                  = graphics::Image::from_path(ctx,"/ammo.png")?;


        let mut shot_sound_p = audio::Source::new(ctx, "/sound/mpistol.wav")?;
        shot_sound_p.set_volume(0.05);
        let mut shot_sound_r = audio::Source::new(ctx, "/sound/rifle.wav")?;
        shot_sound_r.set_volume(0.5);
        let mut shot_sound_s = audio::Source::new(ctx, "/sound/shotgun.wav")?;
        shot_sound_s.set_volume(0.5);
        

        Ok(Assets {
            player_mac10_stat, player_mac10_shoot,p_r_s,p_r_sh,p_s_s,p_s_sh,shot_image,crosshair_normal,crosshair_shoot,mpistol,rifle,shotgun,
            health,ammo,shot_sound_p,shot_sound_r,shot_sound_s
        })
    }
}
pub trait MockSimple //Mock init for Crosshair ,Player,Shot,Structure
{
    fn new_mock(pos: Point2<f32>)->Self;
    fn update_mock(&mut self,pos: Point2<f32>,seconds: f32,amount_x:f32,amount_y:f32,mouse: Point2<f32>);
}

pub trait MockHud
{
    fn new_mock(health: u8,ammo: u32,weapon: WeaponType)->Self;

}


#[derive(Debug)]
pub struct Crosshair {
    pos: Point2<f32>
}

impl Crosshair {
    pub fn new(pos: Point2<f32>)-> Self
    {
        Crosshair {pos}
    }
    pub fn update(&mut self,ctx: &mut Context) {
        let p = ctx.mouse.position();
        self.pos = Point2{x: p.x-30.4,y: p.y-28.6};
    }
    pub fn draw(&mut self, canvas: &mut graphics::Canvas, assets: &Assets,player: &Player,time: u16) {
        let t = time as f32;
        match player.state {
            PlayerState::Normal => {
                let draw_params = graphics::DrawParam::default().
                    dest(Point2{x: self.pos.x-t*0.01,y: self.pos.y+10.5+t*0.01}).scale(Vector2 {  x: 0.20+t*0.01, y: 0.20+t*0.01 });
                canvas.draw(&assets.crosshair_normal, draw_params);
            },

            PlayerState::Shooting => {
                let draw_params = graphics::DrawParam::default().
                    dest(Point2{x: self.pos.x-10.5-t*0.01,y: self.pos.y+t*0.01}).scale(Vector2 { x: 0.20+t*0.01, y: 0.20+t*0.01 });
                canvas.draw(&assets.crosshair_shoot, draw_params);
            },
        }
    }

}

pub struct Hud {
    pub health: f32,
    pub ammo: u32,
    pub weapon: WeaponType,
}
impl Hud
{
    pub fn new(health: u8,ammo: u32,weapon: WeaponType) -> Self
    {
        let h = health as f32;
        Hud{health: h,ammo,weapon}
    }
    pub fn update_health(&mut self,amount: u8,add : bool)
    {
        if add
        {self.health+=amount as f32; return ();}
        self.health-=amount as f32
        
    }
    pub fn update_ammo(&mut self,amount: u8,add : bool)
    {
        if add
        {self.health+=amount as f32; return ();}
        self.health-=amount as f32
    }
    pub fn draw(&mut self,canvas: &mut graphics::Canvas,ctx: &Context,assets: &Assets,ammo1: u32)
    {
        //health
        let rec = graphics::Rect::new(-10.0,-10.0,200.0,50.0);
        let mut color = graphics::Color::WHITE;

        if self.health > 100.0
        {
            self.health = 100.0;
        }
        if self.health >= 0.0 && self.health <=25.0
        {
            color=graphics::Color::RED;

        }
        if self.health > 25.0 && self.health <= 60.0
        {
            color=graphics::Color::YELLOW;
        }
        if self.health > 60.0
        {
            color=graphics::Color::GREEN
        }
        let mesh = graphics::Mesh::new_rectangle(ctx,DrawMode::fill(),rec,color).unwrap();
        let dp = graphics::DrawParam::default().scale(Vector2 { x: self.health as f32 *0.01, y: 1.00 });//.rotation(1.0);
        canvas.draw(&mesh,dp);

        //ammo
        //weapon
        let draw_params = graphics::DrawParam::default().scale(Vector2 { x: 0.20, y: 0.20 }).
        offset(Point2 { x: 0.01, y: -0.391 });
        let mut inf = false;
        match &self.weapon
        {
            WeaponType::SPistol => {canvas.draw(&assets.mpistol, draw_params);inf = true;},
            WeaponType::Rifle =>   canvas.draw(&assets.rifle, draw_params),
            WeaponType::Shotgun => canvas.draw(&assets.shotgun, graphics::DrawParam::default().scale(Vector2 { x: 0.50, y: 0.50 }).
            offset(Point2 { x: 0.01, y: -0.391 })),
        }
        if inf {
            let mut ammo = graphics::Text::new(format!("Ammo: ∞"));
            ammo.set_scale(graphics::PxScale::from(25.0));
            canvas.draw(&ammo,graphics::DrawParam::default().dest(Point2 { x: 0.5, y: 150.391 }));
        } else {
            let mut ammo = graphics::Text::new(format!("Ammo: {}",  ammo1));
            ammo.set_scale(graphics::PxScale::from(25.0));
            canvas.draw(&ammo,graphics::DrawParam::default().dest(Point2 { x: 0.5, y: 150.391 }));
        }
        let mut h = graphics::Text::new(format!("Health: {} ",self.health as u8));
            h.set_scale(graphics::PxScale::from(30.0));
            canvas.draw(&h,graphics::DrawParam::default().dest(Point2 { x: 0.5, y: 5.391 }).color(graphics::Color::BLACK));
       
    }

}