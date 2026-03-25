// Core engine modules
pub mod engine;
pub mod loader;

use anyhow::Result;
use crossterm::{
    cursor,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use engine::{
    animation::{Animation, Interpolation},
    render::{Camera, Material, Mesh, Renderer, Scene, primitives},
    terminal::{AsciiTerminal, ColorTerminal, Terminal},
};
use glam::{Mat4, Vec3};
use std::io::{self, Write};
use std::path::Path;
use std::time::{Duration, Instant};

fn print_banner() {
    println!(
        r#"

  _ __ __ _ ___ | |_ ___ _ __ _ __ ___
 | '__/ _` / __|| __/ _ \ '__| '_ ` _ \
 | | | (_| \__ \| ||  __/ |  | | | | | |
 |_|  \__,_|___/ \__\___|_|  |_| |_| |_|

      Controls: WASD/Arrows=orbit  +/-=zoom  Space=pause
                1-5=demos  Tab=next  Q=quit
"#
    );
}

/// Demo scene definition
struct DemoScene {
    name: &'static str,
    mesh_scale: f32,
    rotation_speed: f32,
    use_gold: bool,
}

const DEMOS: &[DemoScene] = &[
    DemoScene {
        name: "Triforce",
        mesh_scale: 0.6,
        rotation_speed: 0.5,
        use_gold: true,
    },
    DemoScene {
        name: "Suzanne",
        mesh_scale: 1.0,
        rotation_speed: 0.4,
        use_gold: false,
    },
    DemoScene {
        name: "Torus",
        mesh_scale: 1.0,
        rotation_speed: 0.6,
        use_gold: true,
    },
    DemoScene {
        name: "Spaceship",
        mesh_scale: 0.8,
        rotation_speed: 0.3,
        use_gold: false,
    },
    DemoScene {
        name: "Goblet",
        mesh_scale: 0.9,
        rotation_speed: 0.5,
        use_gold: true,
    },
];

/// Build a scripted camera animation for each demo
fn build_camera_animation(demo_index: usize) -> Animation {
    let mut anim = Animation::new("camera");
    anim.looping = true;

    match demo_index {
        0 => {
            // Triforce: sweeping orbit, rising and falling
            let dur = 12.0;
            let steps = 24;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let time = t * dur;
                let angle = t * std::f32::consts::TAU;
                let r = 3.0 + 0.5 * (angle * 2.0).sin(); // breathing zoom
                let h = 0.8 * (angle * 1.5).sin(); // rise and fall
                let x = angle.cos() * r;
                let z = angle.sin() * r;
                anim.position.add_keyframe(time, Vec3::new(x, h, z), Interpolation::Smooth);
            }
        }
        1 => {
            // Suzanne: slow dramatic face reveal — start from side, sweep to front, hold, continue
            let kfs = [
                (0.0,  Vec3::new(3.5, 0.3, 0.0)),    // side profile
                (3.0,  Vec3::new(2.5, 0.5, 2.5)),     // rising quarter view
                (6.0,  Vec3::new(0.0, 0.2, 3.5)),     // front face — hold the smile
                (8.0,  Vec3::new(0.0, -0.2, 3.5)),    // slight dip, still front
                (11.0, Vec3::new(-2.5, 0.8, 2.5)),    // sweep past to other side, rising
                (14.0, Vec3::new(-3.5, 0.0, 0.0)),    // other side profile
                (16.0, Vec3::new(-2.5, -0.3, -2.5)),  // loop back
            ];
            for (time, pos) in kfs {
                anim.position.add_keyframe(time, pos, Interpolation::Smooth);
            }
        }
        2 => {
            // Torus: swooping orbit with dramatic height changes
            let dur = 10.0;
            let steps = 20;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let time = t * dur;
                let angle = t * std::f32::consts::TAU;
                let r = 3.5;
                let h = 1.5 * (angle * 2.0).sin(); // dramatic swoop up/down
                let x = angle.cos() * r;
                let z = angle.sin() * r;
                anim.position.add_keyframe(time, Vec3::new(x, h, z), Interpolation::Smooth);
            }
        }
        3 => {
            // Spaceship: flyby — zoom in close, sweep around, pull back
            let kfs = [
                (0.0,  Vec3::new(0.0, 1.0, 6.0)),     // distant front
                (2.5,  Vec3::new(1.5, 0.5, 3.0)),      // approach from right
                (4.5,  Vec3::new(2.0, 0.2, 0.0)),      // close side pass
                (6.5,  Vec3::new(1.5, -0.3, -2.5)),    // sweep behind and below
                (8.5,  Vec3::new(-1.5, 0.8, -2.5)),    // come around other side, above
                (10.5, Vec3::new(-2.0, 1.2, 0.0)),     // dramatic high side view
                (12.5, Vec3::new(-1.0, 0.5, 3.0)),     // return to front area
                (14.0, Vec3::new(0.0, 1.0, 6.0)),      // back to start
            ];
            for (time, pos) in kfs {
                anim.position.add_keyframe(time, pos, Interpolation::Smooth);
            }
        }
        4 => {
            // Goblet: slow elegant orbit, slight tilt
            let dur = 14.0;
            let steps = 28;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let time = t * dur;
                let angle = t * std::f32::consts::TAU;
                let r = 4.0 + 0.3 * (angle * 3.0).sin(); // gentle zoom pulse
                let h = 1.0 + 0.5 * (angle * 2.0).sin(); // gentle rise
                let x = angle.cos() * r;
                let z = angle.sin() * r;
                anim.position.add_keyframe(time, Vec3::new(x, h, z), Interpolation::Smooth);
            }
        }
        _ => {
            // Fallback: simple orbit
            let dur = 10.0;
            let steps = 16;
            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let time = t * dur;
                let angle = t * std::f32::consts::TAU;
                let x = angle.cos() * 3.0;
                let z = angle.sin() * 3.0;
                anim.position.add_keyframe(time, Vec3::new(x, 0.5, z), Interpolation::Linear);
            }
        }
    }

    anim
}

