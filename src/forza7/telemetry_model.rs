use crate::forza7::convert;
use bevy::prelude::Resource;

#[allow(dead_code)]
#[derive(Default, Debug, Resource)]
pub struct ForzaTelemtry {
    pub is_race_on: i32,
    pub timestamp_ms: u32,

    pub engine: Engine,
    pub acceleration: Acceleration,
    pub velocity: Velocity,
    pub suspension_tarvel: SuspensionTravel,
    pub tire_slip_ratio: TireSlipRatio,
    pub wheel_rotation_speed: WheelRotationSpeed,
    pub rumble_strip: RumbleStrip,
    pub wheel_in_puddle: WheelInPuddle,
    pub surface_rumble: SurfaceRumble,
    pub tire_slip_angle: TireSlipAngle,
    pub tire_combined_slip: TireCombinedSlip,
    pub suspension_travel_meters: SuspensionTravelMeters,
    pub car_info: CarInfo,
    pub position: Position,
    pub tire_temp: TireTemp,
    pub lap: Lap,
    pub controls: Controls,
    pub normalized: Normalized,
}

impl ForzaTelemtry {
    pub fn update(&mut self, bytes: Vec<u8>) {
        self.is_race_on = convert::convert_to_i32(&bytes, 0);
        self.timestamp_ms = convert::convert_to_u32(&bytes, 4);

        self.engine.max_rpm = convert::convert_to_f32(&bytes, 8);
        self.engine.idle_rpm = convert::convert_to_f32(&bytes, 12);
        self.engine.rpm = convert::convert_to_f32(&bytes, 16);

        self.acceleration.x = convert::convert_to_f32(&bytes, 20);
        self.acceleration.y = convert::convert_to_f32(&bytes, 24);
        self.acceleration.z = convert::convert_to_f32(&bytes, 28);

        self.velocity.x = convert::convert_to_f32(&bytes, 32);
        self.velocity.y = convert::convert_to_f32(&bytes, 36);
        self.velocity.x = convert::convert_to_f32(&bytes, 40);
        self.velocity.angular_velocity_x = convert::convert_to_f32(&bytes, 44);
        self.velocity.angular_velocity_y = convert::convert_to_f32(&bytes, 48);
        self.velocity.angular_velocity_z = convert::convert_to_f32(&bytes, 52);
        self.velocity.yaw = convert::convert_to_f32(&bytes, 56);
        self.velocity.pitch = convert::convert_to_f32(&bytes, 60);
        self.velocity.roll = convert::convert_to_f32(&bytes, 64);

        self.suspension_tarvel.front_left = convert::convert_to_f32(&bytes, 68);
        self.suspension_tarvel.front_right = convert::convert_to_f32(&bytes, 72);
        self.suspension_tarvel.rear_left = convert::convert_to_f32(&bytes, 76);
        self.suspension_tarvel.rear_right = convert::convert_to_f32(&bytes, 80);

        self.tire_slip_ratio.front_left = convert::convert_to_f32(&bytes, 84);
        self.tire_slip_ratio.front_right = convert::convert_to_f32(&bytes, 88);
        self.tire_slip_ratio.rear_left = convert::convert_to_f32(&bytes, 92);
        self.tire_slip_ratio.rear_right = convert::convert_to_f32(&bytes, 96);

        self.wheel_rotation_speed.front_left = convert::convert_to_f32(&bytes, 100);
        self.wheel_rotation_speed.front_right = convert::convert_to_f32(&bytes, 104);
        self.wheel_rotation_speed.rear_left = convert::convert_to_f32(&bytes, 108);
        self.wheel_rotation_speed.rear_right = convert::convert_to_f32(&bytes, 112);

        self.rumble_strip.front_left = convert::convert_to_i32(&bytes, 116);
        self.rumble_strip.front_right = convert::convert_to_i32(&bytes, 120);
        self.rumble_strip.rear_left = convert::convert_to_i32(&bytes, 124);
        self.rumble_strip.rear_right = convert::convert_to_i32(&bytes, 128);

        self.wheel_in_puddle.front_left = convert::convert_to_f32(&bytes, 132);
        self.wheel_in_puddle.front_right = convert::convert_to_f32(&bytes, 136);
        self.wheel_in_puddle.rear_left = convert::convert_to_f32(&bytes, 140);
        self.wheel_in_puddle.rear_right = convert::convert_to_f32(&bytes, 144);

        self.surface_rumble.front_left = convert::convert_to_f32(&bytes, 148);
        self.surface_rumble.front_right = convert::convert_to_f32(&bytes, 152);
        self.surface_rumble.rear_left = convert::convert_to_f32(&bytes, 156);
        self.surface_rumble.rear_right = convert::convert_to_f32(&bytes, 160);

        self.tire_slip_angle.front_left = convert::convert_to_f32(&bytes, 164);
        self.tire_slip_angle.front_right = convert::convert_to_f32(&bytes, 168);
        self.tire_slip_angle.rear_left = convert::convert_to_f32(&bytes, 172);
        self.tire_slip_angle.rear_right = convert::convert_to_f32(&bytes, 176);

        self.tire_combined_slip.front_left = convert::convert_to_f32(&bytes, 180);
        self.tire_combined_slip.front_right = convert::convert_to_f32(&bytes, 184);
        self.tire_combined_slip.rear_left = convert::convert_to_f32(&bytes, 188);
        self.tire_combined_slip.rear_right = convert::convert_to_f32(&bytes, 192);

        self.suspension_travel_meters.front_left = convert::convert_to_f32(&bytes, 196);
        self.suspension_travel_meters.front_left = convert::convert_to_f32(&bytes, 200);
        self.suspension_travel_meters.front_left = convert::convert_to_f32(&bytes, 204);
        self.suspension_travel_meters.front_left = convert::convert_to_f32(&bytes, 208);

        self.car_info.car_ordinal = convert::convert_to_i32(&bytes, 212);
        self.car_info.car_class = convert::convert_to_i32(&bytes, 216);
        self.car_info.car_performance_index = convert::convert_to_i32(&bytes, 220);
        self.car_info.drivetrain = convert::convert_to_i32(&bytes, 224);
        self.car_info.cylinders = convert::convert_to_i32(&bytes, 228);

        self.position.x = convert::convert_to_f32(&bytes, 232);
        self.position.y = convert::convert_to_f32(&bytes, 236);
        self.position.z = convert::convert_to_f32(&bytes, 240); 

        self.engine.speed = convert::convert_to_f32(&bytes, 244);
        self.engine.power = convert::convert_to_f32(&bytes, 248);
        self.engine.torque = convert::convert_to_f32(&bytes, 252);

        self.tire_temp.front_left = convert::convert_to_f32(&bytes, 256);
        self.tire_temp.front_right = convert::convert_to_f32(&bytes, 260);
        self.tire_temp.rear_left = convert::convert_to_f32(&bytes, 264);
        self.tire_temp.rear_right = convert::convert_to_f32(&bytes, 268);

        self.engine.boost = convert::convert_to_f32(&bytes, 272);
        self.engine.fuel = convert::convert_to_f32(&bytes, 276);

        self.lap.distance_traveled = convert::convert_to_f32(&bytes, 280);
        self.lap.best_lap = convert::convert_to_f32(&bytes, 284);
        self.lap.last_lap = convert::convert_to_f32(&bytes, 288);
        self.lap.current_lap = convert::convert_to_f32(&bytes, 292);
        self.lap.current_race_time = convert::convert_to_f32(&bytes, 296);

        self.lap.lap_number = convert::convert_to_u16(&bytes, 300);
        self.lap.position = convert::convert_to_u8(&bytes, 302);

        self.controls.accelerator = convert::convert_to_u8(&bytes, 303);
        self.controls.brake = convert::convert_to_u8(&bytes, 304);
        self.controls.clutch = convert::convert_to_u8(&bytes, 305);
        self.controls.handbrake = convert::convert_to_u8(&bytes, 306);
        self.controls.gear = convert::convert_to_u8(&bytes, 307);
        self.controls.steer = convert::convert_to_i8(&bytes, 308);

        self.normalized.driving_line = convert::convert_to_i8(&bytes, 309);
        self.normalized.ai_brake_difference = convert::convert_to_i8(&bytes, 310);
        
    }
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Engine {
    pub max_rpm: f32,
    pub idle_rpm: f32,
    pub rpm: f32,
    pub speed: f32,  // Meters per second
    pub power: f32,  // watts
    pub torque: f32, // newton meter
    pub boost: f32,
    pub fuel: f32,
}

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct Acceleration {
    x: f32,
    y: f32,
    z: f32,
}

//In the car’s local space; X = pitch, Y = yaw, Z = roll
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Velocity {
    x: f32,
    y: f32,
    z: f32,

    angular_velocity_x: f32,
    angular_velocity_y: f32,
    angular_velocity_z: f32,

    yaw: f32,
    pitch: f32,
    roll: f32,
}

// Suspension travel normalized: 0.0f = max stretch; 1.0 = max compression
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct SuspensionTravel {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Tire normalized slip ratio, = 0 means 100% grip and |ratio| > 1.0 means loss of grip.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct TireSlipRatio {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Wheel rotation speed radians/sec.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct WheelRotationSpeed {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// 1 when wheel is on rumble strip, = 0 when off.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct RumbleStrip {
    front_left: i32,
    front_right: i32,
    rear_left: i32,
    rear_right: i32,
}

// From 0 to 1, where 1 is the deepest puddle
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct WheelInPuddle {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Non-dimensional surface rumble values passed to controller force feedback
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct SurfaceRumble {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Tire normalized slip angle, = 0 means 100% grip and |angle| > 1.0 means loss of grip.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct TireSlipAngle {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Tire normalized combined slip, = 0 means 100% grip and |slip| > 1.0 means loss of grip.
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct TireCombinedSlip {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

// Actual suspension travel in meters
#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct SuspensionTravelMeters {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct CarInfo {
    car_ordinal: i32,           // Unique ID of the car make/model
    car_class: i32,             // Between 0 - 7 [D, C, B, A, S, S2, X]
    car_performance_index: i32, // Slowest 100 - 999 Fastest
    drivetrain: i32,            // 0 = FWD, 1 = RWD, 2 = AWD
    cylinders: i32,             // Number of cylinders in the engine
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Position {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct TireTemp {
    front_left: f32,
    front_right: f32,
    rear_left: f32,
    rear_right: f32,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Lap {
    distance_traveled: f32,
    best_lap: f32,
    last_lap: f32,
    current_lap: f32,
    current_race_time: f32,
    lap_number: u16,
    position: u8,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Controls {
    accelerator: u8,
    brake: u8,
    clutch: u8,
    handbrake: u8,
    gear: u8,
    steer: i8,
}

#[derive(Default, Debug)]
#[allow(dead_code)]
pub struct Normalized {
    driving_line: i8,
    ai_brake_difference: i8,
}
