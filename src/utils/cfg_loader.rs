//! Parse and reflect config-file(.cfg) into objects.
use crate::{
    prefabs,
    BackendConfig,
    Material, Hittable,
    Scene, Entity,
    math::{ Vec3, Vec2 }
};

use std::{ cell::RefCell, fs, rc::Rc };
use jzon::{ self, object::Object, Array };
use image::ColorType;
use lazy_regex::regex;

macro_rules! bmx_failure {
    ($counter: ident, $line: ident) => {
        &format!(
            "failed to parse line {}: \"{}\" as a valid bmx-pair!", 
            $counter.borrow(), 
            $line
        )
    };
}

/// Configure Result
pub struct ConfigRes {
    pub comments: String,

    pub target_name: String,
    pub target_pixel: ColorType,
    pub target_resolution: (u32, u32),

    pub renderer_backend: BackendConfig,
    pub renderer_bvh_acc: bool,
    pub renderer_max_depth: u32,
    pub renderer_spp: u32,

    pub camera_pos: Vec3,
    pub camera_dir: Vec3,
    pub camera_fov: f64,
    pub camera_viewport: Vec2,

    pub scene: Option<Scene>
}

/// Default configurations
impl Default for ConfigRes {
    fn default() -> Self {
        ConfigRes {
            comments: String::new(),

            target_name: "out.png".to_string(),
            target_pixel: ColorType::Rgb8,
            target_resolution: (128, 128),

            renderer_backend: BackendConfig::CPUDrivenS,
            renderer_bvh_acc: false,
            renderer_max_depth: 8,
            renderer_spp: 8,

            camera_pos: Vec3::from_scalar(0.0),
            camera_dir: Vec3::new(0.0, 0.0, 1.0),
            camera_fov: 45.0,
            camera_viewport: Vec2::from_scalar(0.5),

            scene: None
        }
    }
}

