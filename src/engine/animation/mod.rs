use glam::{Quat, Vec3};

/// Interpolation method between keyframes
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Interpolation {
    /// Linear interpolation
    Linear,
    /// Smooth cubic Hermite (ease in/out)
    Smooth,
    /// Discrete step (no interpolation, holds previous value)
    Step,
}

/// A single keyframe at a specific time
#[derive(Clone, Debug)]
pub struct Keyframe<T: Interpolatable> {
    pub time: f32,
    pub value: T,
    pub interpolation: Interpolation,
}

/// A channel of keyframes for a single property
#[derive(Clone, Debug)]
pub struct Channel<T: Interpolatable> {
    keyframes: Vec<Keyframe<T>>,
}

impl<T: Interpolatable + Clone> Channel<T> {
    pub fn new() -> Self {
        Self {
            keyframes: Vec::new(),
        }
    }

    pub fn add_keyframe(&mut self, time: f32, value: T, interpolation: Interpolation) {
        let kf = Keyframe { time, value, interpolation };

        // Insert sorted by time
        let pos = self.keyframes
            .binary_search_by(|k| k.time.partial_cmp(&time).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or_else(|e| e);
        self.keyframes.insert(pos, kf);
    }

    pub fn sample(&self, time: f32) -> Option<T> {
        if self.keyframes.is_empty() {
            return None;
        }

        if self.keyframes.len() == 1 {
            return Some(self.keyframes[0].value.clone());
        }

        // Before first keyframe
        if time <= self.keyframes[0].time {
            return Some(self.keyframes[0].value.clone());
        }

        // After last keyframe
        let last = self.keyframes.last().unwrap();
        if time >= last.time {
            return Some(last.value.clone());
        }

        // Find surrounding keyframes
        for i in 0..self.keyframes.len() - 1 {
            let kf0 = &self.keyframes[i];
            let kf1 = &self.keyframes[i + 1];

            if time >= kf0.time && time <= kf1.time {
                let duration = kf1.time - kf0.time;
                if duration <= 0.0 {
                    return Some(kf0.value.clone());
                }

                let raw_t = (time - kf0.time) / duration;
                let t = match kf0.interpolation {
                    Interpolation::Linear => raw_t,
                    Interpolation::Smooth => smoothstep(raw_t),
                    Interpolation::Step => 0.0,
                };

                return Some(T::lerp(&kf0.value, &kf1.value, t));
            }
        }

        Some(last.value.clone())
    }

    pub fn duration(&self) -> f32 {
        if self.keyframes.is_empty() {
            return 0.0;
        }
        self.keyframes.last().unwrap().time - self.keyframes[0].time
    }

    pub fn is_empty(&self) -> bool {
        self.keyframes.is_empty()
    }
}

impl<T: Interpolatable + Clone> Default for Channel<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Trait for types that can be interpolated between keyframes
pub trait Interpolatable: Clone {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
}

impl Interpolatable for f32 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        a + (b - a) * t
    }
}

impl Interpolatable for Vec3 {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Vec3::lerp(*a, *b, t)
    }
}

impl Interpolatable for Quat {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self {
        Quat::slerp(*a, *b, t)
    }
}

/// Transform state produced by sampling an animation at a point in time
#[derive(Clone, Debug)]
pub struct AnimationState {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }
}

/// A complete animation clip with position, rotation, and scale channels
#[derive(Clone)]
pub struct Animation {
    pub name: String,
    pub position: Channel<Vec3>,
    pub rotation: Channel<Quat>,
    pub scale: Channel<Vec3>,
    pub looping: bool,
}

