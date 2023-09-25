
mod primitives;
mod scene;
mod renderer;

use crate::primitives::{
    Vec2i,
    Vec3,
    Rect,
};
use crate::scene::{
    Camera,
    Scene
};

use crate::renderer::{
    Tile,
    RenderProperties,
};

use rand::SeedableRng;
use rand::rngs::SmallRng;


fn main() {
    // image
    let aspect_ratio = 3.0 / 2.0;
    let image = Vec2i {
        x: 400,
        y: (400.0 / aspect_ratio) as i32
    };

    let render_config = RenderProperties {
        samples: 10,
        bounces: 50
    };

    // random generator
    let mut small_rng = SmallRng::seed_from_u64(0);

    // Scene (now includes camera)
    let scene = Scene {
        camera: Camera::new(
            Vec3::new(13.0, 2.0, 3.0), // lookfrom
            Vec3::zero(), // lookat
            Vec3::new(0.0, 1.0, 0.0), // vup
            20.0,
            aspect_ratio, 
            0.1, // aperture
            10.0, // dist_to_focus
        ),
        world: Scene::random_world(&mut small_rng)
    };
    
    // render
    // The render loop should now be a job submission mechanism
    // Iterate lines, submitting them as tasks to the thread.
	println!("P3\n{} {}\n255", image.x, image.y);
    for row in (0..image.y).rev() {
        let bounds = Rect{ // render boundary (a horizontal slice)
            x: 0,
            y: row,
            w: image.x,
            h: 1
        };
        let tile = Tile::render_line(bounds, row, image, &scene, &render_config, &mut small_rng);
        eprintln!("Printing scanline #{}", row);
        for pixel in tile.pixels {
            println!("{}", pixel.print_ppm(render_config.samples))
        }
    }
    // TODO: Dispatcher shutdown mechanism. Right now, we might technically be leaking threads.
    eprintln!("Done!");
}
