use ggez::audio::SoundSource;
use ggez::conf::{Conf, WindowMode};
use ggez::event;
use ggez::graphics::{self, Drawable};
use ggez::input::{keyboard,mouse};
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder, GameResult};
use rand::Rng;
use rand::rngs::ThreadRng;
use ggez::input::mouse::MouseButton;
use ggez::conf::FullscreenType;
use libm::sin;
use libm::cos;

use l4d2::entities::{Player, PlayerState, Shot, Enemy};
use l4d2::assets::{Assets, Crosshair,Hud};
use l4d2::data::{Structure,Weapon,WeaponType,PickUp};
use l4d2::debug;
use l4d2::ai_director::Ai;
use std::env;
use std::path;
use std::mem;

#[derive(Debug, Default)]
struct InputState {
    movement_x: f32,
    movement_y: f32,
    fire: bool,
}
enum GameOver {
    Win,
    Loose,
    InProgress,
}

struct MainState {
    rng: ThreadRng,
    game_over: GameOver,
    assets: Assets,
    input: InputState,
    player: Player,
    shots: Vec<Shot>,
    enemies: Vec<Enemy>,//enemy
    screen_width: f32,
    screen_height: f32,
    crosshair: Crosshair,
    structs: Vec<Structure>,
    hud: Hud,
    weapons: Vec<Weapon>,
    time : u16,
    slow : f32,
    picks : Vec<PickUp>,
    button: u8,
    ai: Ai,
    bound: Structure,
    end: Structure,
   // bound: Structure,

}

impl MainState {
    fn drop_item(&mut self,ctx: &Context,e: &Enemy,num: u8,amount: f32)
    {
            if num < 85 // Do nothing  : (91-96) ammo , (85-90) health,(97) shotgun , (98) rifle
            {
             return;
            }
            if num > 90 && num <=96
            {
             self.spawn_ammo(ctx,e.pos,amount);
             return;
            }
            if num >= 85 && num < 90
            {
             self.spawn_health(ctx,e.pos,amount);
             return;
            }
            if num == 97
            {
             self.spawn_weapon(ctx,WeaponType::Shotgun,e.pos);
             return;
            }
            if num == 98
            {
             self.spawn_weapon(ctx,WeaponType::Rifle,e.pos);
             return;
            }
            else {return;}
        }

    fn spawn_struct(&mut self,ctx: &Context,wp: Point2<f32>)
    {
        let s = Structure::new(ctx,wp);
        self.structs.push(s);
    }
    fn spawn_health(&mut self,ctx: &Context,pos:Point2<f32>,amount: f32)
    {
        let h = PickUp::new(ctx,pos,amount,true);
        self.picks.push(h);
    }
    fn spawn_ammo(&mut self,ctx: &Context,pos:Point2<f32>,amount: f32)
    {
        let a = PickUp::new(ctx,pos,amount,false);
        self.picks.push(a);
    }

