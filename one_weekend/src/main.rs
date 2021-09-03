mod camera;
mod color;
mod hittable;
mod hittable_list;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use color::*;
use hittable::Hittable;
use hittable_list::HittableList;
use material::*;
use rand::Rng;
use ray::Ray;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::prelude::*;
use rtweekend::*;
use sphere::Sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use vec3::*;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return &attenuation * &ray_color(&scattered, world, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.77, 1.0)
}

fn random_scene() -> HittableList {
    let mut world = HittableList::new();
    let ground_material = Lambertian::new(Color::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));
    let mut rng = rand::thread_rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = &Color::random() * &Color::random();
                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_minmax(0.5, 1.0);
                    let fuzz = random_double_range(0.0..0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.add(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.add(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::new(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}

fn main() {
    const ASPECT_RATIO: f64 = 3. / 2.;
    const IMAGE_WIDTH: i32 = 1200;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 1000;
    const MAX_DEPTH: i32 = 50;

    //world
    let world = random_scene();

    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;
    let aperture: f64 = 0.1;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );
    let counter = Mutex::new(0);
    let mut result: Vec<Vec<(i32, i32, i32)>> = Vec::with_capacity(IMAGE_HEIGHT as usize);
    (0..IMAGE_HEIGHT as i32)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let mut color: Vec<(i32, i32, i32)> = Vec::with_capacity(IMAGE_WIDTH as usize);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_double()) / (IMAGE_WIDTH as f64 - 1.0);
                    let v = (j as f64 + random_double()) / (IMAGE_HEIGHT as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &world, MAX_DEPTH);
                }
                color.push(get_color(pixel_color, SAMPLES_PER_PIXEL));
            }
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
                eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - *num);
            }
            color
        })
        .collect_into_vec(&mut result);

    eprintln!("\nFile output start.");
    let mut file = BufWriter::with_capacity(
        (IMAGE_WIDTH * IMAGE_HEIGHT * 11 + 17) as usize,
        File::create("image.ppm").expect("Unable to create file"),
    );
    let header = format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    file.write_all(header.as_bytes())
        .expect("Unable to write data");
    result.iter().for_each(|row| {
        row.iter().for_each(|col| {
            let data = format!("{} {} {}\n", col.0, col.1, col.2,);
            file.write_all(data.as_bytes())
                .expect("Unable to write data");
        });
    });
    file.flush().unwrap();
    eprintln!("Done.");
}
