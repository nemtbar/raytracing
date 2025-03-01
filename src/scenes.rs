use crate::*;

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
        sample_count: 10, 
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