/// Load .cfg file into [`ConfigRes`]
/// 
/// # Panics
/// Any following error will case panic.
/// - Syntax error
/// - File load/read error
/// - Missing required variables
pub fn from_file(path: &str) -> ConfigRes {
    let cfg_content = fs::read_to_string(path)
                        .expect(&format!("failed to read config from \"{}\" !", path));

    let mut config = ConfigRes::default();

    /* Reading Lines and Parse Bmx-Blocks */
    let line_counter = RefCell::new(0u32);
    let mut block_name = Block::None;
    let mut scene_string = String::new();
    let bmx_head_re = regex!(r"\[\s*(\w+)\s*]");
    let bmx_pair_re = regex!(r"@(\w+):\s*([\w\s\(\).,]+)");

    for line in cfg_content.lines()
                        .map(|x| { *line_counter.borrow_mut() += 1; x.trim() })
                        .filter(|x| !x.is_empty())
    {
        if let Some(head) = bmx_head_re.captures(line) {
            match head.get(1).unwrap().as_str() {
                "target"   => block_name = Block::Target,
                "renderer" => block_name = Block::Renderer,
                "camera"   => block_name = Block::Camera,
                "scene"    => block_name = Block::Scene,
                _ => panic!("unrecognized field: \"{}\" !", head.get(1).unwrap().as_str())
            }
        }
        else {
            match block_name {
                Block::Scene => scene_string.push_str(line),
                
                Block::None => { 
                    config.comments.push_str(line); 
                    config.comments.push_str("\n"); 
                },

                Block::Target => {
                    // FIXME BEGIN This part of code has been copid for 3 times!
                    let pair = bmx_pair_re.captures(line)
                                        .expect(bmx_failure!(line_counter, line));
                    let key = pair.get(1).unwrap().as_str();
                    let value = pair.get(2).unwrap().as_str();
                    // FIXME END

                    match key {
                        "name" => config.target_name = value.to_string(),
                        "pixel" => {
                            let color_t;
                            match value {
                                "RGB8"  | "rgb8"  => color_t = ColorType::Rgb8,
                                "RGB16" | "rgb16" => color_t = ColorType::Rgb16,
                                _ => panic!("unsupported target.pixel \"{}\"", value)
                            }
                            config.target_pixel = color_t;
                        },
                        "resolution" => { 
                            let res = bmx_vec2(value).expect(bmx_failure!(line_counter, line));
                            config.target_resolution = (res.x as u32, res.y as u32);
                        },
                        _ => panic!("unrecognized key \"{}\" in \"target\"!", line)
                    }
                },

                Block::Renderer => {
                    let pair = bmx_pair_re.captures(line)
                                        .expect(bmx_failure!(line_counter, line));
                    let key = pair.get(1).unwrap().as_str();
                    let value = pair.get(2).unwrap().as_str();

                    match key {
                        "backend" => {
                            match value {
                                "cpu_st" => config.renderer_backend = BackendConfig::CPUDrivenS,
                                _ => {
                                    let mt_re = regex!(r"cpu_mt\(\s*(0-9+)\s*\)");
                                    let Some(res) = mt_re.captures(value) else {
                                        panic!("failed to get thread_num in renderer.backend: \"{}\" !", line)
                                    };

                                    let thread_num = res.get(1).unwrap()
                                                        .as_str()
                                                        .parse().expect("thread_num must be an interger!");
                                    
                                    if thread_num <= 0 { panic!("thread_num must bigger than 0!") }
                                    config.renderer_backend = BackendConfig::CPUDrivenM(thread_num);
                                }
                            }
                        },
                        "bvh_acc"   => config.renderer_bvh_acc = bmx_bool(value).expect(bmx_failure!(line_counter, line)),
                        "max_depth" => config.renderer_max_depth = bmx_u32(value).expect(bmx_failure!(line_counter, line)),
                        "spp"       => config.renderer_spp = bmx_u32(value).expect(bmx_failure!(line_counter, line)),
                        _ => panic!("unrecognized key \"{}\" in \"renderer\"!", line)
                    }
                },

                Block::Camera => {
                    let pair = bmx_pair_re.captures(line)
                                        .expect(bmx_failure!(line_counter, line));
                    let key = pair.get(1).unwrap().as_str();
                    let value = pair.get(2).unwrap().as_str();

                    match key {
                        "pos" => config.camera_pos = bmx_vec3(value).expect(bmx_failure!(line_counter, line)),
                        "dir" => config.camera_dir = bmx_vec3(value).expect(bmx_failure!(line_counter, line)),
                        "fov" => config.camera_fov = bmx_f64(value).expect(bmx_failure!(line_counter, line)),
                        "viewport" => config.camera_viewport = bmx_vec2(value).expect(bmx_failure!(line_counter, line)),
                        _ => panic!("unrecognized key \"{}\" in \"camera\"!", line)
                    }
                },
            }
        }
    }

    /* Parsing Scene Block */
    if scene_string.trim().is_empty() {
        panic!("field \"scene\" is empty, which is required!");
    }

    let scene_data = jzon::parse(&scene_string)
                          .expect("failed to parse \"scene\" as Json format!");
    let scene_object = scene_data.as_object()
                                .expect("faild to parse \"scene\" as Json::Object!");

    // background
    let bg_object = scene_object.get("background")
                                .expect("failed to find \"scene.background\"!")
                                .as_object()
                                .expect("failed to parse \"scene.background\" as Json::Object!");
    let background = emit_mat(bg_object, None);
    
    // entities
    let entities_object = scene_object.get("entities")
                                      .expect("failed to find \"scene.entities\"!")
                                      .as_array()
                                      .expect("failed to parise \"scene.entities\" as Json::Array!");
    
    let mut entities = Vec::new();

    for node in entities_object {
        let entity = node.as_object()
                         .expect(&format!("Entity <{}> is not a Json::Obejct!", entities.len()));

        let mat_info = entity.get("mat")
                            .expect(&format!("failed to get material from entity <{}>!", entities.len()))
                            .as_object()
                            .expect(&format!("failed to parse entity<{}>.mat as Json::Object!", entities.len()));
        let mesh_info = entity.get("mesh")
                            .expect(&format!("failed to get mesh from entity <{}>!", entities.len()))
                            .as_object()
                            .expect(&format!("failed to parse entity<{}>.mesh as Json::Object!", entities.len()));

        let mat = emit_mat(mat_info, Some(entities.len()));
        let mesh = emit_mesh(mesh_info, entities.len());

        entities.push(Entity::new(mat, mesh));
    }

    let scene = Scene{ background, entities };
    config.scene = Some(scene);

    return config;
}

