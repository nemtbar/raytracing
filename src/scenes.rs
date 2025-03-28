#![allow(dead_code)]
use image::{RgbImage, ImageReader};
use raytracing::{	
	WIDTH,
	textures::Texture,
	geometry::{Material, Reflection, Camera}, 
	Uniforms, 
	objects::{Object, abstract_object}, 
	vec3::Vec3,
    render::Picture
};

pub fn lalaland()-> Uniforms{
    let image: RgbImage = ImageReader::open("moon3.jpg").unwrap().decode().unwrap().into_rgb8();
    let img = Picture::new(image);
    let mat = Material::new(Reflection::Diffuse(), Texture::Img { img }, Vec3::default());
    let star_mat = Material{refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new1(1.) }, emmision: Vec3::new1(2.)};
    let mut stars: Vec<Object> = vec![];
    for _ in 0..100{
        let rand = Vec3::random();
        let pos = Vec3::new(rand.x*300., rand.y*100.+300., rand.z.abs()*100.);
        let star = Object::Sphere { pos, rad: 0.1, mat: star_mat.clone() };
        stars.push(star);
    }
    let mut input = Uniforms {
        sample_count: 200, 
        bounce_count: 5,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -20., 3.3),
            &Vec3::new(0., 0., 3.),
            45.,
            &Vec3::up(),
            0.
        ),
        objects: vec![
            Object::Sphere { pos: Vec3::new(0., 0., -120.), rad: 120., mat: Material::default() },
            Object::Sphere { pos: Vec3::new(100., 300., 50.), rad: 15., mat },
            Object::Sphere {pos: Vec3::new(70., 110., -60.), rad: 20., mat: Material {refl: Reflection::Diffuse(), tex:Texture::Solid { color: Vec3::new1(1.) }, emmision: Vec3::new1(10.)}}
            ],
        env_shader: Box::new(|v|{
            let col1 = Vec3::new(0.3843, 0.1294, 0.702);
            let col2 = Vec3::new(0.018, 0.0157, 0.2039);
            let value = v.dot(&Vec3::up()).max(0.);
            col1.lerp(&col2, value)
        } )
    };
    let b = Object::BoundBox { min: Vec3::new(-300., 200., 0.), max: Vec3::new(300., 400., 100.), inside: stars };
    input.objects.push(b);
    input
}

pub fn cylinder_test()->Uniforms{
    let mut input = Uniforms {
        sample_count: 500,
        bounce_count: 5,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(
            &Vec3::new(0., -5., 3.3),
            &Vec3::new(0., 0., 3.),
            90.,
            &Vec3::up(),
            0.
        ),
        objects: vec![
            Object::Plane { pos: Vec3::default(), normal: Vec3::up(), mat: Material::default() },
            Object::Sphere { pos: Vec3::new(6., -3., 10.), rad: 1., mat: Material{refl: Reflection::Diffuse(), tex:Texture::Solid { color: Vec3::new1(1.) }, emmision: Vec3::new(16., 16., 5.)} }
        ],
        env_shader: Box::new(|v: &Vec3|{
            let col1 = Vec3::new(0.1255, 0., 0.1608);
            let col2 = Vec3::new(0.0275, 0.0078, 0.098);
            col1.lerp(&col2, v.dot(&Vec3::up()).max(0.))
        }),
    };
    //let mat = Material {refl: Reflection::Glass { reflective: 1.5 }, tex: Texture::Solid { color: Vec3::new1(1.) }, emmision: Vec3::new1(0.)};
    let cyl = abstract_object::new_cylinder(&Vec3::new(3., 0.,  1.5), &Vec3::new(0., 0., 3.), 32, 2.,true, Material::default());
    let cyl_bound = Object::BoundBox { min: Vec3::new(0.99, -1.99, 0.), max: Vec3::new(5., 2., 3.), inside: cyl };
    input.objects.push(cyl_bound);
    let pyr = abstract_object::new_pyramid(&Vec3::new(0., 0., 2.), 3., &(Vec3::up()*4.), 45., Material::default());
    let pyr_bound = Object::BoundBox { min: Vec3::new(-2., -2.3, 0.), max: Vec3::new(2., 2., 4.), inside: pyr };
    input.objects.push(pyr_bound);
    input
}

pub fn scene3()->Uniforms{
    let mut boxes: Vec<Object> = vec![];
    let mut _rand = rand::thread_rng();
    for x in (-2..=2).step_by(2){
        for y in (-2..=2).step_by(2){
            let height = 2.;
            let pos = Vec3::new((x+1) as f32, (y+1) as f32, height/2.);
            let mut box1 = abstract_object::new_box(&pos, &Vec3::new(0., 0., height), height, Material::default());
            boxes.append(&mut box1);
        }
    }
    //boxes.append(&mut abstract_object::new_box(&Vec3::new(0., 0., 1.), &Vec3::new(0., 0., 2.), 2., Material::default()));
    let boxes_bound = Object::BoundBox { min: Vec3::new1(-10.), max: Vec3::new1(10.), inside: boxes };
    Uniforms{
        sample_count: 100,
        bounce_count: 10,
        offset: WIDTH as f32/1000.,
        cam: Camera::new(&Vec3::new(0., -5., 4.), &Vec3::default(), 90., &Vec3::up(), 0.1),
        objects: vec![
            Object::Plane { pos: Vec3::default(), normal: Vec3::up(), mat: Material{refl: Reflection::Diffuse(), tex: Texture::Solid { color: Vec3::new(0.9, 0.9, 0.) }, emmision: Vec3::new1(0.)} },
            boxes_bound
        ],
        env_shader: Uniforms::get_env_shader()
    }
} 

pub fn fast()->Uniforms{
    Uniforms {
        sample_count: 20,
        bounce_count: 5,
        offset: 0.,
        cam: Camera::new(&Vec3::default(), &Vec3::back(), 45., &Vec3::up(), 0.),
        objects: vec![
            Object::Plane { pos: Vec3::up()*-1., normal: Vec3::up(), mat: Material::default() },
        ],
        env_shader: Uniforms::get_env_shader()
    }
}

