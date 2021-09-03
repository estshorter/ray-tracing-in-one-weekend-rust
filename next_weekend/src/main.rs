mod aabb;
mod aarect;
mod bvh;
mod camera;
mod color;
mod cube;
mod hittable;
mod hittable_list;
mod material;
mod medium;
mod moving_sphere;
mod perlin;
mod ray;
mod rotate;
mod rtweekend;
mod sphere;
mod texture;
mod translate;
mod vec3;

use aarect::*;
use bvh::*;
use camera::Camera;
use color::*;
use cube::*;
use hittable::*;
use hittable_list::HittableList;
use image;
use material::*;
use medium::*;
use moving_sphere::MovingSphere;
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
use texture::CheckerTexture;
use texture::*;
use translate::*;
use vec3::*;

fn ray_color(r: &Ray, background: &Color, world: &dyn Hittable, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(rec) = world.hit(r, 0.001, INFINITY) {
        let emitted = rec.material.emitted(rec.u, rec.v, &rec.p);

        if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
            return emitted + &attenuation * &ray_color(&scattered, background, world, depth - 1);
        }
        return emitted;
    }
    background.clone()
}

#[allow(dead_code)]
fn random_scene() -> HittableList {
    let mut rng = rand::thread_rng();

    let mut world = HittableList::new();
    let checker = CheckerTexture::new(
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let ground_material = Lambertian::new(checker);
    world.push(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (&center - &Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = &Color::random() * &Color::random();
                    let center2 = &center + &Vec3::new(0.0, rng.gen_range(0.0..0.5), 0.0);
                    world.push(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Lambertian::from_color(albedo),
                    ));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_minmax(0.5, 1.0);
                    let fuzz: f64 = rng.gen_range(0.0..0.5);
                    world.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // glass
                    world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                }
            }
        }
    }

    world.push(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Dielectric::new(1.5),
    ));
    world.push(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Lambertian::from_color(Color::new(0.4, 0.2, 0.1)),
    ));
    world.push(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Metal::new(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    world
}
#[allow(dead_code)]
fn two_spheres() -> HittableList {
    let checker = CheckerTexture::new(
        SolidColor::new(0.2, 0.3, 0.1),
        SolidColor::new(0.9, 0.9, 0.9),
    );
    let mut world = HittableList::new();
    world.push(Sphere::new(
        Vec3::new(0.0, -10.0, 0.0),
        10.0,
        Lambertian::new(checker.clone()),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 10.0, 0.0),
        10.0,
        Lambertian::new(checker),
    ));
    world
}

#[allow(dead_code)]
fn two_perlin_spheres() -> HittableList {
    let noise = NoiseTexture::new(4.0);
    let mut world = HittableList::new();
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(noise.clone()),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(noise),
    ));
    world
}

#[allow(dead_code)]
fn earth() -> HittableList {
    let image = image::open("earthmap.png")
        .expect("image not found")
        .to_rgb8();
    let (nx, ny) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, nx, ny);
    let earth = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));
    let mut world = HittableList::new();
    world.push(earth);
    world
}

#[allow(dead_code)]
fn simple_light() -> HittableList {
    let noise = NoiseTexture::new(4.0);
    let mut world = HittableList::new();
    world.push(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Lambertian::new(noise.clone()),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 2.0, 0.0),
        2.0,
        Lambertian::new(noise),
    ));
    world.push(Sphere::new(
        Vec3::new(0.0, 7.0, 0.0),
        2.0,
        DiffuseLight::new(SolidColor::new(4.0, 4.0, 4.0)),
    ));
    world.push(AARect::new(
        Plane::XY,
        3.0,
        5.0,
        1.0,
        3.0,
        -2.0,
        DiffuseLight::new(SolidColor::new(4.0, 4.0, 4.0)),
    ));
    world
}

#[allow(dead_code)]
fn cornell_box() -> HittableList {
    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new(15.0, 15.0, 15.0));
    let mut world = HittableList::new();
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(
        Plane::ZX,
        227.0,
        332.0,
        213.0,
        343.0,
        554.0,
        light,
    ));
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
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    ));
    world.push(Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    ));
    world
}

#[allow(dead_code)]
fn cornell_smoke() -> HittableList {
    let red = Lambertian::new(SolidColor::new(0.65, 0.05, 0.05));
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let green = Lambertian::new(SolidColor::new(0.12, 0.45, 0.15));
    let light = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    let mut world = HittableList::default();
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(
        Plane::ZX,
        127.0,
        432.0,
        113.0,
        443.0,
        554.0,
        light,
    ));
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
    let box1 = Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 165.0, 165.0),
                white.clone(),
            ),
            -18.0,
        ),
        Vec3::new(130.0, 0.0, 65.0),
    );
    let box2 = Translate::new(
        Rotate::new(
            Axis::Y,
            Cube::new(
                Vec3::new(0.0, 0.0, 0.0),
                Vec3::new(165.0, 330.0, 165.0),
                white,
            ),
            15.0,
        ),
        Vec3::new(265.0, 0.0, 295.0),
    );
    world.push(ConstantMedium::new(
        box1,
        0.01,
        SolidColor::new(1.0, 1.0, 1.0),
    ));
    world.push(ConstantMedium::new(
        box2,
        0.01,
        SolidColor::new(0.0, 0.0, 0.0),
    ));
    world
}