    fn pick_drop_item(&mut self, ctx: &mut Context) {
        for pick in self.picks.iter_mut()
        {
            if pick.pick.contains(self.player.pos)
            {
                if pick.is_health &&  self.hud.health == 100.0
                {
                    break;
                }
                if pick.is_health && self.hud.health+pick.amount > 100.0
                {
                    self.hud.health = 100.0;
                }
                else
                if pick.is_health
                {
                    self.hud.health += pick.amount;
                }
                else {      
                    let mut max = 0;
                    match self.player.weapon.w_type
                    {
                        WeaponType::SPistol => {max = self.player.weapon.max_ammo.0;break},
                        WeaponType::Rifle => max = self.player.weapon.max_ammo.1,
                        WeaponType::Shotgun => max = self.player.weapon.max_ammo.2,

                    }     
                     if self.player.weapon.ammo < max
                    {
                        if self.player.weapon.ammo + pick.amount.round() as u32 > max
                            {self.player.weapon.ammo = max;break;}
                            else {self.player.weapon.ammo += pick.amount.round() as u32;}
                    }
                           
                        
                    }
                    pick.amount=0.0;
                }
                
            }
        for w in self.weapons.iter_mut()
        {
           // println!("{:?}-{:?}",self.player.f_pos,w.rect.point());
            if w.bounds(self.player.pos)
            {
                
                if let WeaponType::SPistol = self.player.weapon.w_type
                {
                    if let WeaponType::Rifle = w.w_type {
                         let mut ded = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
                         ded.ammo = 0;
                         self.player.weapon = mem::replace(w,ded);
                         self.hud.weapon = WeaponType::Rifle;
                    }
                    if let WeaponType::Shotgun = w.w_type {
                         let mut ded = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
                         ded.ammo = 0;
                         self.player.weapon = mem::replace(w,ded);
                         self.hud.weapon = WeaponType::Shotgun;
                    }
                }
                if let WeaponType::Shotgun = w.w_type
                {
                    if w.ammo > self.player.weapon.ammo
                    {
                        let mut ded = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
                         ded.ammo = 0;
                         self.player.weapon = mem::replace(w,ded);
                         self.hud.weapon = WeaponType::Shotgun;

                    }
                }
                if let WeaponType::Rifle = w.w_type
                {
                    if w.ammo > self.player.weapon.ammo
                    {
                         let mut ded = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
                         ded.ammo = 0;
                         self.player.weapon = mem::replace(w,ded);
                         self.hud.weapon = WeaponType::Rifle;

                    }
                }
            }
             
        }
        if let WeaponType::Rifle = self.player.weapon.w_type
        {
           if self.player.weapon.ammo == 0
           {
               let default = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
               self.player.weapon = default;
               self.hud.weapon = WeaponType::SPistol;
           }
        }
        if let WeaponType::Shotgun = self.player.weapon.w_type
        {
           if self.player.weapon.ammo == 0
           {
               let default = Weapon::new(WeaponType::SPistol,Point2{x:0.0,y:0.0},ctx);
               self.player.weapon = default;
               self.hud.weapon = WeaponType::SPistol;
           }
        }
        
    }
       

    fn all_collisions(&mut self)
    {
        let mut slowed = false;
        for e in self.enemies.iter_mut()
        {
            if e.hit_box.contains(self.player.pos)
            {
                slowed = true;
                if self.slow < 0.65
                {
                    self.slow+=0.1;
                }
                if self.slow < -0.65
                {
                    self.slow-=0.1;
                }
                if self.hud.health > e.dps
                {
                     self.hud.health-= e.dps;
                 }
               else {
                self.hud.health = 0.0;
                    }
            }
        }
        
        if !slowed && self.slow != 0.0
        {
            self.slow = 0.0;
        }
      
        for s in self.structs.iter_mut(){
            //1->block A y=0.0 , x=0.0
           // println!("{:?} ,{}",s.rec.point(),i);
            //println!("{:?}",self.player.square.point());
            if s.right_side().overlaps(&self.player.square) //|| self.bound.left_side().overlaps(&self.player.square)
            {
                if self.button != 1 //AWSD  0
                {
                   // println!("Дясна {}",i);
                    self.input.movement_y=0.0;
                    self.input.movement_x=0.0;
                }
                self.button = 1;
                return;
                
               // self.input.movement_x*=0.8;self.input.movement_y*=0.8;
            }else if s.left_side().overlaps(&self.player.square) //|| self.bound.right_side().overlaps(&self.player.square)
            {
               // self.input.movement_x*=0.8;self.input.movement_y*=0.8;
               if self.button != 2
                {
                  //  println!("Лява");
                    self.input.movement_y=0.0;
                    self.input.movement_x=0.0;
                }
                self.button = 2;
                return;
            }else if s.top_side().overlaps(&self.player.square) //|| self.bound.bottom_side().overlaps(&self.player.square)
            {
                //println!("Collision Detected: Top");
               // self.input.movement_x*=0.8;self.input.movement_y*=0.8;
               if self.button != 3
                {
                  //  println!("Горе");
                    self.input.movement_y=0.0;
                    self.input.movement_x=0.0;
                }
                self.button = 3;
                return;
            }else if s.bottom_side().overlaps(&self.player.square)// || self.bound.top_side().overlaps(&self.player.square)
            {
                //self.input.movement_x*=0.8;self.input.movement_y*=0.8;
                if self.button != 4
                {
                   // println!("Долу");
                    self.input.movement_y=0.0;
                    self.input.movement_x=0.0;
                }
                self.button = 4;
                return;
            } else { self.button = 0; continue;}
        }// println!("Cycle ended");
        
       
    }
    fn delete_shot(&mut self)
    {
        let scr = graphics::Rect::new(0.0,0.0,self.screen_width,self.screen_height);
        for shot in self.shots.iter_mut()
        {
            for e in self.enemies.iter_mut() 
            {
                if e.hit_box.contains(shot.pos)
                {
                 if e.health < self.player.weapon.damage
                 {
                    e.health=0.0;
                    shot.is_alive=false;
                    return;
                 }
                 e.health-=self.player.weapon.damage;
                 shot.is_alive = false;
                }
             }
        for s in self.structs.iter_mut(){
            if s.rec.contains(shot.pos) || !scr.contains(shot.pos)
            {
                shot.is_alive=false;
                
            }
          
        }
    }
    }


