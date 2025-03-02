use crate::{Vec3, Material, Ray, HitInfo, Texture, scatter};


pub enum Object {
    Sphere {pos: Vec3, rad: f32, mat: Material},
    Plane {pos: Vec3, normal: Vec3, mat: Material},
    BoundBox {min: Vec3, max: Vec3, inside: Vec<Object>},
    Quad {pos: Vec3, delta_x: Vec3, delta_y: Vec3, kind: QuadType, n: Vec3, w: Vec3, mat: Material}
}

pub enum QuadType{
    Rect(),
    Triangle(),
    Disk(),
}

impl QuadType{
    fn get_fn(&self)->impl Fn(f32, f32)->bool{
        match self {
            Self::Triangle() => |a: f32, b: f32| a > 0. && b > 0. && a+b < 1.,
            Self::Rect() => |a: f32, b: f32| 0. < a && a < 1. && 0. < b && b < 1.,
            Self::Disk() => |a: f32, b: f32| a*a+b*b < 1.
        }
    }
}

impl Object {

    fn overlap(min1: f32, max1: f32, min2: f32, max2: f32) -> bool {
        let over_min = min1.max(min2);
        let over_max = max1.min(max2);
        over_min < over_max
    }

    fn calc_quadrilet(p: &Vec3, u: &Vec3, v: &Vec3, w: &Vec3) -> (f32, f32){
        let alpha = w.dot(&p.cross(v));
        let beta = w.dot(&u.cross(&p));
        (alpha, beta)
    }

    fn intersect(&self, ray: &Ray) -> Option<HitInfo>{
        assert!(ray.dir.is_normalized());
        match self {
            //https://kylehalladay.com/blog/tutorial/math/2013/12/24/Ray-Sphere-Intersection.html
            Self::Sphere {pos, rad, mat} => {
                let camera_self = pos - &ray.start;
                let project_len = camera_self.dot(&ray.dir);
                if project_len < 0.0{
                    None
                } else {
                    let closest = camera_self.length_squared()-project_len*project_len;
                    let rad2 = rad * rad;
                    if closest > rad2{
                        None
                    } else {
                        let t1c = (rad2 - closest).sqrt();
                        let inters = project_len - t1c;
                        if inters <= 0. {
                            return None;
                        }
                        let normal = (&ray.start + &ray.dir * inters - pos).normalize();
                        let hitp = &ray.start + &ray.dir * inters;
                        let (u,v) = Texture::sphere_uv_coord(pos, &hitp);
                        Some(HitInfo{p: hitp, normal, material: mat.clone(), u, v})
                    }
                }
            }
            Self::Plane {pos, normal, mat} => {
                //https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-plane-and-ray-disk-intersection.html
                //pos-hitP dot normal = 0
                let mut n = normal.clone();
                if n.dot(&ray.dir) > 0. {
                    n = n * -1.;
                }
                let denom = n.dot(&ray.dir);
                let t = (pos - &ray.start).dot(&n) / denom;
                if t > 0.{
                    let hit = HitInfo{p: &ray.start + &ray.dir * t, normal: n.clone(), material: mat.clone(), u: 0., v: 0.};
                    return Some(hit);
                }
                None  
            }
            Self::Quad { pos, delta_x, delta_y, kind, n, w, mat } => {
                if let Some(mut hit) = Self::intersect(&Self::Plane { pos: pos.clone(), normal: n.normalize(), mat: mat.clone()}, ray) {
                    let p = pos - &hit.p;
                    let (alpha, beta) = Self::calc_quadrilet(&p, delta_x, delta_y, w);
                    if kind.get_fn()(alpha, beta){
                        hit.u = alpha;
                        hit.v = beta;
                        return Some(hit);
                    }
                    return None;
                }

                None
            },
            Self::BoundBox { min, max, inside } => {
                let tx0 = ((min.x - ray.start.x)/ray.dir.x).min((max.x - ray.start.x)/ray.dir.x);
                let tx1 = ((min.x - ray.start.x)/ray.dir.x).max((max.x - ray.start.x)/ray.dir.x);
                let ty0 = ((min.y - ray.start.y)/ray.dir.y).min((max.y - ray.start.y)/ray.dir.y);
                let ty1 = ((min.y - ray.start.y)/ray.dir.y).max((max.y - ray.start.y)/ray.dir.y);
                let tz0 = ((min.z - ray.start.z)/ray.dir.z).min((max.z - ray.start.z)/ray.dir.z);
                let tz1 = ((min.z - ray.start.z)/ray.dir.z).max((max.z - ray.start.z)/ray.dir.z);
                let before_cam = tx1 > 0. && ty1 > 0. && tz1 > 0.;
                let nan = tx0.is_nan() || tx1.is_nan() || ty0.is_nan() || ty1.is_nan() || tz0.is_nan() || tz1.is_nan();
                let over =  Self::overlap(tx0, tx1, ty0, ty1) && Self::overlap(tx0, tx1, tz0, tz1) && Self::overlap(tz0, tz1, ty0, ty1);
                if (nan || over) && before_cam {
                    return Self::hit_all(ray, inside);
                }
                None
            }
        }
    }
    pub fn bounce(ray: &Ray, objs: &Vec<Object>, max_bounce: u8, env_shader: &Box<dyn Fn(&Vec3)->Vec3+Send+Sync>) -> Vec3{
        assert!(ray.dir.is_normalized());
        if max_bounce <= 0 {
            return Vec3::new(0., 0., 0.);
        }

        match Self::hit_all(ray, objs) {
            Some(hit) => {
                let r = scatter(ray, &hit);
                let future = Self::bounce(&r, objs, max_bounce - 1, env_shader);
                hit.get_color(future)

            }

            None => {
                
                env_shader(&ray.dir)
            }
        } 
    }
   
    
    pub fn hit_all(ray: &Ray, lis: &Vec<Self>) -> Option<HitInfo>{
        let mut inf: Option<HitInfo> = None;
        let mut min_dist = 100000.;
        for obj in lis {
            match Self::intersect(&obj, ray){
                Some(i) => {
                    let len = (&i.p - &ray.start).length();
                    if len < 0.0001{
                        continue;
                    }
                    else if len < min_dist {
                        inf = Some(i);
                        min_dist = len;
                    } else {
                        continue;
                    }
                }

                None => continue,
            }
        }

        inf
    }