/// Camera controller with scripted animation + manual override
struct CameraController {
    // Manual orbit state
    yaw: f32,
    pitch: f32,
    distance: f32,
    auto_rotate_speed: f32,

    // Animation state
    camera_anim: Option<Animation>,
    anim_time: f32,
    manual_override: bool,
    paused: bool,
}

impl CameraController {
    fn new() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            distance: 3.0,
            auto_rotate_speed: 0.5,
            camera_anim: None,
            anim_time: 0.0,
            manual_override: false,
            paused: false,
        }
    }

    fn set_animation(&mut self, anim: Animation) {
        self.camera_anim = Some(anim);
        self.anim_time = 0.0;
        self.manual_override = false;
    }

    fn handle_key(&mut self, code: KeyCode) {
        let orbit = 0.1;
        let zoom = 0.3;

        match code {
            KeyCode::Char('w') | KeyCode::Up => {
                self.manual_override = true;
                self.pitch = (self.pitch + orbit).min(1.4);
            }
            KeyCode::Char('s') | KeyCode::Down => {
                self.manual_override = true;
                self.pitch = (self.pitch - orbit).max(-1.4);
            }
            KeyCode::Char('a') | KeyCode::Left => {
                self.manual_override = true;
                self.yaw -= orbit;
            }
            KeyCode::Char('d') | KeyCode::Right => {
                self.manual_override = true;
                self.yaw += orbit;
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.manual_override = true;
                self.distance = (self.distance - zoom).max(0.5);
            }
            KeyCode::Char('-') => {
                self.manual_override = true;
                self.distance = (self.distance + zoom).min(20.0);
            }
            KeyCode::Char(' ') => self.paused = !self.paused,
            _ => {}
        }
    }

    fn update(&mut self, dt: f32) {
        if self.paused {
            return;
        }

        if self.manual_override {
            self.yaw += self.auto_rotate_speed * dt;
        } else {
            self.anim_time += dt;
        }
    }

    fn camera_position(&self) -> Vec3 {
        if self.manual_override || self.camera_anim.is_none() {
            // Manual orbit
            let x = self.yaw.sin() * self.pitch.cos() * self.distance;
            let y = self.pitch.sin() * self.distance;
            let z = self.yaw.cos() * self.pitch.cos() * self.distance;
            Vec3::new(x, y, z)
        } else {
            // Scripted animation
            let anim = self.camera_anim.as_ref().unwrap();
            anim.sample(self.anim_time).position
        }
    }


}

struct App {
    renderer: Renderer,
    camera: Camera,
    scene: Scene,
    terminal: Box<dyn Terminal>,
    aspect_multiplier: f32,
    camera_controller: CameraController,
    current_demo: usize,
    demo_mode: bool,
    mesh_scale: f32,
    rotation_speed: f32,
}

impl App {
    fn new(color_mode: bool) -> Result<Self> {
        let terminal: Box<dyn Terminal> = if color_mode {
            Box::new(ColorTerminal::new()?)
        } else {
            Box::new(AsciiTerminal::new()?)
        };

        let aspect_multiplier = if color_mode { 1.0 } else { 2.0 };

        Ok(Self {
            renderer: Renderer::new(),
            camera: Camera::new(),
            scene: Scene::new(),
            terminal,
            aspect_multiplier,
            camera_controller: CameraController::new(),
            current_demo: 0,
            demo_mode: true,
            mesh_scale: 0.6,
            rotation_speed: 0.5,
        })
    }