/* INTERNAL FIELD */
enum Block { None, Target, Renderer, Camera, Scene }

fn bmx_bool(value: &str) -> Result<bool, &'static str> {
    match value.to_lowercase().as_str() {
        "true" => return Ok(true),
        "false" => return Ok(false),
        _ => return Err("value is not boolean, which must be \"true\" or \"false\"!")
    }
}

fn bmx_u32(value: &str) -> Result<u32, &'static str> {
    let Ok(num) = value.parse() else {
        return Err("value is not a interger!");
    };
    return Ok(num);
}

fn bmx_f64(value: &str) -> Result<f64, &'static str> {
    let Ok(num) = value.parse() else {
        return Err("value is not a float number!");
    };
    return Ok(num);
}

fn bmx_vec2(value: &str) -> Result<Vec2, &'static str> {
    let pattern = regex!(r"\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\)");

    let Some(res) = pattern.captures(value.trim()) else {
        return Err("value is not a Vector2D!")
    };

    let x = res.get(1).unwrap()
                .as_str()
                .parse().unwrap();
    let y = res.get(2).unwrap()
                .as_str()
                .parse().unwrap();

    return Ok(Vec2 { x, y });
}

fn bmx_vec3(value: &str) -> Result<Vec3, &'static str> {
    let pattern = regex!(r"\(\s*([0-9.]+)\s*,\s*([0-9.]+)\s*,\s*([0-9.]+)\s*\)");

    let Some(res) = pattern.captures(value.trim()) else {
        return Err("value is not a Vector3D!")
    };

    let x = res.get(1).unwrap()
                .as_str()
                .parse().unwrap();
    let y = res.get(2).unwrap()
                .as_str()
                .parse().unwrap();
    let z = res.get(3).unwrap()
                .as_str()
                .parse().unwrap();

    return Ok(Vec3 { x, y, z });
}

fn json_vec3(value: &Array) -> Result<Vec3, &'static str> {
    if value.len() != 3 {
        return Err("length is not equal to 3!");
    }

    let Some(x) = value[0].as_f64() else {
        return Err("\"x\" component is not a float number!");
    };
    let Some(y) = value[1].as_f64() else {
        return Err("\"y\" component is not a float number!");
    };
    let Some(z) = value[2].as_f64() else {
        return Err("\"z\" component is not a float number!");
    };

    return Ok(Vec3 { x, y, z })
}

