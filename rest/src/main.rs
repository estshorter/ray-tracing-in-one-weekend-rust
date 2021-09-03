mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod cube;
mod hittable;
mod hittable_list;
mod material;
mod moving_sphere;
mod onb;
mod pdf;
mod perlin;
mod ray;
mod rotate;
mod rtweekend;
mod sphere;
mod texture;
mod translate;
mod vec3;

use aarect::*;
use camera::Camera;
use color::*;
use cube::*;
use hittable::*;
use hittable_list::HittableList;
use material::*;
use pdf::*;
use rand::Rng;
use ray::Ray;
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator};
use rayon::prelude::*;
use rotate::*;
use rtweekend::*;
use sphere::Sphere;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Mutex;
use texture::*;
use translate::*;
use vec3::*;

fn cornell_box() -> (Box<dyn Hittable>, Box<dyn Hittable>) {
    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new(15.0, 15.0, 15.0));
    let aluminum = Metal::new(Vec3::new(0.8, 0.85, 0.88), 0.0);
    let glass = Dielectric::new(1.5);
    let glass_sphere = Sphere::new(Vec3::new(190.0, 90.0, 190.0), 90.0, glass);
    let light_shape = AARect::new(Plane::ZX, 227.0, 332.0, 213.0, 343.0, 554.0, light);

    let mut world = HittableList::new();
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(FlipNormals::new(light_shape.clone()));
    world.push(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    world.push(AARect::new(
        Plane::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    ));
    world.push(AARect::new(
        Plane::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    ));
    world.push(Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                aluminum,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    // world.push(Translate::new(
    //     Rotate::new(
    //         Axis::Y,
    //         Cube::new(
    //             Vec3::new(0.0, 0.0, 0.0),
    //             Vec3::new(165.0, 165.0, 165.0),
    //             white,
    //         ),
    //         -18.0,
    //     ),
    //     Vec3::new(130.0, 0.0, 65.0),
    // ));
    world.push(glass_sphere.clone());

    let mut lights = HittableList::new();
    lights.push(light_shape);
    lights.push(glass_sphere);

    (Box::new(world), Box::new(lights))
}

fn ray_color(
    r: &Ray,
    background: &Color,
    world: &Box<dyn Hittable>,
    lights: &Box<dyn Hittable>,
    depth: i32,
) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.emitted(&r, &rec);

        if let Some(srec) = rec.material.scatter(r, &rec) {
            match srec {
                ScatterRecord::Scatter { pdf, attenuation } => {
                    let light_pdf = PDF::hittable(lights, &rec.p);
                    let mixture_pdf = PDF::mixture(&light_pdf, &pdf);
                    let dir = mixture_pdf.generate();
                    let scattered = Ray::new(rec.p, dir, r.time());
                    let pdf = mixture_pdf.value(&scattered.direction());
                    let scattering_pdf = rec.material.scattering_pdf(&r, &rec, &scattered);
                    return emitted
                        + &ray_color(&scattered, background, world, lights, depth - 1)
                            * &attenuation
                            * (scattering_pdf / pdf);
                }
                ScatterRecord::Specular {
                    specular_ray,
                    attenuation,
                } => {
                    return &attenuation
                        * &ray_color(&specular_ray, background, world, lights, depth - 1)
                }
            }
        }
        return emitted;
    }
    background.clone()
}

fn main() {
    const MAX_DEPTH: i32 = 50;

    //world
    let (world, lights) = cornell_box();
    let aperture: f64 = 0.0;

    let aspect_ratio = 1.0;
    let image_width = 600;
    let samples_per_pixel = 1000;
    let background = Color::default();
    let lookfrom = Point3::new(278., 278., -800.);
    let lookat = Point3::new(278., 278., 0.);
    let vfov = 40.0;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    let vup = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );
    let counter = Mutex::new(0);
    let mut result: Vec<Vec<(i32, i32, i32)>> = Vec::with_capacity(image_height as usize);
    (0..image_height as i32)
        .into_par_iter()
        .rev()
        .map(move |j| {
            let mut color: Vec<(i32, i32, i32)> = Vec::with_capacity(image_width as usize);
            let mut rng = rand::thread_rng();
            for i in 0..image_width {
                let mut pixel_color = Color::new(0., 0., 0.);
                for _ in 0..samples_per_pixel {
                    let u = (i as f64 + rng.gen::<f64>()) / (image_width as f64 - 1.0);
                    let v = (j as f64 + rng.gen::<f64>()) / (image_height as f64 - 1.0);
                    let r = cam.get_ray(u, v);
                    pixel_color += ray_color(&r, &background, &world, &lights, MAX_DEPTH);
                }
                color.push(get_color(pixel_color, samples_per_pixel));
            }
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
                eprint!("\rScanlines remaining: {} ", image_height - *num);
            }
            color
        })
        .collect_into_vec(&mut result);

    eprintln!("\nFile output start.");
    let mut file = BufWriter::with_capacity(
        (image_width * image_height * 11 + 17) as usize,
        File::create("image.ppm").expect("Unable to create file"),
    );
    let header = format!("P3\n{} {}\n255\n", image_width, image_height);
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