    fn load_obj_file(&mut self, path: &Path, max_triangles: usize, texture_path: Option<&Path>) -> Result<()> {
        let geometry = loader::load_obj_auto_reduce(path, max_triangles)?;
        let material = if let Some(tex_path) = texture_path {
            Material::from_image(tex_path)?
        } else {
            Material::new()
        };
        let mesh = Mesh::new(geometry, material);

        self.scene.meshes.clear();
        self.scene.add_mesh(mesh);

        Ok(())
    }

    fn load_demo(&mut self, index: usize) {
        let demo = &DEMOS[index];
        self.current_demo = index;
        self.mesh_scale = demo.mesh_scale;
        self.rotation_speed = demo.rotation_speed;

        // Set up scripted camera
        let anim = build_camera_animation(index);
        self.camera_controller.set_animation(anim);

        self.scene.meshes.clear();

        // Try to load bundled model, fall back to procedural
        let model_path = format!("models/{}.obj", demo.name.to_lowercase());
        let geometry = if Path::new(&model_path).exists() {
            match loader::load_obj_auto_reduce(Path::new(&model_path), 2000) {
                Ok(g) => g,
                Err(_) => self.fallback_geometry(index),
            }
        } else {
            self.fallback_geometry(index)
        };

        let material = if demo.use_gold {
            Material::gold()
        } else {
            Material::new()
        };

        let mut mesh = Mesh::new(geometry, material);

        // Suzanne: load morph target for smile animation
        if index == 1 {
            let neutral_path = Path::new("models/suzanne_neutral.obj");
            let smile_path = Path::new("models/suzanne_smile.obj");
            if neutral_path.exists() && smile_path.exists() {
                if let (Ok(neutral), Ok(smile)) = (
                    loader::load_obj_auto_reduce(neutral_path, 2000),
                    loader::load_obj_auto_reduce(smile_path, 2000),
                ) {
                    if neutral.vertices.len() == smile.vertices.len() {
                        mesh.geometry = neutral;
                        mesh.set_morph_target(smile);
                    }
                }
            }
        }

        self.scene.add_mesh(mesh);
    }

    fn fallback_geometry(&self, index: usize) -> engine::render::Geometry {
        match index {
            0 => primitives::create_triforce(),
            1 => primitives::create_goblin(),
            2 => primitives::create_cube(),
            3 => primitives::create_pyramid(),
            4 => primitives::create_cube(),
            _ => primitives::create_triforce(),
        }
    }

    fn next_demo(&mut self) {
        let next = (self.current_demo + 1) % DEMOS.len();
        self.load_demo(next);
    }

    fn handle_input(&mut self) -> Result<bool> {
        while crossterm::event::poll(Duration::from_millis(0))? {
            match crossterm::event::read()? {
                Event::Key(KeyEvent { code: KeyCode::Char('q'), .. }) => return Ok(true),
                Event::Key(KeyEvent { code: KeyCode::Char('c'), modifiers, .. })
                    if modifiers.contains(KeyModifiers::CONTROL) => return Ok(true),
                Event::Key(KeyEvent { code: KeyCode::Tab, .. }) if self.demo_mode => {
                    self.next_demo();
                }
                Event::Key(KeyEvent { code: KeyCode::Char(c), .. })
                    if self.demo_mode && ('1'..='5').contains(&c) =>
                {
                    let idx = (c as usize) - ('1' as usize);
                    if idx < DEMOS.len() {
                        self.load_demo(idx);
                    }
                }
                Event::Key(KeyEvent { code, .. }) => {
                    self.camera_controller.handle_key(code);
                }
                _ => {}
            }
        }
        Ok(false)
    }

