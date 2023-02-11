use crate::data::{Structure,Weapon,WeaponType,PickUp};
use rand::Rng;
use rand::rngs::ThreadRng;
use ggez::Context;
use ggez::mint::Point2;
use crate::entities::Player;
use crate::entities::Enemy;
use crate::assets::Hud;
pub struct Ai
{
    pub timer: f32,
    pub count_enemies:usize,
    pub hud: Hud,
    pub drop: u8, 
    pub dist_end: f32,
}
impl Ai
{
    fn weapon(w: &WeaponType)->WeaponType
    { 
        match w
        {
        WeaponType::SPistol => return WeaponType::SPistol,
        WeaponType::Rifle =>   return WeaponType::Rifle,
        WeaponType::Shotgun => return WeaponType::Shotgun,
        }
    }
    pub fn calculate(&mut self,ctx: &Context,c_enemies: usize,bound: &Structure,player: &Player,structs: &Vec<Structure>,hud: &Hud,end: &Structure) -> (Vec<Enemy>,u8,f32)
    {
        //Calculate drop-chance
        //per 5 sec
        let mut moreEnemies = Vec::new();
        if self.timer == 0.0 
        {
            let dist = Enemy::euclid_dist(player.pos,end.rec.center());
            let lr = rand::thread_rng().gen_range(0..=1);
           // println!("{}",self.dist_end - dist);
            if self.dist_end - dist > 550.0 && self.dist_end - dist < 30000.0
            {
                for i in 0..5
                {
                    
                    let mut xx = rand::thread_rng().gen_range(player.pos.x+500.0..player.pos.x+1700.0);//(bound.rec.x..(bound.rec.x+bound.rec.w));
                    let mut yy = rand::thread_rng().gen_range(player.pos.y+500.0..player.pos.y+700.0);//(bound.rec.y..(bound.rec.x+bound.rec.y));
                    if lr == 0
                    {
                        yy = rand::thread_rng().gen_range(player.pos.y-1800.0*0.6..player.pos.y-400.0);
                    }
                    //test successful commit
                    let dps = rand::thread_rng().gen_range(0.2..0.5);
                    let enemy = Enemy::new(ctx,Point2{x:xx,y:yy},100.0,dps); 
                    //println!("Spawn with dps {}",dps);
                    moreEnemies.push(enemy);
            }
        }
            self.dist_end = dist;
            if c_enemies < 5
            {
                for i in 0..10
                {
                    
                    let mut xx = rand::thread_rng().gen_range(player.pos.x+1200.0..player.pos.x+1800.0);//(bound.rec.x..(bound.rec.x+bound.rec.w));
                    let mut yy = rand::thread_rng().gen_range(player.pos.y+1000.0..player.pos.y+1800.0);//(bound.rec.y..(bound.rec.x+bound.rec.y));
                    if lr == 0
                    {
                        xx = rand::thread_rng().gen_range(player.pos.x-1800.0..player.pos.x-1200.0);
                        yy = rand::thread_rng().gen_range(player.pos.y-1800.0..player.pos.y-1000.0);
                    }
                    
                   
                    let enemy = Enemy::new(ctx,Point2{x:xx,y:yy},100.0,0.1); 
                    moreEnemies.push(enemy);
                }
            }
            if c_enemies < 30
            {
                let mut xx = rand::thread_rng().gen_range(player.pos.x-2000.0..player.pos.x-1200.0);//(bound.rec.x..(bound.rec.x+bound.rec.w));
                let mut yy = rand::thread_rng().gen_range(player.pos.y-1800.0..player.pos.y-1000.0);
                //(bound.rec.y..(bound.rec.x+bound.rec.y));
                if lr == 0
                {
                    xx = rand::thread_rng().gen_range(player.pos.x+1200.0..player.pos.x+2000.0);
                    yy = rand::thread_rng().gen_range(player.pos.y+1000.0..player.pos.y+1800.0);
                }
                let enemy = Enemy::new(ctx,Point2{x:xx,y:yy},100.0,0.1); 
                moreEnemies.push(enemy);
            }
            //println!("{}-{}",self.count_enemies,c_enemies);
            if self.count_enemies - c_enemies > 3
            {
                let mut xx = rand::thread_rng().gen_range(player.pos.x+1200.0..player.pos.x+2000.0);//(bound.rec.x..(bound.rec.x+bound.rec.w));
                let mut yy = rand::thread_rng().gen_range(player.pos.y+1000.0..player.pos.y+1800.0);
                //(bound.rec.y..(bound.rec.x+bound.rec.y));
                if lr == 0
                {
                    xx = rand::thread_rng().gen_range(player.pos.x-1800.0..player.pos.x-1200.0);
                    yy = rand::thread_rng().gen_range(player.pos.y-1800.0..player.pos.y-1000.0);
                }
                let h = rand::thread_rng().gen_range(120.0..700.0);
                let enemy = Enemy::new(ctx,Point2{x:xx,y:yy},h,0.1); 
                moreEnemies.push(enemy);
                

                //println!("Spawn Health+ enemies")
            }
            for s in structs.iter()
             {
              moreEnemies.retain(|e| !e.hit_box.overlaps(&s.rec) && !e.hit_box.overlaps(&player.square) && e.hit_box.overlaps(&bound.rec));
             }
            self.count_enemies = c_enemies+moreEnemies.len();
        }
        
       
        //Health and ammo;
        let mut amount = 50.0;
        if self.timer as u32 % 60 == 0
        {
          //  println!("{}-{}",self.hud.health,hud.health);
            if self.hud.health - hud.health > 5.0 && self.count_enemies > 10
            {
              //  println!("hud-hp");
                self.drop = rand::thread_rng().gen_range(70..=100);
                if hud.health <= 30.0
                {
                    self.drop = rand::thread_rng().gen_range(85..90);
                }
               
                
            }
            if hud.health >= 30.0 && hud.health <= 80.0 && self.count_enemies > 11 
            {
                //println!("health >=30 <=80 enemies >11");
                match player.weapon.w_type
                {
                    WeaponType::SPistol => {self.drop = rand::thread_rng().gen_range(97..=100);},
                    _=> self.drop = rand::thread_rng().gen_range(60..=100),
                }
               
            }
            else { self.drop = rand::thread_rng().gen_range(0..=98);}
           // println!("num {} , amount {}",self.hud.health,hud.health);
            self.hud.health = hud.health;
            
           
        }
        amount = rand::thread_rng().gen_range(40.0..=75.0);
        (moreEnemies,self.drop,amount)

       
    }
    pub fn new(count_enemies: usize,hud: &Hud)->Self
    {
        let hud = Hud::new(hud.health as u8,hud.ammo,Ai::weapon(&hud.weapon));
        Ai{timer:0.0,count_enemies,hud,drop:0,dist_end: f32::MAX}
    }
    pub fn map_gen(ctx: &Context,bound: &Structure,player: &Player,end: &Structure) -> Vec<Structure>
    {
        let num_obj: u8 = rand::thread_rng().gen_range(40..=60);
        let mut obj_vec: Vec<Structure> = Vec::new();
        for i in 0..num_obj
        {
            let xx = rand::thread_rng().gen_range(bound.rec.x+200.0..(bound.rec.x+bound.rec.w-200.0));//(bound.rec.x..(bound.rec.x+bound.rec.w));
            let yy = rand::thread_rng().gen_range(bound.rec.y+200.0..(bound.rec.y+bound.rec.h-200.0));//(bound.rec.y..(bound.rec.x+bound.rec.y));
            let obj = Structure::new(ctx,Point2{x:xx,y:yy});
           
            if i>0
            {
              obj_vec.retain(|s| !s.rec.overlaps(&end.rec) && s.rec != obj.rec && !obj.rec.overlaps(&s.rec) && !s.rec.overlaps(&player.square));
            }
            obj_vec.push(obj);
            
            
            
        }
        obj_vec
    }
    pub fn enemies_gen(ctx: &Context,max: u32,player: &Player,structs: &Vec<Structure>,bound: &Structure) -> Vec<Enemy>
    {
       
        let mut enemies: Vec<Enemy> = Vec::new();
        for i in 0..max
        {
            let xx = rand::thread_rng().gen_range(bound.rec.x..(bound.rec.x+bound.rec.w));//(bound.rec.x..(bound.rec.x+bound.rec.w));
            let yy = rand::thread_rng().gen_range(bound.rec.y..(bound.rec.y+bound.rec.h));//(bound.rec.y..(bound.rec.x+bound.rec.y));
            let enemy = Enemy::new(ctx,Point2{x:xx,y:yy},100.0,0.1);
            enemies.push(enemy);
           

        }
        for s in structs.iter()
        {
          enemies.retain(|e| !e.hit_box.overlaps(&s.rec) && !e.hit_box.overlaps(&player.square) && e.hit_box.overlaps(&bound.rec));
        }
        enemies
    }
    pub fn timer(&mut self)
    {
        if self.timer >= 5.0*60.0
        {self.timer=0.0;}
        else {
            self.timer+=1.0;
        }
       
    }
}