use rand::Rng;

struct Particle {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
}

impl Particle {
    fn update(&mut self, dt: f64) {
        self.x += self.vx * dt;
        self.y += self.vy * dt;
    }
}

struct Simulation {
    particles: Vec<Particle>,
    width: f64,
    height: f64,
}

impl Simulation {
    fn new(count: usize, w: f64, h: f64) -> Self {
        let mut rng = rand::thread_rng();
        let particles = (0..count).map(|_| Particle {
            x: rng.gen_range(0.0..w),
            y: rng.gen_range(0.0..h),
            vx: rng.gen_range(-10.0..10.0),
            vy: rng.gen_range(-10.0..10.0),
        }).collect();
        Self { particles, width: w, height: h }
    }

    fn step(&mut self, dt: f64) {
        for p in &mut self.particles {
            p.update(dt);
            if p.x < 0.0 || p.x > self.width { p.vx *= -1.0; }
            if p.y < 0.0 || p.y > self.height { p.vy *= -1.0; }
        }
    }
}

fn main() {
    let mut sim = Simulation::new(100, 800.0, 600.0);
    for i in 0..10 {
        sim.step(0.1);
        println!("Step {}: Particle[0] pos ({:.2}, {:.2})", i, sim.particles[0].x, sim.particles[0].y);
    }
}