    fn update(&mut self, dt: f32, time: f32) {
        let (width, height) = self.terminal.size();
        let aspect = width as f32 / (height as f32 * self.aspect_multiplier);

        self.camera_controller.update(dt);
        let cam_pos = self.camera_controller.camera_position();

        self.camera.projection = Mat4::perspective_rh(
            60.0_f32.to_radians(),
            aspect,
            0.1,
            1000.0,
        );

        self.camera.view = Mat4::look_at_rh(
            cam_pos,
            Vec3::ZERO,
            Vec3::Y,
        );

        if let Some(mesh) = self.scene.meshes.first_mut() {
            let angle = time * self.rotation_speed;
            mesh.matrix = Mat4::from_rotation_y(angle) * Mat4::from_scale(Vec3::splat(self.mesh_scale));

            // Suzanne smile animation — synced to camera choreography
            if self.current_demo == 1 && mesh.morph_target.is_some() {
                // Camera path: side(0s) → front(6s) → hold(8s) → away(16s), loops at 16s
                let anim_t = self.camera_controller.anim_time % 16.0;
                let smile_weight = if anim_t < 3.0 {
                    0.0 // approaching, neutral
                } else if anim_t < 6.0 {
                    // building smile as camera faces her
                    (anim_t - 3.0) / 3.0
                } else if anim_t < 10.0 {
                    1.0 // full smile while camera is in front
                } else if anim_t < 13.0 {
                    // fade smile as camera leaves
                    1.0 - (anim_t - 10.0) / 3.0
                } else {
                    0.0 // neutral
                };
                // Smooth the transition
                let smooth = smile_weight * smile_weight * (3.0 - 2.0 * smile_weight);
                mesh.apply_morph(smooth);
            }
        }
    }

    fn render(&mut self) -> Result<()> {
        let (width, height) = self.terminal.size();

        self.renderer.resize(width, height);
        self.renderer.clear([0.0, 0.0, 0.0, 1.0]);

        if !self.scene.meshes.is_empty() {
            self.renderer.render(&self.camera, &self.scene);
        }

        self.terminal.present(&self.renderer.color_buffer)?;

        Ok(())
    }

    fn scene_name(&self) -> &str {
        if self.demo_mode {
            DEMOS[self.current_demo].name
        } else {
            "Custom"
        }
    }
}

fn main() -> Result<()> {
    print_banner();

    // Parse arguments
    let args: Vec<String> = std::env::args().collect();
    let color_mode = args.contains(&"--color".to_string());

    // Setup terminal
    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        cursor::Hide,
    )?;
    terminal::enable_raw_mode()?;

    // Create app
    let mut app = App::new(color_mode)?;

    // Parse --texture flag
    let texture_path = args.windows(2)
        .find(|w| w[0] == "--texture")
        .map(|w| w[1].clone());

    // Load model or demo mode
    let obj_arg = args.iter().skip(1).find(|a| !a.starts_with("--") && {
        let idx = args.iter().position(|x| x == *a).unwrap_or(0);
        idx < 2 || args[idx - 1] != "--texture"
    });

    if let Some(obj_file) = obj_arg {
        let obj_path = Path::new(obj_file);
        let tex_path = texture_path.as_deref().map(Path::new);
        match app.load_obj_file(obj_path, 2000, tex_path) {
            Ok(_) => {
                app.demo_mode = false;
                app.camera_controller.manual_override = true;
            }
            Err(e) => {
                eprintln!("Failed to load {:?}: {}", obj_path, e);
                app.load_demo(0);
            }
        }
    } else {
        app.load_demo(0);
    }

    // Clear screen once
    execute!(stdout, Clear(ClearType::All))?;

    // Main loop
    let start_time = Instant::now();
    let mut last_frame = Instant::now();
    let mut frame_count = 0u64;
    let mut last_fps_update = Instant::now();
    let mut fps = 0.0;

    let result = loop {
        let now = Instant::now();
        let dt = (now - last_frame).as_secs_f32();
        last_frame = now;
        let elapsed = start_time.elapsed().as_secs_f32();

        // Update FPS counter
        frame_count += 1;
        let fps_elapsed = last_fps_update.elapsed();
        if fps_elapsed >= Duration::from_secs(1) {
            fps = frame_count as f32 / fps_elapsed.as_secs_f32();
            frame_count = 0;
            last_fps_update = Instant::now();
        }

        // Update window title
        let (width, height) = app.terminal.size();
        let status = if app.camera_controller.paused {
            " [paused]"
        } else if app.camera_controller.manual_override {
            " [manual]"
        } else {
            ""
        };
        let scene_name = app.scene_name();
        print!(
            "\x1b]0;rasterm: {} | {} x {} @ {:.0} fps{}\x07",
            scene_name, width, height, fps, status
        );
        let _ = io::stdout().flush();

        // Handle input
        match app.handle_input() {
            Ok(true) => break Ok(()),
            Err(e) => break Err(e),
            _ => {}
        }

        // Update and render
        app.update(dt, elapsed);
        if let Err(e) = app.render() {
            break Err(e);
        }

        // Frame limiting (~60fps target)
        std::thread::sleep(Duration::from_millis(16));
    };

    // Cleanup
    let _ = terminal::disable_raw_mode();
    let _ = execute!(
        stdout,
        cursor::Show,
        LeaveAlternateScreen,
        Clear(ClearType::All),
    );

    println!();

    result
}