    pub fn new_quad(pos: Vec3, delta_x: Vec3, delta_y: Vec3, kind: QuadType, mat: Material) -> Self{
        let n = delta_y.cross(&delta_x);
        let w = &n / &n.dot(&n);
        Self::Quad { pos, delta_x, delta_y, kind, n, w, mat }
    }
}



pub mod abstract_object {
    use super::*;
    pub fn new_cylinder(pos: &Vec3, delta_y: &Vec3, faces: u32, radius: f32, fill: bool, mat: Material)-> Vec<Object>{
        let mut rects: Vec<Object> = vec![];
        let mut top: Vec<Vec3> = vec![];
        let mut bottom: Vec<Vec3> = vec![];
        let angle = 360/faces;
        let rad2 = radius * radius;
        //c^2 = a^2+b^2-2*a*b*cos(theta)
        let width = (2.* rad2 - 2.*rad2*f32::to_radians(angle as f32).cos()).abs().sqrt();
        let base_z = delta_y.normalize();
        let trans = Vec3::calc_new_bases(&base_z);
        for i in (0..360).step_by(angle as usize){
            let rect_pos = pos+Vec3::back().rot_z(i as f32)*&trans;
            let delta_x = delta_y.cross(&(&rect_pos-pos)).normalize()*width;
            let corner = rect_pos - &delta_x/2.- delta_y/2.;
            if fill{
                top.push(&corner+&delta_x+delta_y);
                bottom.push(&corner+&delta_x);
            }
            let rect = Object::new_quad(corner, delta_x.clone(), delta_y.clone(), QuadType::Rect(), mat.clone());
            rects.push(rect);
        }
        if fill{
            let mut top_obj = concave_n_poly(top, mat.clone());
            let mut bottom_obj = concave_n_poly(bottom, mat.clone());
            rects.append(&mut top_obj);
            rects.append(&mut bottom_obj);
        }
        rects
    }

    pub fn concave_n_poly(points: Vec<Vec3>, mat: Material) -> Vec<Object>{
        let mut triangles: Vec<Object> = vec![];
        let last = &points[points.len()-1];
        for i in 1..points.len()-1{
            let delta_x = &points[i-1] - last;
            let delta_y = &points[i] - last;

            let triangle = Object::new_quad(last.clone(), delta_x, delta_y, QuadType::Triangle(), mat.clone());
            triangles.push(triangle);
        }
        triangles
    }

    pub fn new_pyramid(pos: &Vec3, size: f32, delta_y: &Vec3, rotation: f32, mat: Material) -> Vec<Object>{
        let mut objs: Vec<Object> = vec![];
        let mut points: Vec<Vec3> = vec![];
        let trans = &Vec3::calc_new_bases(&delta_y.normalize());
        points.push((pos-Vec3::side()*size/2.-Vec3::back()*size/2.).rot_z(rotation)*trans-delta_y/2.);
        points.push((pos+Vec3::side()*size/2.-Vec3::back()*size/2.).rot_z(rotation)*trans-delta_y/2.);
        points.push((pos+Vec3::side()*size/2.+Vec3::back()*size/2.).rot_z(rotation)*trans-delta_y/2.);
        points.push((pos-Vec3::side()*size/2.+Vec3::back()*size/2.).rot_z(rotation)*trans-delta_y/2.);
        points.push(points[0].clone());
        points.push(pos+delta_y/2.);

        objs.push(Object::new_quad(points[0].clone(), &points[1]-&points[0], &points[3]-&points[0], QuadType::Rect(), mat.clone()));
        objs.append(&mut concave_n_poly(points, mat));
        objs
    }

    pub fn new_box(pos: &Vec3, delta_y: &Vec3, size: f32, mat: Material)->Vec<Object>{
        let radius = (2.*f32::powi(size, 2)).sqrt()/2.;
        self::new_cylinder(pos, delta_y, 4, radius, true, mat)
    }
}