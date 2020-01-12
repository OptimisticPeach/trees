use android_base::{InputEvent, AppContainer, AppImpl};
use std::sync::{Arc, Mutex};
use piston::input::{Input, Motion, Touch};
use cgmath::{Vector2, InnerSpace, Matrix4, Vector3, Rad, MetricSpace};
use std::f32::consts::{FRAC_PI_4, PI, FRAC_PI_2};

#[derive(Debug)]
pub struct Camera {
    height: f64,
    width_height_units: (f64, f64),
    angle_from_ground: f32, //↕ movement with two fingers
    angle_about_y: f32,     //↔ movement with two fingers
    dist_origin: f32,       //pinch/zoom with two fingers
    x_target: f32,          //↕ movement with one finger
    z_target: f32,          //↔ movement with one finger
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            height: 0.,
            width_height_units: (0., 0.),
            angle_from_ground: FRAC_PI_4,
            angle_about_y: 0.,
            dist_origin: 10.,
            x_target: 0.,
            z_target: 0.,
        }
    }
}

const PERCENT_OF_SCREEN_PER_UNIT: f64 = 0.2;
const MOVEMENT_MARGIN_SQ: f64 = 0.0;

impl Camera {
    pub fn two_fingers_same_direction(&mut self, mut new: Vector2<f64>, mut old: Vector2<f64>) {
        new.y = self.height - new.y;
        old.y = self.height - old.y;
        let mut direction = new - old;
        direction.y /= self.width_height_units.1 * -8.0;
        direction.x /= self.width_height_units.0 * 8.0;

        self.angle_from_ground += direction.y as f32;

        if self.angle_from_ground < (5.0 * 2.0 * PI) / 360.0 { // Five degrees
            self.angle_from_ground = (5.0 * 2.0 * PI) / 360.0;
        } else if self.angle_from_ground > FRAC_PI_2 { // 90 degrees
            self.angle_from_ground = FRAC_PI_2;
        }

        self.angle_about_y += direction.x as f32;

        if self.angle_about_y < -PI {
            self.angle_about_y = 2.0 * PI + self.angle_about_y;
            //self.angle_about_y = -(-PI - self.angle_about_y) + PI;
        } else if self.angle_about_y > PI {
            self.angle_about_y = -2.0 * PI + self.angle_about_y;
            //self.angle_about_y = -(PI - self.angle_about_y) - PI;
        }
    }
    pub fn pan(&mut self, mut new: Vector2<f64>, mut old: Vector2<f64>) {
        new.y = self.height - new.y;
        old.y = self.height - old.y;
        let mut direction: Vector2<f64> = new - old;
        direction.y /= -self.width_height_units.1;
        direction.x /= self.width_height_units.0;

        let (sin, cos) = self.angle_about_y.sin_cos();

        let direction = Vector2 {
            x: direction.x as f32 * cos - direction.y as f32 * sin,
            y: direction.x as f32 * sin + direction.y as f32 * cos,
        };
        let dist_sqrt = self.dist_origin.sqrt();
        self.x_target += direction.x * dist_sqrt;
        self.z_target += direction.y * dist_sqrt;
    }
    pub fn zoom(&mut self, mut new_a: Vector2<f64>, mut new_b: Vector2<f64>, mut old_a: Vector2<f64>, mut old_b: Vector2<f64>) {
        new_a.y = self.height - new_a.y;
        old_a.y = self.height - old_a.y;
        new_b.y = self.height - new_b.y;
        old_b.y = self.height - old_b.y;

        new_a.y /= self.width_height_units.1;
        new_a.x /= self.width_height_units.0;

        old_a.y /= self.width_height_units.1;
        old_a.x /= self.width_height_units.0;

        new_b.y /= self.width_height_units.1;
        new_b.x /= self.width_height_units.0;

        old_b.y /= self.width_height_units.1;
        old_b.x /= self.width_height_units.0;

        let new = new_a - new_b;
        let old = old_a - old_b;

        let distance = old.magnitude() - new.magnitude();

        self.dist_origin += distance as f32 * 3.0;
        if self.dist_origin < 1.0 {
            self.dist_origin = 1.0;
        } else if self.dist_origin > 40.0 {
            self.dist_origin = 40.0;
        }
    }
    pub fn rotate(&mut self, mut new_a: Vector2<f64>, mut new_b: Vector2<f64>, mut old_a: Vector2<f64>, mut old_b: Vector2<f64>) {
        new_a.y = self.height - new_a.y;
        old_a.y = self.height - old_a.y;
        new_b.y = self.height - new_b.y;
        old_b.y = self.height - old_b.y;

        new_a.y /= self.width_height_units.1;
        new_a.x /= self.width_height_units.0;

        old_a.y /= self.width_height_units.1;
        old_a.x /= self.width_height_units.0;

        new_b.y /= self.width_height_units.1;
        new_b.x /= self.width_height_units.0;

        old_b.y /= self.width_height_units.1;
        old_b.x /= self.width_height_units.0;

        let new = new_a - new_b;
        let old = old_a - old_b;

        self.angle_about_y += new.normalize().angle(old.normalize()).0 as f32;
        println!("{:?}", self);
    }

