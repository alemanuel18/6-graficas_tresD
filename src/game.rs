//! Reglas de la partida: niveles, enemigos, armas y condiciones terminales.

use crate::{
    controls::update_player,
    map::{Map, TILE_SIZE},
    player::Player,
    raycaster::cast_ray,
};
use raylib::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Weapon {
    Pistol,
    Shotgun,
}

impl Weapon {
    pub fn name(self) -> &'static str {
        match self {
            Self::Pistol => "PISTOLA",
            Self::Shotgun => "ESCOPETA",
        }
    }
    fn damage(self) -> i32 {
        match self {
            Self::Pistol => 25,
            Self::Shotgun => 70,
        }
    }
    fn cooldown(self) -> f32 {
        match self {
            Self::Pistol => 0.22,
            Self::Shotgun => 0.9,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Enemy {
    pub position: Vector2,
    pub hp: i32,
    pub boss: bool,
    pub attack_timer: f32,
    pub hurt_timer: f32,
}

impl Enemy {
    fn new(position: Vector2, boss: bool) -> Self {
        Self {
            position,
            hp: if boss { 240 } else { 60 },
            boss,
            attack_timer: 1.5,
            hurt_timer: 0.0,
        }
    }
}

pub struct Level {
    pub map: Map,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub weapon: Weapon,
    pub health: i32,
    pub shot_timer: f32,
    pub muzzle_timer: f32,
    pub animation_time: f32,
    pub fired_this_frame: bool,
}

impl Level {
    pub fn load(number: usize) -> std::io::Result<Self> {
        let path = if number == 2 {
            "src/map/map2.txt"
        } else {
            "src/map/map.txt"
        };
        let map = Map::load(path)?;
        let mut enemies: Vec<Enemy> = map
            .positions_of('e')
            .into_iter()
            .map(|p| Enemy::new(p, false))
            .collect();
        enemies.extend(
            map.positions_of('b')
                .into_iter()
                .map(|p| Enemy::new(p, true)),
        );
        Ok(Self {
            player: Player::new(map.player_spawn()),
            map,
            enemies,
            weapon: Weapon::Pistol,
            health: 100,
            shot_timer: 0.0,
            muzzle_timer: 0.0,
            animation_time: 0.0,
            fired_this_frame: false,
        })
    }

    pub fn update(&mut self, window: &RaylibHandle, delta: f32) -> Event {
        update_player(&mut self.player, &self.map, window, delta);
        self.animation_time += delta;
        self.fired_this_frame = false;
        self.shot_timer = (self.shot_timer - delta).max(0.0);
        self.muzzle_timer = (self.muzzle_timer - delta).max(0.0);
        if window.is_key_pressed(KeyboardKey::KEY_ONE) {
            self.weapon = Weapon::Pistol;
        }
        if window.is_key_pressed(KeyboardKey::KEY_TWO) {
            self.weapon = Weapon::Shotgun;
        }
        if (window.is_key_pressed(KeyboardKey::KEY_SPACE)
            || window.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT))
            && self.shot_timer == 0.0
        {
            self.fire();
        }
        for enemy in &mut self.enemies {
            if enemy.hp <= 0 {
                continue;
            }
            enemy.hurt_timer = (enemy.hurt_timer - delta).max(0.0);
            let distance = enemy.position.distance_to(self.player.position);
            if distance < 190.0 {
                enemy.attack_timer -= delta;
                if enemy.attack_timer <= 0.0 {
                    self.health -= if enemy.boss { 18 } else { 8 };
                    enemy.attack_timer = if enemy.boss { 0.8 } else { 1.5 };
                }
            }
        }
        if self.health <= 0 {
            return Event::Defeat;
        }
        if self.enemies.iter().all(|enemy| enemy.hp <= 0) {
            return Event::Victory;
        }
        Event::Continue
    }

    fn fire(&mut self) {
        self.shot_timer = self.weapon.cooldown();
        self.muzzle_timer = 0.12;
        self.fired_this_frame = true;
        let forward = self.player.forward();
        let mut target: Option<(usize, f32)> = None;
        for (index, enemy) in self.enemies.iter().enumerate() {
            if enemy.hp <= 0 {
                continue;
            }
            let delta = enemy.position - self.player.position;
            let distance = delta.length();
            let direction = delta / distance.max(0.001);
            let alignment = forward.dot(direction);
            if alignment > 0.94 && target.is_none_or(|(_, old)| distance < old) {
                target = Some((index, distance));
            }
        }
        if let Some((index, _)) = target {
            self.enemies[index].hp -= self.weapon.damage();
            self.enemies[index].hurt_timer = 0.12;
        }
    }
    pub fn alive_enemies(&self) -> usize {
        self.enemies.iter().filter(|e| e.hp > 0).count()
    }

    /// Indica si la mirilla está sobre un enemigo visible y alcanzable.
    pub fn has_aim_target(&self) -> bool {
        let forward = self.player.forward();
        let wall_distance = cast_ray(&self.map, self.player.position, self.player.angle).distance;
        self.enemies.iter().any(|enemy| {
            if enemy.hp <= 0 {
                return false;
            }
            let delta = enemy.position - self.player.position;
            let distance = delta.length();
            let direction = delta / distance.max(0.001);
            forward.dot(direction) > 0.94 && distance < wall_distance + TILE_SIZE * 0.25
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    Continue,
    Victory,
    Defeat,
}