fn emit_mat(value: &Object, index: Option<usize>) -> Rc<dyn Material> {
    let location = match index {
        Some(i) => &format!("scene.entity<{}>", i),
        None => "scene.background"
    };

    let src = value.get("src")
                   .expect(&format!("missing material source (at {})!", location))
                   .as_str()
                   .expect(&format!("\"mat.src\" is supposed to be a Json::Str (at {})!", location))
                   .trim();
    
    let args = value.get("args")
                    .expect(&format!("missing material's arguments (at {})!", location))
                    .as_object()
                    .expect(&format!("\"mat.args\" is supposed to be a Json::Object (at {})!", location));

    let src_pattern = regex!(r"(\w+):\s*([\s\w\/\\.-]+)");
    let source = src_pattern.captures(src)
                            .expect(&format!("invalid material source format (at {})!", location));

    if source.get(1).unwrap().as_str() != "prefab" {
        panic!("only support prefab material (at {}.src)", location);
    }

    let material: Rc<dyn Material>;
    match source.get(2).unwrap().as_str().trim() {
        "mat.bg_sky" => material = Rc::new(prefabs::materials::BgSky),
        "mat.bg_pure" => {
            let color_arg = args.get("color")
                                .expect(&format!("\"prefab.mat.bg_pure.color\" is missing (at {})!", location))
                                .as_array()
                                .expect(&format!("\"prefab.mat.bg_pure.color\" is supposed to be a Vector3D (at {})!", location));
            
            let color = json_vec3(color_arg)
                                .expect(&format!("failed to parse \"prefab.mat.bg_pure.color\" as Vector3D (at {})", location));

            material = Rc::new(prefabs::materials::BgPure::new(color));
        },
        "mat.lambertian" => {
            let albedo_arg = args.get("albedo")
                                .expect(&format!("\"prefab.mat.lambertain.albedo\" is missing (at {})!", location))
                                .as_array()
                                .expect(&format!("\"prefab.mat.lambertain.albedo\" is supposed to be a Vector3D (at {})!", location));
            
            let albedo = json_vec3(albedo_arg)
                                .expect(&format!("failed to parse \"prefab.mat.lambertain.albedo\" as Vector3D (at {})", location));
            
            material = Rc::new(prefabs::materials::Lambertian::new(albedo));
        },
        "mat.emissive" => {
            let emissive_arg = args.get("emissive")
                                .expect(&format!("\"prefab.mat.emissive.emissive\" is missing (at {})!", location))
                                .as_array()
                                .expect(&format!("\"prefab.mat.emissive.emissive\" is supposed to be a Vector3D (at {})!", location));
            
            let emissive = json_vec3(emissive_arg)
                                .expect(&format!("failed to parse \"prefab.mat.emissive.emissive\" as Vector3D (at {})", location));
            
            material = Rc::new(prefabs::materials::Emissive::new(emissive));
        },
        src => panic!("unrecognized material \"{}\" (at {})!", src, location),
    }

    return material;
}

fn emit_mesh(value: &Object, index: usize) -> Rc<dyn Hittable> {
    let location = format!("scene.entity<{}>", index);

    let src = value.get("src")
                   .expect(&format!("missing mesh source (at {})!", location))
                   .as_str()
                   .expect(&format!("\"mesh.src\" is supposed to be a Json::Str (at {})!", location))
                   .trim();
    
    let args = value.get("args")
                    .expect(&format!("missing mesh's arguments (at {})!", location))
                    .as_object()
                    .expect(&format!("\"mesh.args\" is supposed to be a Json::Object (at {})!", location));

    let src_pattern = regex!(r"(\w+):\s*([\s\w\/\\.-]+)");
    let source = src_pattern.captures(src)
                            .expect(&format!("invalid mesh source format (at {})!", location));

    let mesh: Rc<dyn Hittable>;
    match source.get(1).unwrap().as_str() {
        "prefab" => {
            match source.get(2).unwrap().as_str().trim() {
                "shape.sphere" => {
                    let radius = args.get("radius")
                                     .expect(&format!("\"prefab.shape.sphere.radius\" is missing (at {})!", location))
                                     .as_f64()
                                     .expect(&format!("\"prefab.shape.sphere.radius\" is supposed to be a float number (at {})!", location));

                    let center_arg = args.get("center")
                                        .expect(&format!("\"prefab.shape.sphere.center\" is missing (at {})!", location))
                                        .as_array()
                                        .expect(&format!("\"prefab.shape.sphere.center\" is supposed to be a Vector3D (at {})!", location));
                    
                    let center = json_vec3(center_arg)
                                .expect(&format!("\"prefab.shape.sphere.center\" is supposed to be a Vector3D (at {})!", location));
                    
                    mesh = Rc::new(prefabs::shapes::Sphere::new(center, radius));
                },
                src => panic!("unrecognized mesh source \"{}\" (at {})!", src, location)
            }
        },
        "extern" => panic!("external model is not supported yet (at {})!", location),
        src => panic!("unrecognized mesh source \"{}\" (at {})!", src, location)
    }

    return mesh;
}