    pub fn mat(&self) -> Matrix4<f32> {
        Matrix4::from_translation(Vector3 { z: -self.dist_origin, y: 0.0, x: 0.0 }) *
            Matrix4::from_angle_x(Rad(self.angle_from_ground)) *
            Matrix4::from_angle_y(Rad(self.angle_about_y)) *
            Matrix4::from_translation(Vector3 {x: self.x_target, z: self.z_target, y: 0.0})
    }
    pub fn size(&mut self, size: (usize, usize)) {
        self.height = size.1 as f64;
        self.width_height_units = (size.0 as f64 * PERCENT_OF_SCREEN_PER_UNIT, size.1 as f64 * PERCENT_OF_SCREEN_PER_UNIT);
    }
}

#[derive(Clone, Copy)]
struct Finger {
    direction: Vector2<f64>,
    old_pos: [f64; 2],
}

impl Finger {
    pub fn with_pos(pos: [f64; 2]) -> Self {
        Self {
            direction: Vector2 {
                x: 0.,
                y: 0.,
            },
            old_pos: pos,
        }
    }

    pub fn update(&mut self, pos: [f64; 2]) {
        let delta = Vector2 {
            x: pos[0] - self.old_pos[0],
            y: pos[1] - self.old_pos[1],
        };
        *self = Self {
            old_pos: pos,
            direction: delta
        };
    }
}

macro_rules! log {
    ($msg:literal) => {
        concat!("[", file!(), ":", line!(), "]: ", $msg)
    }
}