impl Animation {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            position: Channel::new(),
            rotation: Channel::new(),
            scale: Channel::new(),
            looping: false,
        }
    }

    pub fn duration(&self) -> f32 {
        self.position.duration()
            .max(self.rotation.duration())
            .max(self.scale.duration())
    }

    /// Sample the animation at a given time, returning the interpolated transform state.
    pub fn sample(&self, time: f32) -> AnimationState {
        let t = if self.looping && self.duration() > 0.0 {
            time % self.duration()
        } else {
            time
        };

        AnimationState {
            position: self.position.sample(t).unwrap_or(Vec3::ZERO),
            rotation: self.rotation.sample(t).unwrap_or(Quat::IDENTITY),
            scale: self.scale.sample(t).unwrap_or(Vec3::ONE),
        }
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self::new("default")
    }
}

/// Smoothstep function for ease-in/ease-out interpolation
#[inline]
fn smoothstep(t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

/// Create a simple orbit camera animation
pub fn create_orbit_animation(radius: f32, height: f32, duration: f32, steps: u32) -> Animation {
    let mut anim = Animation::new("orbit");
    anim.looping = true;

    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let time = t * duration;
        let angle = t * std::f32::consts::TAU;

        let x = angle.cos() * radius;
        let z = angle.sin() * radius;

        anim.position.add_keyframe(time, Vec3::new(x, height, z), Interpolation::Linear);
    }

    anim
}

/// Create a bobbing animation (up and down)
pub fn create_bob_animation(amplitude: f32, frequency: f32, duration: f32) -> Animation {
    let mut anim = Animation::new("bob");
    anim.looping = true;

    let steps = (duration * frequency * 4.0) as u32; // 4 keyframes per cycle
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let time = t * duration;
        let y = (t * frequency * std::f32::consts::TAU).sin() * amplitude;

        anim.position.add_keyframe(time, Vec3::new(0.0, y, 0.0), Interpolation::Smooth);
    }

    anim
}

