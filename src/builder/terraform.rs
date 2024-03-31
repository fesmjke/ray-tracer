use crate::camera::Camera;
use crate::canvas::Canvas;
use crate::color::Color;
use crate::lights::PointLight;
use crate::material::Material;
use crate::matrices::{Matrix, Matrix4};
use crate::patterns::Pattern;
use crate::point::Point;
use crate::primitives::PrimitiveShape::{CubeShape, PlaneShape, SphereShape};
use crate::primitives::{Cube, Plane, PrimitiveShape, Sphere};
use crate::render::Render;
use crate::transformations::Transform::{Orientation, Rotate, Scale, Translate};
use crate::transformations::{Over, Transformable};
use crate::utils::degrees_to_radians;
use crate::vector::Vector3;
use crate::world::World;

// THIS IS THE MOST HORRIBLE CODE THAT I EVER WROTE
// IF YOU READ THIS, PLEASE, SKIP...
// I KNOW THAT THIS CAN BE DONE USING SERDE

// TODO: REWRITE USING SERDE

pub struct Terraform {
    world: World,
    camera: Camera,
}

impl Terraform {
    pub fn render(&self) -> Canvas {
        self.camera.render(&self.world, Render::default())
    }
    pub fn parse(raw_world: &str) -> Terraform {
        let mut camera = Camera::new(0, 0, 0.0);

        let mut primitives: Vec<PrimitiveShape> = vec![];
        let mut lights: Vec<PointLight> = vec![];

        let raw_world = raw_world.to_string();

        let splitted_world = raw_world
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        for (index, splitted) in splitted_world.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Primitive" => {
                        let title_general = splitted_world[index];
                        let primitive_type = splitted_world[index + 1];
                        // material color
                        let title_material = splitted_world[index + 2];
                        let title_color = splitted_world[index + 3];
                        let r = splitted_world[index + 4];
                        let g = splitted_world[index + 5];
                        let b = splitted_world[index + 6];
                        // material details
                        let details_ambient = splitted_world[index + 7];
                        let details_diffuse = splitted_world[index + 8];
                        let details_specular = splitted_world[index + 9];
                        let details_reflective = splitted_world[index + 10];
                        let details_refractive_index = splitted_world[index + 11];
                        let details_transparency = splitted_world[index + 12];
                        let details_shininess = splitted_world[index + 13];
                        // pattern
                        let title_pattern = splitted_world[index + 14];
                        let pattern_type = splitted_world[index + 15];
                        let title_pattern_color_a = splitted_world[index + 16];
                        let pattern_color_a_r = splitted_world[index + 17];
                        let pattern_color_a_g = splitted_world[index + 18];
                        let pattern_color_a_b = splitted_world[index + 19];
                        let title_pattern_color_b = splitted_world[index + 20];
                        let pattern_color_b_r = splitted_world[index + 21];
                        let pattern_color_b_g = splitted_world[index + 22];
                        let pattern_color_b_b = splitted_world[index + 23];

                        let title_transform = splitted_world[index + 24];
                        let title_rotate = splitted_world[index + 25];
                        let over = splitted_world[index + 26];
                        let angle = splitted_world[index + 27];
                        // translate
                        let title_translate = splitted_world[index + 28];
                        let translate_x = splitted_world[index + 29];
                        let translate_y = splitted_world[index + 30];
                        let translate_z = splitted_world[index + 31];
                        // scale
                        let title_scale = splitted_world[index + 32];
                        let scale_x = splitted_world[index + 33];
                        let scale_y = splitted_world[index + 34];
                        let scale_z = splitted_world[index + 35];

                        let title_transform_primitive = splitted_world[index + 36];
                        let title_rotate_primitive = splitted_world[index + 37];
                        let over_primitive = splitted_world[index + 38];
                        let angle_primitive = splitted_world[index + 39];
                        // translate
                        let title_translate_primitive = splitted_world[index + 40];
                        let translate_x_primitive = splitted_world[index + 41];
                        let translate_y_primitive = splitted_world[index + 42];
                        let translate_z_primitive = splitted_world[index + 43];
                        // scale
                        let title_scale_primitive = splitted_world[index + 44];
                        let scale_x_primitive = splitted_world[index + 45];
                        let scale_y_primitive = splitted_world[index + 46];
                        let scale_z_primitive = splitted_world[index + 47];

                        let temp = vec![
                            title_general,
                            primitive_type,
                            title_material,
                            title_color,
                            r,
                            g,
                            b,
                            details_ambient,
                            details_diffuse,
                            details_specular,
                            details_reflective,
                            details_refractive_index,
                            details_transparency,
                            details_shininess,
                            title_pattern,
                            pattern_type,
                            title_pattern_color_a,
                            pattern_color_a_r,
                            pattern_color_a_g,
                            pattern_color_a_b,
                            title_pattern_color_b,
                            pattern_color_b_r,
                            pattern_color_b_g,
                            pattern_color_b_b,
                            title_transform,
                            title_rotate,
                            over,
                            angle,
                            title_translate,
                            translate_x,
                            translate_y,
                            translate_z,
                            title_scale,
                            scale_x,
                            scale_y,
                            scale_z,
                            title_transform_primitive,
                            title_rotate_primitive,
                            over_primitive,
                            angle_primitive,
                            title_translate_primitive,
                            translate_x_primitive,
                            translate_y_primitive,
                            translate_z_primitive,
                            title_scale_primitive,
                            scale_x_primitive,
                            scale_y_primitive,
                            scale_z_primitive,
                        ];

                        let primitive = Self::parse_primitive(temp.join("\n").as_str());
                        primitives.push(primitive);
                    }
                    "PointLight" => {
                        let title_general = splitted_world[index];
                        // material color
                        let origin_point = splitted_world[index + 1];
                        let x = splitted_world[index + 2];
                        let y = splitted_world[index + 3];
                        let z = splitted_world[index + 4];
                        let color = splitted_world[index + 5];
                        let r = splitted_world[index + 6];
                        let g = splitted_world[index + 7];
                        let b = splitted_world[index + 8];

                        let temp = vec![title_general, origin_point, x, y, z, color, r, g, b];

                        let light = Self::parse_light(temp.join("\n").as_str());
                        lights.push(light);
                    }
                    "Camera" => {
                        let title_general = splitted_world[index];
                        // material color
                        let camera_factor = splitted_world[index + 1];
                        let camera_hbase = splitted_world[index + 2];
                        let camera_vbase = splitted_world[index + 3];
                        let camera_fov = splitted_world[index + 4];
                        let title_vectorf = splitted_world[index + 5];
                        let fx = splitted_world[index + 6];
                        let fy = splitted_world[index + 7];
                        let fz = splitted_world[index + 8];
                        let title_vectort = splitted_world[index + 9];
                        let tx = splitted_world[index + 10];
                        let ty = splitted_world[index + 11];
                        let tz = splitted_world[index + 12];
                        let title_vectoru = splitted_world[index + 13];
                        let ux = splitted_world[index + 14];
                        let uy = splitted_world[index + 15];
                        let uz = splitted_world[index + 16];

                        let temp = vec![
                            title_general,
                            camera_factor,
                            camera_hbase,
                            camera_vbase,
                            camera_fov,
                            title_vectorf,
                            fx,
                            fy,
                            fz,
                            title_vectort,
                            tx,
                            ty,
                            tz,
                            title_vectoru,
                            ux,
                            uy,
                            uz,
                        ];

                        camera = Self::parse_camera(temp.join("\n").as_str());
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general material `{:?}` property",
                            splitted_properties[0]
                        );
                    }
                }
            }
        }
        let world = World::default()
            .with_objects(primitives)
            .with_light_sources(lights);

        Self { world, camera }
    }

    fn parse_primitive(raw_primitive: &str) -> PrimitiveShape {
        let raw_primitive = raw_primitive.to_string();

        let splitted_primitive = raw_primitive
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut primitive_type = "";
        let mut transformation = Matrix4::identity();
        let mut material = Material::default();

        for (index, splitted) in splitted_primitive.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Primitive" => {}
                    "Material" => {
                        let title_general = splitted_primitive[index];
                        // material color
                        let title_color = splitted_primitive[index + 1];
                        let r = splitted_primitive[index + 2];
                        let g = splitted_primitive[index + 3];
                        let b = splitted_primitive[index + 4];
                        // material details
                        let details_ambient = splitted_primitive[index + 5];
                        let details_diffuse = splitted_primitive[index + 6];
                        let details_specular = splitted_primitive[index + 7];
                        let details_reflective = splitted_primitive[index + 8];
                        let details_refractive_index = splitted_primitive[index + 9];
                        let details_transparency = splitted_primitive[index + 10];
                        let details_shininess = splitted_primitive[index + 11];
                        // pattern
                        let title_pattern = splitted_primitive[index + 12];
                        let pattern_type = splitted_primitive[index + 13];
                        let title_pattern_color_a = splitted_primitive[index + 14];
                        let pattern_color_a_r = splitted_primitive[index + 15];
                        let pattern_color_a_g = splitted_primitive[index + 16];
                        let pattern_color_a_b = splitted_primitive[index + 17];
                        let title_pattern_color_b = splitted_primitive[index + 18];
                        let pattern_color_b_r = splitted_primitive[index + 19];
                        let pattern_color_b_g = splitted_primitive[index + 20];
                        let pattern_color_b_b = splitted_primitive[index + 21];

                        let title_transform = splitted_primitive[index + 22];
                        let title_rotate = splitted_primitive[index + 23];
                        let over = splitted_primitive[index + 24];
                        let angle = splitted_primitive[index + 25];
                        // translate
                        let title_translate = splitted_primitive[index + 26];
                        let translate_x = splitted_primitive[index + 27];
                        let translate_y = splitted_primitive[index + 28];
                        let translate_z = splitted_primitive[index + 29];
                        // scale
                        let title_scale = splitted_primitive[index + 30];
                        let scale_x = splitted_primitive[index + 31];
                        let scale_y = splitted_primitive[index + 32];
                        let scale_z = splitted_primitive[index + 33];

                        let temp = vec![
                            title_general,
                            title_color,
                            r,
                            g,
                            b,
                            details_ambient,
                            details_diffuse,
                            details_specular,
                            details_reflective,
                            details_refractive_index,
                            details_transparency,
                            details_shininess,
                            title_pattern,
                            pattern_type,
                            title_pattern_color_a,
                            pattern_color_a_r,
                            pattern_color_a_g,
                            pattern_color_a_b,
                            title_pattern_color_b,
                            pattern_color_b_r,
                            pattern_color_b_g,
                            pattern_color_b_b,
                            title_transform,
                            title_rotate,
                            over,
                            angle,
                            title_translate,
                            translate_x,
                            translate_y,
                            translate_z,
                            title_scale,
                            scale_x,
                            scale_y,
                            scale_z,
                        ];

                        material = Self::parse_material(temp.join("\n").as_str());
                    }
                    "Transform" => {
                        let title_transform = splitted_primitive[index];
                        let title_rotate = splitted_primitive[index + 1];
                        let over = splitted_primitive[index + 2];
                        let angle = splitted_primitive[index + 3];
                        // translate
                        let title_translate = splitted_primitive[index + 4];
                        let translate_x = splitted_primitive[index + 5];
                        let translate_y = splitted_primitive[index + 6];
                        let translate_z = splitted_primitive[index + 7];
                        // scale
                        let title_scale = splitted_primitive[index + 8];
                        let scale_x = splitted_primitive[index + 9];
                        let scale_y = splitted_primitive[index + 10];
                        let scale_z = splitted_primitive[index + 11];

                        let temp = vec![
                            title_transform,
                            title_rotate,
                            over,
                            angle,
                            title_translate,
                            translate_x,
                            translate_y,
                            translate_z,
                            title_scale,
                            scale_x,
                            scale_y,
                            scale_z,
                        ];

                        transformation = Self::parse_transformation(temp.join("\n").as_str());
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general material `{:?}` property",
                            splitted_properties[0]
                        );
                    }
                }
            }

            if splitted_properties.len() == 2 {
                match splitted_properties[1] {
                    "sphere" => {
                        primitive_type = "sphere";
                    }
                    "cube" => {
                        primitive_type = "cube";
                    }
                    "plane" => {
                        primitive_type = "plane";
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped material `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }
        }

        return match primitive_type {
            "sphere" => SphereShape(
                Sphere::default()
                    .transform(&transformation)
                    .apply_material(material),
            ),
            "cube" => CubeShape(
                Cube::default()
                    .transform(&transformation)
                    .apply_material(material),
            ),
            "plane" => PlaneShape(
                Plane::default()
                    .transform(&transformation)
                    .apply_material(material),
            ),
            _ => {
                println!("TERRAFORM WARNING: skipped primitive type");
                SphereShape(Sphere::default())
            }
        };
    }

    fn parse_light(raw_light: &str) -> PointLight {
        let raw_light = raw_light.to_string();

        let splitted_light = raw_light
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut light = PointLight::default();

        for (index, splitted) in splitted_light.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "PointLight" => {}
                    "OriginPoint" => {
                        // assume that x,y,z is exists
                        let title = splitted_light[index];
                        let x = splitted_light[index + 1];
                        let y = splitted_light[index + 2];
                        let z = splitted_light[index + 3];

                        let temp = vec![title, x, y, z];

                        let position = Self::parse_point(temp.join("\n").as_str());
                        light.with_position(position);
                    }
                    "Color" => {
                        // assume that r,g,b is exists
                        let title = splitted_light[index];
                        let r = splitted_light[index + 1];
                        let g = splitted_light[index + 2];
                        let b = splitted_light[index + 3];

                        let temp = vec![title, r, g, b];

                        let color = Self::parse_color(temp.join("\n").as_str());
                        light.with_color(color);
                    }
                    _ => panic!("TERRAFORM PANIC: Unknown general property in lightning!"),
                }
            } else {
                continue;
            }
        }

        light
    }

    fn parse_material(raw_material: &str) -> Material {
        let raw_material = raw_material.to_string();

        let splitted_material = raw_material
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut material_color = Color::default();
        // default properties from material :)
        let mut ambient = 0.1;
        let mut diffuse = 0.9;
        let mut specular = 0.9;
        let mut reflective = 0.0;
        let mut refractive_index = 1.0;
        let mut transparency = 0.0;
        let mut shininess = 200.0;
        let mut pattern = None;

        for (index, splitted) in splitted_material.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 2 {
                match splitted_properties[0] {
                    "ambient" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse ambient value property");
                        ambient = parsed_property;
                    }
                    "diffuse" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse diffuse value property");
                        diffuse = parsed_property;
                    }
                    "specular" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse specular value property");
                        specular = parsed_property;
                    }
                    "reflective" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse reflective value property");
                        reflective = parsed_property;
                    }
                    "refractive_index" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse refractive_index value property");
                        refractive_index = parsed_property;
                    }
                    "transparency" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse transparency value property");
                        transparency = parsed_property;
                    }
                    "shininess" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse shininess value property");
                        shininess = parsed_property;
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped material `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Material" => {}

                    "Transform" => {}

                    "Pattern" => {
                        let title = splitted_material[index];
                        let pattern_type = splitted_material[index + 1];
                        let color_a = splitted_material[index + 2];
                        let r1 = splitted_material[index + 3];
                        let g1 = splitted_material[index + 4];
                        let b1 = splitted_material[index + 5];
                        let color_b = splitted_material[index + 6];
                        let r2 = splitted_material[index + 7];
                        let g2 = splitted_material[index + 8];
                        let b2 = splitted_material[index + 9];

                        let title_transformation = splitted_material[index + 10];
                        // rotate
                        let title_rotate = splitted_material[index + 11];
                        let over = splitted_material[index + 12];
                        let angle = splitted_material[index + 13];
                        // translate
                        let title_translate = splitted_material[index + 14];
                        let translate_x = splitted_material[index + 15];
                        let translate_y = splitted_material[index + 16];
                        let translate_z = splitted_material[index + 17];
                        // scale
                        let title_scale = splitted_material[index + 18];
                        let scale_x = splitted_material[index + 19];
                        let scale_y = splitted_material[index + 20];
                        let scale_z = splitted_material[index + 21];

                        let temp = vec![
                            title,
                            pattern_type,
                            color_a,
                            r1,
                            g1,
                            b1,
                            color_b,
                            r2,
                            g2,
                            b2,
                            title_transformation,
                            title_rotate,
                            over,
                            angle,
                            title_translate,
                            translate_x,
                            translate_y,
                            translate_z,
                            title_scale,
                            scale_x,
                            scale_y,
                            scale_z,
                        ];

                        pattern = Some(Self::parse_pattern(temp.join("\n").as_str()));
                    }
                    "Color" => {
                        let title = splitted_material[index];
                        let r = splitted_material[index + 1];
                        let g = splitted_material[index + 2];
                        let b = splitted_material[index + 3];

                        let temp = vec![title, r, g, b];

                        material_color = Self::parse_color(temp.join("\n").as_str());
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general material `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }
        }

        let mut material = Material::default()
            .color(material_color)
            .ambient(ambient)
            .diffuse(diffuse)
            .specular(specular)
            .reflective(reflective)
            .refractive_index(refractive_index)
            .transparency(transparency)
            .shininess(shininess);

        match pattern {
            Some(pattern) => {
                material = material.apply_pattern(pattern);
            }
            None => {}
        }

        material
    }

    fn parse_transformation(raw_transformation: &str) -> Matrix4 {
        let raw_transformation = raw_transformation.to_string();

        let splitted_transformation = raw_transformation
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut transformation_matrix = Matrix4::identity();

        for (index, splitted) in splitted_transformation.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Transform" => {}
                    "Rotate" => {
                        // assume that over and angle is exists
                        let title = splitted_transformation[index];
                        let over = splitted_transformation[index + 1];
                        let angle = splitted_transformation[index + 2];

                        let temp = vec![title, over, angle];

                        transformation_matrix =
                            Self::parse_rotate(temp.join("\n").as_str()) * transformation_matrix;
                    }
                    "Scale" => {
                        // assume that x, y, z is exists
                        let title = splitted_transformation[index].to_string();
                        let x = splitted_transformation[index + 1].to_string();
                        let y = splitted_transformation[index + 2].to_string();
                        let z = splitted_transformation[index + 3].to_string();

                        let temp = vec![title, x, y, z];

                        transformation_matrix =
                            Self::parse_scale(temp.join("\n").as_str()) * transformation_matrix;
                    }
                    "Translate" => {
                        // assume that x, y, z is exists
                        let title = splitted_transformation[index].to_string();
                        let x = splitted_transformation[index + 1].to_string();
                        let y = splitted_transformation[index + 2].to_string();
                        let z = splitted_transformation[index + 3].to_string();

                        let temp = vec![title, x, y, z];
                        transformation_matrix =
                            Self::parse_translate(temp.join("\n").as_str()) * transformation_matrix;
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general transformation `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            } else {
                continue;
            }
        }

        transformation_matrix
    }

    fn parse_rotate(raw_rotate: &str) -> Matrix4 {
        let raw_rotate = raw_rotate.to_string();

        // KILL ME)

        let splitted_rotate = raw_rotate
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let mut angle: f64;

        // assume that angle is exists

        match splitted_rotate.get(2) {
            Some(angle_property) => {
                let angle_property = angle_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                angle = angle_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse angle value property");
            }
            None => panic!("TERRAFORM PANIC: missing rotate `angle` property!"),
        }

        // assume that rotate is exists

        angle = degrees_to_radians(angle);

        let rotate_matrix: Matrix4;

        match splitted_rotate.get(1) {
            Some(over_property) => {
                let splited_over_property = over_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                match splited_over_property[1] {
                    "x" => rotate_matrix = Rotate(Over::X, angle).transformation(),
                    "y" => rotate_matrix = Rotate(Over::Y, angle).transformation(),
                    "z" => rotate_matrix = Rotate(Over::Z, angle).transformation(),
                    _ => {
                        panic!("TERRAFORM PANIC: missing rotate `over` property!")
                    }
                }
            }
            None => {
                panic!("TERRAFORM PANIC: missing rotate `over` property!")
            }
        }

        rotate_matrix
    }

    fn parse_translate(raw_translate: &str) -> Matrix4 {
        let raw_translate = raw_translate.to_string();

        // KILL ME)

        let splitted_translate = raw_translate
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let x: f64;
        let y: f64;
        let z: f64;

        // assume that coords is exists

        match splitted_translate.get(1) {
            Some(x_property) => {
                let x_property = x_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                x = x_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `x` value property");
            }
            None => panic!("TERRAFORM PANIC: missing translate `x` property!"),
        }

        // assume that rotate is exists

        match splitted_translate.get(2) {
            Some(y_property) => {
                let y_property = y_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                y = y_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `y` value property");
            }
            None => panic!("TERRAFORM PANIC: missing translate `y` property!"),
        }

        match splitted_translate.get(3) {
            Some(z_property) => {
                let z_property = z_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                z = z_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `z` value property");
            }
            None => panic!("TERRAFORM PANIC: missing translate `z` property!"),
        }

        Translate(x, y, z).transformation()
    }

    fn parse_scale(raw_scale: &str) -> Matrix4 {
        let raw_scale = raw_scale.to_string();

        // KILL ME)

        let splitted_scale = raw_scale
            .split("\n")
            .map(|x| x.trim())
            .filter(|x| !x.is_empty())
            .collect::<Vec<&str>>();

        let x: f64;
        let y: f64;
        let z: f64;

        // assume that coords is exists

        match splitted_scale.get(1) {
            Some(x_property) => {
                let x_property = x_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                x = x_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `x` value property");
            }
            None => panic!("TERRAFORM PANIC: missing scale `x` property!"),
        }

        // assume that rotate is exists

        match splitted_scale.get(2) {
            Some(y_property) => {
                let y_property = y_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                y = y_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `y` value property");
            }
            None => panic!("TERRAFORM PANIC: missing scale `y` property!"),
        }

        match splitted_scale.get(3) {
            Some(z_property) => {
                let z_property = z_property
                    .split(":")
                    .map(|x| x.trim())
                    .collect::<Vec<&str>>();

                z = z_property[1]
                    .parse::<f64>()
                    .expect("TERRAFORM ERROR: parse `z` value property");
            }
            None => panic!("TERRAFORM PANIC: missing scale `z` property!"),
        }

        Scale(x, y, z).transformation()
    }

    fn parse_color(raw_color: &str) -> Color {
        let raw_color = raw_color.to_string();

        let splitted_color = raw_color
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut color = Color::default();

        for splitted in splitted_color.into_iter() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() < 2 || splitted_properties.is_empty() {
                continue;
            }

            let parsed_property = splitted_properties[1]
                .parse::<f64>()
                .expect("TERRAFORM ERROR: parse color value property");

            match splitted_properties[0] {
                "r" => color.with_red(parsed_property),
                "g" => color.with_green(parsed_property),
                "b" => color.with_blue(parsed_property),
                _ => panic!("TERRAFORM PANIC: Unknown color property!"),
            }
        }

        color
    }

    fn parse_pattern(raw_pattern: &str) -> Pattern {
        let raw_pattern = raw_pattern.to_string();

        let splitted_pattern = raw_pattern
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut pattern_type = "";
        let mut color_a = Color::default();
        let mut color_b = Color::default();
        let mut transformation = Matrix4::identity();

        for (index, splitted) in splitted_pattern.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Pattern" => {}

                    "Transform" => {
                        let title_transform = splitted_pattern[index];
                        let title_rotate = splitted_pattern[index + 1];
                        let over = splitted_pattern[index + 2];
                        let angle = splitted_pattern[index + 3];
                        // translate
                        let title_translate = splitted_pattern[index + 4];
                        let translate_x = splitted_pattern[index + 5];
                        let translate_y = splitted_pattern[index + 6];
                        let translate_z = splitted_pattern[index + 7];
                        // scale
                        let title_scale = splitted_pattern[index + 8];
                        let scale_x = splitted_pattern[index + 9];
                        let scale_y = splitted_pattern[index + 10];
                        let scale_z = splitted_pattern[index + 11];

                        let temp = vec![
                            title_transform,
                            title_rotate,
                            over,
                            angle,
                            title_translate,
                            translate_x,
                            translate_y,
                            translate_z,
                            title_scale,
                            scale_x,
                            scale_y,
                            scale_z,
                        ];

                        transformation = Self::parse_transformation(temp.join("\n").as_str());
                    }

                    "Color_a" => {
                        let title = splitted_pattern[index];
                        let r = splitted_pattern[index + 1];
                        let g = splitted_pattern[index + 2];
                        let b = splitted_pattern[index + 3];

                        let temp = vec![title, r, g, b];

                        color_a = Self::parse_color(temp.join("\n").as_str());
                    }
                    "Color_b" => {
                        let title = splitted_pattern[index];
                        let r = splitted_pattern[index + 1];
                        let g = splitted_pattern[index + 2];
                        let b = splitted_pattern[index + 3];

                        let temp = vec![title, r, g, b];

                        color_b = Self::parse_color(temp.join("\n").as_str());
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general pattern `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }

            if splitted_properties.len() == 2 {
                match splitted_properties[0] {
                    "pattern_type" => pattern_type = splitted_properties[1],
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped pattern `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }
        }

        return match pattern_type {
            "plain" => Pattern::new_plain(color_a).transform(&transformation),
            "checker" => Pattern::new_checker(color_a, color_b).transform(&transformation),
            "gradient" => Pattern::new_gradient(color_a, color_b).transform(&transformation),
            "ring" => Pattern::new_ring(color_a, color_b).transform(&transformation),
            "stripe" => Pattern::new_stripe(color_a, color_b).transform(&transformation),
            _ => Pattern::new_plain(Color::default()).transform(&transformation),
        };
    }

    fn parse_point(raw_point: &str) -> Point {
        let raw_point = raw_point.to_string();

        let splitted_point = raw_point
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut point = Point::default();

        for splitted in splitted_point.into_iter() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() < 2 || splitted_properties.is_empty() {
                continue;
            }

            let parsed_property = splitted_properties[1]
                .parse::<f64>()
                .expect("TERRAFORM ERROR: parse point value property");

            match splitted_properties[0] {
                "x" => point.with_x(parsed_property),
                "y" => point.with_y(parsed_property),
                "z" => point.with_z(parsed_property),
                _ => panic!("TERRAFORM PANIC: Unknown point property!"),
            }
        }

        point
    }

    fn parse_vector(raw_vector: &str) -> Vector3 {
        let raw_vector = raw_vector.to_string();

        let splitted_vector = raw_vector
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut vector = Vector3::default();

        for splitted in splitted_vector.into_iter() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() < 2 || splitted_properties.is_empty() {
                continue;
            }

            let parsed_property = splitted_properties[1]
                .parse::<f64>()
                .expect("TERRAFORM ERROR: parse vector value property");

            match splitted_properties[0] {
                "x" => vector.with_x(parsed_property),
                "y" => vector.with_y(parsed_property),
                "z" => vector.with_z(parsed_property),
                _ => panic!("TERRAFORM PANIC: Unknown vector property!"),
            }
        }

        vector
    }

    fn parse_camera(raw_camera: &str) -> Camera {
        let raw_camera = raw_camera.to_string();

        let splitted_camera = raw_camera
            .split("\n")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();

        let mut factor = 0;
        let mut hsize = 0;
        let mut vsize = 0;
        let mut fov = 0.0;

        let mut from = Vector3::default();
        let mut to = Vector3::default();
        let mut up = Vector3::default();

        for (index, splitted) in splitted_camera.iter().enumerate() {
            let splitted_properties = splitted
                .split(":")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim())
                .collect::<Vec<&str>>();

            if splitted_properties.len() == 1 {
                match splitted_properties[0] {
                    "Camera" => {}
                    "VectorFrom" => {
                        let title = splitted_camera[index];
                        let x = splitted_camera[index + 1];
                        let y = splitted_camera[index + 2];
                        let z = splitted_camera[index + 3];

                        let temp = vec![title, x, y, z];
                        from = Self::parse_vector(temp.join("\n").as_str())
                    }
                    "VectorTo" => {
                        let title = splitted_camera[index];
                        let x = splitted_camera[index + 1];
                        let y = splitted_camera[index + 2];
                        let z = splitted_camera[index + 3];

                        let temp = vec![title, x, y, z];
                        to = Self::parse_vector(temp.join("\n").as_str())
                    }
                    "VectorUp" => {
                        let title = splitted_camera[index];
                        let x = splitted_camera[index + 1];
                        let y = splitted_camera[index + 2];
                        let z = splitted_camera[index + 3];

                        let temp = vec![title, x, y, z];
                        up = Self::parse_vector(temp.join("\n").as_str())
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped general camera `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }

            if splitted_properties.len() == 2 {
                match splitted_properties[0] {
                    "factor" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<usize>()
                            .expect("TERRAFORM ERROR: parse camera value property");
                        factor = parsed_property;
                    }
                    "horizontal_base" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<usize>()
                            .expect("TERRAFORM ERROR: parse camera value property");
                        hsize = parsed_property;
                    }
                    "vertical_base" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<usize>()
                            .expect("TERRAFORM ERROR: parse camera value property");
                        vsize = parsed_property;
                    }
                    "fov" => {
                        let parsed_property = splitted_properties[1]
                            .parse::<f64>()
                            .expect("TERRAFORM ERROR: parse camera value property");

                        fov = degrees_to_radians(parsed_property);
                    }
                    _ => {
                        println!(
                            "TERRAFORM WARNING: skipped camera `{:?}` property",
                            splitted_properties[0]
                        );
                        continue;
                    }
                }
            }
        }

        let camera = Camera::new(factor * hsize, factor * vsize, fov)
            .transform(&Orientation(from, to, up).transformation());

        camera
    }
}

#[cfg(test)]
mod terraform_tests {
    use crate::builder::terraform::Terraform;
    use crate::camera::Camera;
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::material::Material;
    use crate::patterns::Pattern;
    use crate::point::Point;
    use crate::primitives::{Cube, Primitive, PrimitiveShape, Sphere};
    use crate::transformations::Transform::{Orientation, Rotate, Scale, Translate};
    use crate::transformations::{Over, Transformable};
    use crate::vector::Vector3;
    use crate::world::World;
    use std::f64::consts::PI;

    #[test]
    fn terraform_creation() {
        let raw_world = "";

        let terraform = Terraform::parse(raw_world);
        let expected_world = World::default();

        assert_eq!(expected_world, terraform.world);
    }

    #[test]
    fn terraform_parse_color() {
        let raw_color = r#"
        Color:
          r: 0.53
          g: 0.13
          b: 1.0
        "#;

        let color = Terraform::parse_color(raw_color);
        let expected_color = Color::new(0.53, 0.13, 1.0);

        assert_eq!(expected_color, color);
    }

    #[test]
    fn terraform_parse_point() {
        let raw_point = r#"
        Origin:
          x: 0.0
          y: 0.2
          z: 1.0
        "#;

        let point = Terraform::parse_point(raw_point);
        let expected_point = Point::new(0.0, 0.2, 1.0);

        assert_eq!(expected_point, point);
    }

    #[test]
    fn terraform_parse_vector() {
        let raw_vector = r#"
          From:
            x: 1.01
            y: 2.21
            z: 0.0
        "#;

        let vector = Terraform::parse_vector(raw_vector);
        let expected_vector = Vector3::new(1.01, 2.21, 0.0);

        assert_eq!(expected_vector, vector);
    }

    #[test]
    fn terraform_parse_rotate() {
        let raw_rotate = r#"
          Rotate:
            over: y
            angle: 30.0
        "#;

        let rotate = Terraform::parse_rotate(raw_rotate);
        let expected_matrix = Rotate(Over::Y, PI / 6.0).transformation();

        assert_eq!(expected_matrix, rotate);
    }

    #[test]
    fn terraform_parse_translate() {
        let raw_translate = r#"
          Translate:
            x: 1.0
            y: 1.0
            z: 0.0
        "#;

        let translate = Terraform::parse_translate(raw_translate);
        let expected_matrix = Translate(1.0, 1.0, 0.0).transformation();

        assert_eq!(expected_matrix, translate);
    }

    #[test]
    fn terraform_parse_scale() {
        let raw_scale = r#"
          Scale:
            x: 1.0
            y: 1.0
            z: 1.0
        "#;

        let scale = Terraform::parse_scale(raw_scale);
        let expected_matrix = Scale(1.0, 1.0, 1.0).transformation();

        assert_eq!(expected_matrix, scale);
    }

    #[test]
    fn terraform_parse_light() {
        let raw_primitive = r#"
        PointLight:
          OriginPoint:
            x: 0.0
            y: 0.0
            z: 0.0
          Color:
            r: 0
            g: 0
            b: 0
        "#;

        let light = Terraform::parse_light(raw_primitive);
        let expected_light = PointLight::default();

        assert_eq!(expected_light, light);
    }

    #[test]
    fn terraform_parse_camera() {
        let raw_primitive = r#"
        Camera:
          factor: 16
          horizontal_base: 64
          vertical_base: 48
          fov: 30.0
            VectorFrom:
              x: 0.0
              y: 1.5
              z: -5.0
            VectorTo:
              x: 0.0
              y: 1.0
              z: 0.0
            VectorUp:
              x: 0.0
              y: 1.0
              z: 0.0
        "#;

        let camera = Terraform::parse_camera(raw_primitive);

        let factor = 16;
        let from = Vector3::new(0.0, 1.5, -5.0);
        let to = Vector3::new(0.0, 1.0, 0.0);
        let up = Vector3::new(0.0, 1.0, 0.0);
        let expected_camera = Camera::new(factor * 64, factor * 48, PI / 6.0)
            .transform(&Orientation(from, to, up).transformation());

        assert_eq!(expected_camera, camera);
    }

    #[test]
    fn terraform_parse_material() {
        let raw_material = r#"
          Material:
            Color:
              r: 0
              g: 0
              b: 0
            ambient: 0.0
            diffuse: 0.0
            specular: 0.0
            reflective: 0.0
            refractive_index: 0.0
            transparency: 0.0
            shininess: 0.0
        "#;

        let material = Terraform::parse_material(raw_material);
        let expected_material = Material::default()
            .color(Color::new(0.0, 0.0, 0.0))
            .ambient(0.0)
            .diffuse(0.0)
            .specular(0.0)
            .reflective(0.0)
            .refractive_index(0.0)
            .transparency(0.0)
            .shininess(0.0);

        assert_eq!(expected_material, material);
    }

    #[test]
    fn terraform_parse_material_pattern() {
        let raw_material = r#"
          Material:
            Color:
              r: 0
              g: 0
              b: 0
            ambient: 0.0
            diffuse: 0.0
            specular: 0.0
            reflective: 0.0
            refractive_index: 0.0
            transparency: 0.0
            shininess: 0.0
            Pattern:
              pattern_type: ring
              Color_a:
                r: 1.0
                g: 1.0
                b: 1.0
              Color_b:
                r: 0.0
                g: 0.0
                b: 0.0
              Transform:
                Rotate:
                  over: y
                  angle: 0
                Translate:
                  x: 0.0
                  y: 0.0
                  z: 0.0
                Scale:
                  x: 1.0
                  y: 1.0
                  z: 1.0
        "#;

        let pattern = Pattern::new_ring(Color::white(), Color::black());
        let material = Terraform::parse_material(raw_material);
        let expected_material = Material::default()
            .color(Color::new(0.0, 0.0, 0.0))
            .ambient(0.0)
            .diffuse(0.0)
            .specular(0.0)
            .reflective(0.0)
            .refractive_index(0.0)
            .transparency(0.0)
            .shininess(0.0)
            .apply_pattern(pattern);

        assert_eq!(expected_material, material);
    }

    #[test]
    fn terraform_parse_material_pattern_transform() {
        let raw_material = r#"
          Material:
            Color:
              r: 0
              g: 0
              b: 0
            ambient: 0.0
            diffuse: 0.0
            specular: 0.0
            reflective: 0.0
            refractive_index: 0.0
            transparency: 0.0
            shininess: 0.0
            Pattern:
              pattern_type: ring
              Color_a:
                r: 1.0
                g: 1.0
                b: 1.0
              Color_b:
                r: 0.0
                g: 0.0
                b: 0.0
              Transform:
                Rotate:
                  over: x
                  angle: 30
                Translate:
                  x: 0.0
                  y: 0.0
                  z: 0.0
                Scale:
                  x: 1.0
                  y: 1.0
                  z: 1.0
        "#;

        let pattern = Pattern::new_ring(Color::white(), Color::black())
            .rotate(Over::X, PI / 6.0)
            .translate(0.0, 0.0, 0.0)
            .scale(1.0, 1.0, 1.0)
            .transform();
        let material = Terraform::parse_material(raw_material);
        let expected_material = Material::default()
            .color(Color::new(0.0, 0.0, 0.0))
            .ambient(0.0)
            .diffuse(0.0)
            .specular(0.0)
            .reflective(0.0)
            .refractive_index(0.0)
            .transparency(0.0)
            .shininess(0.0)
            .apply_pattern(pattern);

        assert_eq!(expected_material, material);
    }

    #[test]
    fn terraform_parse_transformation() {
        let raw_world = r#"
          Transform:
            Rotate:
              over: x
              angle: 30
            Translate:
              x: 0.0
              y: 0.0
              z: 0.0
            Scale:
              x: 1.0
              y: 1.0
              z: 1.0 
        "#;

        let transform = Terraform::parse_transformation(raw_world);

        let transformation_default = Sphere::default()
            .rotate(Over::X, std::f64::consts::FRAC_PI_6)
            .translate(0.0, 0.0, 0.0)
            .scale(1.0, 1.0, 1.0)
            .transform();

        let expected_sphere = PrimitiveShape::SphereShape(transformation_default);

        assert_eq!(transform, *expected_sphere.transformation());
    }

    #[test]
    fn terraform_one_primitive_sphere() {
        let raw_primitive = r#"
        Primitive:
          primitive_type: sphere
        "#;

        let primitive = Terraform::parse_primitive(raw_primitive);
        let sphere_default = Sphere::default();
        let expected_sphere = PrimitiveShape::SphereShape(sphere_default);

        assert_eq!(expected_sphere, primitive);
    }

    #[test]
    fn terraform_one_primitive_sphere_full() {
        let raw_primitive = r#"
        Primitive:
          primitive_type: sphere
          Material:
            Color:
              r: 0
              g: 0
              b: 0
            ambient: 0.0
            diffuse: 0.0
            specular: 0.0
            reflective: 0.0
            refractive_index: 0.0
            transparency: 0.0
            shininess: 0.0
            Pattern:
              pattern_type: ring
              Color_a:
                r: 1.0
                g: 1.0
                b: 1.0
              Color_b:
                r: 0.0
                g: 0.0
                b: 0.0
              Transform:
                Rotate:
                  over: x
                  angle: 30
                Translate:
                  x: 0.0
                  y: 0.0
                  z: 0.0
                Scale:
                  x: 1.0
                  y: 1.0
                  z: 1.0
          Transform:
            Rotate:
              over: x
              angle: 30
            Translate:
              x: 0.0
              y: 0.0
              z: 0.0
            Scale:
              x: 1.0
              y: 1.0
              z: 1.0
        "#;

        let pattern = Pattern::new_ring(Color::white(), Color::black())
            .rotate(Over::X, PI / 6.0)
            .translate(0.0, 0.0, 0.0)
            .scale(1.0, 1.0, 1.0)
            .transform();

        let material = Material::default()
            .color(Color::new(0.0, 0.0, 0.0))
            .ambient(0.0)
            .diffuse(0.0)
            .specular(0.0)
            .reflective(0.0)
            .refractive_index(0.0)
            .transparency(0.0)
            .shininess(0.0)
            .apply_pattern(pattern);

        let primitive = Terraform::parse_primitive(raw_primitive);
        let sphere_default = Sphere::default()
            .rotate(Over::X, PI / 6.0)
            .translate(0.0, 0.0, 0.0)
            .scale(1.0, 1.0, 1.0)
            .transform()
            .apply_material(material);
        let expected_sphere = PrimitiveShape::SphereShape(sphere_default);

        assert_eq!(expected_sphere, primitive);
    }

    #[test]
    fn terraform_one_primitive_cube() {
        let raw_primitive = r#"
        Primitive:
          primitive_type: cube
        "#;

        let primitive = Terraform::parse_primitive(raw_primitive);
        let cube_default = Cube::default();
        let expected_cube = PrimitiveShape::CubeShape(cube_default);

        assert_eq!(expected_cube, primitive);
    }
}