    fn new(ctx: &mut Context, conf: &Conf) -> GameResult<MainState> {
        mouse::set_cursor_hidden(ctx,true);
        let assets = Assets::new(ctx)?;
        let screen_width = conf.window_mode.width;
        let screen_height = conf.window_mode.height;

        // Player starts in bottom-middle of the screen
        let player_pos = Point2 {
            x: screen_width / 2.0, //-200.0,
            y: screen_height /1.7, //+ 50.0,
        };
       
        let p = Vec::new();
        //let health1 = PickUp::new(ctx,Point2{x:-200.0,y:50.0},100.0,true);
      //  let ammo1 = PickUp::new(ctx,Point2{x:200.0,y:50.0},100.0,false);
        //p.push(health1);
       // p.push(ammo1);
        //test weapons spawn
       // let rp = Point2{x:50.0,y:50.0};
      //  let rifle = Weapon::new(WeaponType::Rifle,rp,ctx);
        let w = Vec::new();
    //    let sp = Point2{x:200.0,y:200.0};
      //  let shotgun = Weapon::new(WeaponType::Shotgun,sp,ctx);
       // let mut enemies =  Vec::new();
        //new(pos: Point2<f32>,health: u8,dps:u8)->Self
      //  let base_enemy = Enemy::new(ctx,Point2{x:1000.0,y:500.0},100.0,0.1);
       // enemies.push(base_enemy);


        // let wp1 = Point2{x:-500.0,y:0.0};
        //let wall2 = Structure::new(ctx,wp1,true);

      // let wp = Point2{x:-200.0,y:500.0};
      // let wall1 = Structure::new(ctx,wp,true);
      // let mut v = Vec::new();

       // v.push(wall1);
       // v.push(wall2);
      //  w.push(rifle);
       // w.push(shotgun);
       
        let bound = Structure::new_color_size(ctx,Point2{x:0.0,y:-500.0},graphics::Color::from_rgb(64, 64, 64),5000.0,5000.0);
       //more structures
        let lbound = Structure::new_rect(ctx,&mut bound.left_side(),200.0);
        let rbound = Structure::new_rect(ctx,&mut bound.right_side(),200.0);
        let tbound = Structure::new_rect(ctx,&mut bound.top_side(),200.0);
        let bbound = Structure::new_rect(ctx,&mut bound.bottom_side(),200.0);

        let player = Player::new(player_pos,ctx);
        let end_pos_y = rand::thread_rng().gen_range(bound.right_side().y+500.0..(bound.right_side().y+bound.rec.y+bound.rec.h-500.0));
        let end = Structure::new_color_size(ctx,Point2{x:(bound.rec.x+bound.rec.w-500.0),y:end_pos_y},graphics::Color::from_rgb(0, 0, 0),100.0,100.0);
        let mut ms = Ai::map_gen(ctx,&bound,&player,&end);
        ms.push(lbound);
        ms.push(rbound);
        ms.push(tbound);
        ms.push(bbound);
        let hud = Hud{health:100.0,ammo: 100,weapon: WeaponType::SPistol};
        let enemies = Ai::enemies_gen(ctx,10,&player,&ms,&bound);
        let ai = Ai::new(enemies.len(),&hud);
       
       // v.append(&mut ms);
        let s = MainState {
            rng: rand::thread_rng(),
            game_over: GameOver::InProgress,
            assets: assets,
            input: InputState::default(),
            player ,
            shots: Vec::new(),
            enemies,
            screen_width: conf.window_mode.width,
            screen_height: conf.window_mode.height,
            crosshair: Crosshair::new(Point2{x:0.0,y:0.0}),
            structs: ms,
            hud,
            weapons: w,
            time : 0,
            slow : 0.0,
            picks: p,
            button: 0,
            ai,
            bound,
            end,
        };

        Ok(s)
    }

}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        if let GameOver::Loose = self.game_over {
            return Ok(());
        }
      
        const DESIRED_FPS: u32 = 60;
        while ctx.time.check_update_time(DESIRED_FPS) {
            let seconds = ctx.time.delta().as_secs_f32();
           // println!("{}",self.ai.timer);
            self.ai.timer();
            self.player.update(self.input.movement_x,self.input.movement_y, seconds,ctx.mouse.position());
            
            self.player.time_until_next_shot -= self.player.weapon.fire_rate*0.9*seconds;
            
            if self.input.fire && self.player.time_until_next_shot < 0.0 {
                let shot_pos = Point2 {
                    x: self.player.pos.x + 70.0*cos(self.player.angle as f64 +182.0_f64.to_radians()) as f32,
                    y: self.player.pos.y + 70.0*sin(self.player.angle as f64 +182.0_f64.to_radians()) as f32,
                };

                let recoil = Point2 {
                    x: self.rng.gen_range(self.player.weapon.recoil.x .. self.player.weapon.recoil.y),
                    y: self.rng.gen_range(self.player.weapon.recoil.x .. self.player.weapon.recoil.y),
                };
                let shot = Shot::new(shot_pos,Point2{x:ctx.mouse.position().x+recoil.x,y:ctx.mouse.position().y+recoil.y});
                self.time+=1;
                if self.time>10
                {
                  self.time = 10;
                }
                self.player.weapon.shoot();
                self.player.weapon.cooldown(self.time);
                let shot1 = Shot::new(shot_pos,Point2{x:ctx.mouse.position().x-10.0+recoil.x,
                    y:ctx.mouse.position().y+10.0+recoil.y});
                let shot2 = Shot::new(shot_pos,Point2{x:ctx.mouse.position().x-5.0+recoil.x,
                    y:ctx.mouse.position().y+5.0+recoil.y});
                let shot3 = Shot::new(shot_pos,Point2{x:ctx.mouse.position().x+10.0+recoil.x,
                    y:ctx.mouse.position().y-10.0+recoil.y});
                let shot4 = Shot::new(shot_pos,Point2{x:ctx.mouse.position().x+5.0+recoil.x,
                    y:ctx.mouse.position().y-5.0+recoil.y});
                if let WeaponType::Shotgun = self.player.weapon.w_type
                {
                    self.shots.push(shot1);self.shots.push(shot2);self.shots.push(shot3);self.shots.push(shot4);
                }
                self.shots.push(shot);

                let _ = self.assets.shot_sound_p.play(ctx);

                self.player.time_until_next_shot = Player::SHOT_TIMEOUT;
                self.player.state = PlayerState::Shooting;
            } else if !self.input.fire {
                self.player.state = PlayerState::Normal;
                if self.time < 1
                {
                  self.time = 1;
                }
                self.time-=1;
            }
            for picks in self.picks.iter_mut() {
                picks.update(self.player.dis_mov,ctx,seconds,self.input.movement_x,self.input.movement_y);
            }
            self.crosshair.update(ctx);
            for shot in self.shots.iter_mut() {
                shot.update(self.player.pos,ctx,seconds,self.input.movement_x,self.input.movement_y);
            }
            for s in self.structs.iter_mut() {
                s.update(self.player.dis_mov,ctx,seconds,self.input.movement_x,self.input.movement_y);
            }

            //test
            for w in self.weapons.iter_mut() {
                w.update(self.player.dis_mov,ctx,seconds,self.input.movement_x,self.input.movement_y);
            }
           

            
            for e in self.enemies.iter_mut() {
                let mut check: bool = false;
                let rand_mov = Point2 {
                    x: self.rng.gen_range(-5.0 .. 5.0),
                    y: self.rng.gen_range(-5.0 .. 5.0),
                };
                for s in self.structs.iter_mut()
                {
                    if s.rec.contains(e.mid_point)
                    {   
                        e.struct_hit(self.player.pos,ctx,seconds,self.input.movement_x,self.input.movement_y,&s);
                        check= true;
                    }
                   
                }
                if !check
                {e.update(self.player.pos,ctx,seconds,self.input.movement_x,self.input.movement_y,rand_mov);}
            }
         
            

            self.shots.retain(|shot| shot.is_alive);
            self.weapons.retain(|w| w.ammo > 0);
            let mut dead: Vec<Enemy> = Vec::new();
            self.end.update(self.player.dis_mov,ctx,seconds,self.input.movement_x,self.input.movement_y);
            self.pick_drop_item(ctx);
            self.all_collisions();
            self.delete_shot();
                for e in self.enemies.iter()
                {
                    if e.health==0.0
                    {
                        dead.push(e.clone());
                    }
                }
            self.enemies.retain(|e| e.health > 0.0);
            let (mut vec,drop,amount) = self.ai.calculate(ctx,self.enemies.len(),&self.bound,&self.player,&self.structs,
                    &self.hud,&self.end);
            self.enemies.append(&mut vec);
            let _ = dead.iter().map(|e| self.drop_item(ctx,e,drop,amount)).collect::<()>();
            
            self.picks.retain(|p| p.amount > 0.0);
            self.bound.update(self.player.dis_mov,ctx,seconds,self.input.movement_x,self.input.movement_y);
            if self.player.square.overlaps(&self.end.rec)
            {
                self.game_over = GameOver::Win;
            }
            if self.hud.health == 0.0
            {
                self.game_over = GameOver::Loose;
            }
            
            
           
        }

        Ok(())
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: keyboard::KeyInput, _repeat: bool) -> GameResult<()> {
        //let k_ctx = &ctx.keyboard;
        match input.keycode {
            Some(keyboard::KeyCode::A) => { if self.button != 1 {
                self.input.movement_x = 1.0 - self.slow;}
            else {self.input.movement_x=0.0;}} ,//self.input.movement_x = -1.0,
            Some(keyboard::KeyCode::D) => {if self.button != 2 {self.input.movement_x = -1.0 + self.slow;}
            else {self.input.movement_x=0.0;}} ,//self.input.movement_x = 1.0,
            Some(keyboard::KeyCode::S) => {if self.button != 3 {self.input.movement_y = -1.0 + self.slow;}
            else {self.input.movement_y=0.0;}} ,//self.input.movement_y = 1.0,
            Some(keyboard::KeyCode::W) => {if self.button != 4 {self.input.movement_y = 1.0 -self.slow;}
            else {self.input.movement_y=0.0;}} ,//self.input.movement_y = -1.0,
            Some(keyboard::KeyCode::Escape) => ctx.request_quit(),
            _ => (), // Do nothing
        }  
        Ok(())
    }
    fn mouse_button_down_event(&mut self, _ctx: &mut Context,button: MouseButton,_x: f32,_y: f32) ->GameResult<()>
    {   
        self.player.state  = PlayerState::Normal;
        self.input.fire = false;
        if let mouse::MouseButton::Left = button 
        {self.input.fire = true;
        }
        Ok(())
    }
    fn mouse_button_up_event(&mut self, _ctx: &mut Context,button: MouseButton,_x: f32,_y: f32) ->GameResult<()>
    {
        if let mouse::MouseButton::Left = button {self.input.fire = false;}
        Ok(()) 
    }
    fn key_up_event(&mut self, ctx: &mut Context, input: keyboard::KeyInput) -> GameResult<()> {
        let k_ctx = &ctx.keyboard;
        match input.keycode {
            Some(keyboard::KeyCode::A) => {
                if !k_ctx.is_key_pressed(keyboard::KeyCode::D) 
                {self.input.movement_x = 0.0;}
            },
            Some(keyboard::KeyCode::D) => {
                if !k_ctx.is_key_pressed(keyboard::KeyCode::A)  
                {self.input.movement_x = 0.0;}
            },
            Some(keyboard::KeyCode::W) => {
                if !k_ctx.is_key_pressed(keyboard::KeyCode::S) 
                {self.input.movement_y = 0.0;}
            },
            Some(keyboard::KeyCode::S) => {
                if !k_ctx.is_key_pressed(keyboard::KeyCode::W) 
                {self.input.movement_y = 0.0;}
            }
            _ => (), 
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let col = graphics::Color::from_rgb(33, 50, 67);
        let mut canvas = graphics::Canvas::from_frame(ctx, col);
       
       
        
        
        if let GameOver::Loose = self.game_over {
            let mut text = graphics::Text::new(format!("Game Over"));
            text.set_scale(graphics::PxScale::from(40.0));

            let top_left = Point2 {
                x: (self.screen_width - text.dimensions(ctx).unwrap().w) * 0.5,
                y: (self.screen_height - text.dimensions(ctx).unwrap().h) * 0.5,
            };
            canvas.draw(&text, graphics::DrawParam::default().dest(top_left));
            canvas.finish(ctx)?;
            return Ok(());
        }else if let GameOver::Win = self.game_over {
            let mut text = graphics::Text::new(format!("You Win"));
            text.set_scale(graphics::PxScale::from(40.0));

            let top_left = Point2 {
                x: (self.screen_width - text.dimensions(ctx).unwrap().w) * 0.5,
                y: (self.screen_height - text.dimensions(ctx).unwrap().h) * 0.5,
            };
            canvas.draw(&text, graphics::DrawParam::default().dest(top_left));
            canvas.finish(ctx)?;
            return Ok(());
        }
        else {
            self.bound.draw(ctx,&mut canvas);
        
       
        for picks in  self.picks.iter_mut()
        {
            picks.draw(&mut canvas, &self.assets);
        }
        for s in self.structs.iter_mut() {
            s.draw(ctx,&mut canvas);
        }
        for w in self.weapons.iter_mut() {

            w.draw(&mut canvas, &self.assets);
           }
       
        self.player.draw(&ctx , &mut canvas, &self.assets);
        
       

        for e in self.enemies.iter_mut() {
            //&mut self,ctx: &Context, canvas: &mut graphics::Canvas, assets: &Assets
            e.draw(ctx,&mut canvas,&self.assets);
        }

        for shot in self.shots.iter_mut() {
            shot.draw(&mut canvas, &self.assets);
        }
        self.end.draw(ctx,&mut canvas);
        self.hud.draw(&mut canvas,ctx,&self.assets,self.player.weapon.ammo);
        self.crosshair.draw(&mut canvas,&self.assets,&self.player,self.time);
        canvas.finish(ctx)?;
        return Ok(());
    }
    }
}

pub fn main() {
    let conf = Conf::new().
        window_mode(WindowMode {
            width: 1200.0,
            height: 1000.0,
            ..Default::default()
        });
    let (mut ctx, event_loop) = ContextBuilder::new("l4d2", "Iskender Chobanov").
        default_conf(conf.clone()).
        build().
        unwrap();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        ctx.fs.mount(&path, true);
    }
    ctx.gfx.set_window_title("L4D2");
    let state = MainState::new(&mut ctx, &conf).unwrap();

    event::run(ctx, event_loop, state);
}