pub fn spawn<T: AppImpl>(app: &mut AppContainer<T>, camera: Arc<Mutex<Camera>>) {
    let mut fingers: Vec<i64> = Vec::new();
    let mut primary: Option<Finger> = None;
    let mut secondary: Option<Finger> = None;
    let resolve_finger_with_two = |
        first: (Vector2<f64>, Vector2<f64>, &mut Finger),
        second: (Vector2<f64>, Vector2<f64>, &mut Finger),
        camera: &Arc<Mutex<Camera>>
    | {
        if first.2.direction.magnitude2() > MOVEMENT_MARGIN_SQ { // More than 16 px in movement
            let normalized_first = first.2.direction.normalize();
            let direction_fingers: Vector2<f64> = (second.1 - first.1).normalize();
            let normalized_second = second.2.direction.normalize();
            let dot_f_s = normalized_first.dot(normalized_second);
            let dot_direction_first = direction_fingers.dot(normalized_first);
            if dot_f_s > 0.8 {
                camera.lock().expect(log!("Camera should always be present")).two_fingers_same_direction(first.0, first.1);
            } else if dot_direction_first > -0.4 && dot_direction_first < 0.4 {
                camera.lock().expect(log!("Camera should always be present")).rotate(first.0, second.0, first.1, second.1);
            } else {
                camera.lock().expect(log!("Camera should always be present")).zoom(first.0, second.0, first.1, second.1);
            }
        }
    };
    app.spawn_user_thread(move |e: InputEvent| -> () {
        if let InputEvent::Piston(input) = e {
            match input {
                Input::Move(motion) => match motion {
                    Motion::Touch(touch) => {
                        match touch.touch {
                            Touch::Start => {
                                fingers.push(touch.id);
                                if fingers.len() == 1 {
                                    primary = Some(Finger::with_pos(touch.position()));
                                } else if fingers.len() == 2 {
                                    secondary = Some(Finger::with_pos(touch.position()));
                                }
                            },
                            Touch::End | Touch::Cancel => {
                                for i in 0..fingers.len() {
                                    if fingers[i] == touch.id {
                                        fingers.remove(i);
                                        if i == 0 {
                                            primary = secondary;
                                            if fingers.len() == 1 {
                                                secondary = Some(Finger::with_pos([0.; 2]));
                                            } else if fingers.len() == 0 {
                                                secondary = None;
                                                primary = None;
                                            } else {
                                                secondary = None;
                                            }
                                        } else if i == 1 {
                                            if fingers.len() > 1 {
                                                secondary = Some(Finger::with_pos([0.; 2]));
                                            } else {
                                                secondary = None;
                                            }
                                        }
                                        break;
                                    }
                                }
                            },
                            Touch::Move => {
                                assert!(!touch.is_3d, "3d touch is not supported!");
                                assert_eq!(touch.device, 0, "Touch inputs from other devices are not supported!");
                                match (fingers.get(0).cloned(), fingers.get(1).cloned()) {
                                    (Some(x), None) if x == touch.id => {
                                        let primary = if let Some(primary) = primary.as_mut() {
                                            primary
                                        } else {
                                            return;
                                        };
                                        let old_pos = primary.old_pos;
                                        primary.update(touch.position());
                                        if primary.direction.magnitude2() > MOVEMENT_MARGIN_SQ {
                                            camera.lock().expect(log!("Camera should always be present")).pan(old_pos.into(), touch.position().into()); // One finger pan
                                        }
                                    },
                                    (Some(x), Some(_)) if x == touch.id => {
                                        let primary = if let Some(primary) = primary.as_mut() {
                                            primary
                                        } else {
                                            return;
                                        };
                                        let old_primary_pos: Vector2<f64> = primary.old_pos.into();
                                        let new_primary_pos: Vector2<f64> = touch.position().into();
                                        let secondary = if let Some(secondary) = secondary.as_mut() {
                                            secondary
                                        } else {
                                            return;
                                        };
                                        let mut old_secondary_pos: Vector2<f64> = secondary.old_pos.into();
                                        old_secondary_pos = old_secondary_pos - secondary.direction;
                                        let new_secondary_pos: Vector2<f64> = secondary.old_pos.into();
                                        primary.update(touch.position());

                                        resolve_finger_with_two(
                                            (new_primary_pos, old_primary_pos, primary),
                                            (new_secondary_pos, old_secondary_pos, secondary),
                                            &camera,
                                        );
                                    },
                                    (Some(_), Some(x)) if x == touch.id => {
                                        let primary = if let Some(primary) = primary.as_mut() {
                                            primary
                                        } else {
                                            return;
                                        };
                                        let mut old_primary_pos: Vector2<f64> = primary.old_pos.into();
                                        old_primary_pos = old_primary_pos - primary.direction;
                                        let new_primary_pos: Vector2<f64> = primary.old_pos.into();
                                        let secondary = if let Some(secondary) = secondary.as_mut() {
                                            secondary
                                        } else {
                                            return;
                                        };
                                        let old_secondary_pos: Vector2<f64> = secondary.old_pos.into();
                                        let new_secondary_pos: Vector2<f64> = touch.position().into();
                                        secondary.update(touch.position());

                                        resolve_finger_with_two(
                                            (new_secondary_pos, old_secondary_pos, secondary),
                                            (new_primary_pos, old_primary_pos, primary),
                                            &camera
                                        );
                                    },
                                    (None, Some(_)) => unreachable!(),
                                    _ => {
                                        if !fingers.contains(&touch.id) {
                                            fingers.push(touch.id);
                                        }
                                    }
                                }
                            },
                        }
                    },
                    Motion::MouseCursor(_) | Motion::MouseRelative(_) | Motion::MouseScroll(_) | Motion::ControllerAxis(_) => {
                        panic!("App not prepared to handle event yet.");
                    },
                },
                Input::Button(_) | Input::Text(_) | Input::Resize(_) | Input::Focus(_) | Input::Cursor(_) | Input::FileDrag(_) | Input::Close(_) => {
                    panic!("App not prepared to handle event yet.")
                },
            }
        } else {
            panic!("Received an unknown event!");
        }
    });
}