/// Create a spin animation (rotation around Y axis)
pub fn create_spin_animation(duration: f32, revolutions: f32) -> Animation {
    let mut anim = Animation::new("spin");
    anim.looping = true;

    let steps = (revolutions * 8.0) as u32; // 8 keyframes per revolution
    for i in 0..=steps {
        let t = i as f32 / steps as f32;
        let time = t * duration;
        let angle = t * revolutions * std::f32::consts::TAU;

        anim.rotation.add_keyframe(
            time,
            Quat::from_rotation_y(angle),
            Interpolation::Linear,
        );
    }

    anim
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_single_keyframe() {
        let mut ch = Channel::new();
        ch.add_keyframe(0.0, 5.0_f32, Interpolation::Linear);
        assert_eq!(ch.sample(0.0), Some(5.0));
        assert_eq!(ch.sample(1.0), Some(5.0));
    }

    #[test]
    fn test_channel_linear_interpolation() {
        let mut ch = Channel::new();
        ch.add_keyframe(0.0, 0.0_f32, Interpolation::Linear);
        ch.add_keyframe(1.0, 10.0_f32, Interpolation::Linear);

        assert_eq!(ch.sample(0.0), Some(0.0));
        assert_eq!(ch.sample(1.0), Some(10.0));

        let mid = ch.sample(0.5).unwrap();
        assert!((mid - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_channel_smooth_interpolation() {
        let mut ch = Channel::new();
        ch.add_keyframe(0.0, 0.0_f32, Interpolation::Smooth);
        ch.add_keyframe(1.0, 10.0_f32, Interpolation::Smooth);

        // smoothstep(0.5) = 0.5, so midpoint should be 5.0
        let mid = ch.sample(0.5).unwrap();
        assert!((mid - 5.0).abs() < 0.01);

        // smoothstep(0.25) = 0.15625, so value should be 1.5625
        let quarter = ch.sample(0.25).unwrap();
        assert!((quarter - 1.5625).abs() < 0.01);
    }

    #[test]
    fn test_channel_step_interpolation() {
        let mut ch = Channel::new();
        ch.add_keyframe(0.0, 0.0_f32, Interpolation::Step);
        ch.add_keyframe(1.0, 10.0_f32, Interpolation::Step);

        // Step holds previous value until next keyframe
        assert_eq!(ch.sample(0.5), Some(0.0));
        assert_eq!(ch.sample(0.99), Some(0.0));
    }

    #[test]
    fn test_channel_clamp_before_and_after() {
        let mut ch = Channel::new();
        ch.add_keyframe(1.0, 5.0_f32, Interpolation::Linear);
        ch.add_keyframe(2.0, 10.0_f32, Interpolation::Linear);

        // Before first keyframe: clamp to first value
        assert_eq!(ch.sample(0.0), Some(5.0));
        // After last keyframe: clamp to last value
        assert_eq!(ch.sample(3.0), Some(10.0));
    }

    #[test]
    fn test_channel_sorted_insertion() {
        let mut ch = Channel::new();
        ch.add_keyframe(2.0, 20.0_f32, Interpolation::Linear);
        ch.add_keyframe(0.0, 0.0_f32, Interpolation::Linear);
        ch.add_keyframe(1.0, 10.0_f32, Interpolation::Linear);

        let mid = ch.sample(0.5).unwrap();
        assert!((mid - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_vec3_channel() {
        let mut ch = Channel::new();
        ch.add_keyframe(0.0, Vec3::ZERO, Interpolation::Linear);
        ch.add_keyframe(1.0, Vec3::new(10.0, 20.0, 30.0), Interpolation::Linear);

        let mid = ch.sample(0.5).unwrap();
        assert!((mid.x - 5.0).abs() < 0.01);
        assert!((mid.y - 10.0).abs() < 0.01);
        assert!((mid.z - 15.0).abs() < 0.01);
    }

    #[test]
    fn test_animation_duration() {
        let mut anim = Animation::new("test");
        anim.position.add_keyframe(0.0, Vec3::ZERO, Interpolation::Linear);
        anim.position.add_keyframe(5.0, Vec3::ONE, Interpolation::Linear);
        anim.rotation.add_keyframe(0.0, Quat::IDENTITY, Interpolation::Linear);
        anim.rotation.add_keyframe(3.0, Quat::from_rotation_y(1.0), Interpolation::Linear);

        assert_eq!(anim.duration(), 5.0);
    }

    #[test]
    fn test_animation_looping() {
        let mut anim = Animation::new("test");
        anim.looping = true;
        anim.position.add_keyframe(0.0, Vec3::ZERO, Interpolation::Linear);
        anim.position.add_keyframe(2.0, Vec3::new(10.0, 0.0, 0.0), Interpolation::Linear);

        // At t=1.0, should be at x=5.0
        let state = anim.sample(1.0);
        assert!((state.position.x - 5.0).abs() < 0.01);

        // At t=3.0 with looping (wraps to t=1.0), should be at x=5.0
        let state_looped = anim.sample(3.0);
        assert!((state_looped.position.x - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_orbit_animation() {
        let anim = create_orbit_animation(3.0, 1.0, 4.0, 16);
        assert!(anim.looping);
        assert!((anim.duration() - 4.0).abs() < 0.01);

        let state = anim.sample(0.0);
        assert!((state.position.x - 3.0).abs() < 0.1);
        assert!((state.position.y - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_smoothstep() {
        assert_eq!(smoothstep(0.0), 0.0);
        assert_eq!(smoothstep(1.0), 1.0);
        assert!((smoothstep(0.5) - 0.5).abs() < 0.001);
        // smoothstep should have zero derivative at endpoints
        assert!(smoothstep(0.01) < 0.01 * 1.5); // slower than linear at start
        assert!(smoothstep(0.99) > 1.0 - 0.01 * 1.5); // slower than linear at end
    }

    #[test]
    fn test_empty_channel() {
        let ch: Channel<f32> = Channel::new();
        assert!(ch.is_empty());
        assert_eq!(ch.sample(0.0), None);
        assert_eq!(ch.duration(), 0.0);
    }
}