#[allow(dead_code)]
fn final_scene() -> HittableList {
    let mut rng = rand::thread_rng();
    let white = Lambertian::new(SolidColor::new(0.73, 0.73, 0.73));
    let ground = Lambertian::new(SolidColor::new(0.48, 0.83, 0.53));
    let mut world = HittableList::default();
    let mut box_list1: Vec<Box<dyn Hittable>> = Vec::new();
    let nb = 20;
    for i in 0..nb {
        for j in 0..20 {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = 100.0 * (rng.gen::<f64>() + 0.01);
            let z1 = z0 + w;
            box_list1.push(Box::new(Cube::new(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }
    world.push(BVH::new(box_list1, 0.0, 1.0));
    let light = DiffuseLight::new(SolidColor::new(7.0, 7.0, 7.0));
    world.push(AARect::new(
        Plane::ZX,
        147.0,
        412.0,
        123.0,
        423.0,
        554.0,
        light,
    ));
    // let center = Vec3::new(400.0, 400.0, 200.0);
    // world.push(MovingSphere::new(
    //     center,
    //     center + Vec3::new(30.0, 0.0, 0.0),
    //     0.0,
    //     1.0,
    //     50.0,
    //     Lambertian::new(SolidColor::new(0.7, 0.3, 0.1)),
    // ));
    // world.push(Sphere::new(
    //     Vec3::new(260.0, 150.0, 45.0),
    //     50.0,
    //     Dielectric::new(1.5),
    // ));
    // world.push(Sphere::new(
    //     Vec3::new(0.0, 150.0, 145.0),
    //     50.0,
    //     Metal::new(Vec3::new(0.8, 0.8, 0.9), 10.0),
    // ));
    // let boundary = Sphere::new(Vec3::new(360.0, 150.0, 145.0), 70.0, Dielectric::new(1.5));
    // world.push(boundary.clone());
    // world.push(ConstantMedium::new(
    //     boundary,
    //     0.2,
    //     SolidColor::new(0.2, 0.4, 0.9),
    // ));
    // let boundary = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5000.0, Dielectric::new(1.5));
    // world.push(ConstantMedium::new(
    //     boundary,
    //     0.0001,
    //     SolidColor::new(1.0, 1.0, 1.0),
    // ));
    // let image = image::open("earthmap.png")
    //     .expect("image not found")
    //     .to_rgb8();
    // let (nx, ny) = image.dimensions();
    // let data = image.into_raw();
    // let texture = ImageTexture::new(data, nx, ny);
    // world.push(Sphere::new(
    //     Vec3::new(400.0, 200.0, 400.0),
    //     100.0,
    //     Lambertian::new(texture),
    // ));
    world.push(Sphere::new(
        Vec3::new(220.0, 280.0, 300.0),
        80.0,
        Lambertian::new(NoiseTexture::new(0.1)),
    ));
    // let mut box_list2: Vec<Box<dyn Hittable>> = Vec::new();
    // let ns = 1000;
    // for _ in 0..ns {
    //     box_list2.push(Box::new(Sphere::new(
    //         Vec3::new(
    //             165.0 * rng.gen::<f64>(),
    //             165.0 * rng.gen::<f64>(),
    //             165.0 * rng.gen::<f64>(),
    //         ),
    //         10.0,
    //         white.clone(),
    //     )));
    // }
    // world.push(Translate::new(
    //     Rotate::new(Axis::Y, BVH::new(box_list2, 0.0, 0.1), 15.0),
    //     Vec3::new(-100.0, 270.0, 395.0),
    // ));
    world
}

fn main() {
    let mut aspect_ratio: f64 = 16. / 9.;
    let mut image_width: i32 = 400;
    const MAX_DEPTH: i32 = 50;

    let mut samples_per_pixel: i32 = 100;

    //world
    let world: HittableList;
    let mut lookfrom = Point3::new(13.0, 2.0, 3.0);
    let mut lookat = Point3::new(0.0, 0.0, 0.0);
    let mut vfov = 20.0;
    let mut aperture: f64 = 0.0;
    let mut background = Color::new(0.70, 0.80, 1.00);

    match 10 {
        1 => {
            world = random_scene();
            aperture = 0.1;
        }
        2 => {
            world = two_spheres();
        }
        3 => {
            world = two_perlin_spheres();
        }
        4 => {
            world = earth();
        }
        5 => {
            world = simple_light();
            samples_per_pixel = 400;
            background = Color::default();
            lookfrom = Point3::new(26., 3., 6.);
            lookat = Point3::new(0., 2., 0.);
        }
        6 => {
            world = cornell_box();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.0;
        }
        7 => {
            world = cornell_smoke();
            aspect_ratio = 1.0;
            image_width = 600;
            samples_per_pixel = 200;
            background = Color::default();
            lookfrom = Point3::new(278., 278., -800.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.0;
        }
        _ => {
            world = final_scene();
            aspect_ratio = 1.0;
            image_width = 800;
            samples_per_pixel = 1000;
            background = Color::default();
            lookfrom = Point3::new(478., 278., -600.);
            lookat = Point3::new(278., 278., 0.);
            vfov = 40.0;
        }
    }
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
                    pixel_color += ray_color(&r, &background, &world, MAX_DEPTH);
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
