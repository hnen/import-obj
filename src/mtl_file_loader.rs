use error::*;

use std::collections::HashMap;

pub struct ObjMaterialCollection {
    pub materials : HashMap<String, ObjMaterial>
}

#[derive(Debug, Clone)]
pub struct ObjMaterial {
    pub name : String,
    pub specular_exponent : Option<f32>,
    pub optical_density : Option<f32>,
    pub transparency : Option<f32>,
    pub transmission_filter : Option<[f32;3]>,
    pub illumination_model : Option<u32>,

    pub ambient_color : Option<[f32;3]>,
    pub diffuse_color : Option<[f32;3]>,
    pub specular_color : Option<[f32;3]>,
    pub emissive_color : Option<[f32;3]>,

    pub ambient_map_filename : Option<String>,
    pub diffuse_map_filename : Option<String>,
    pub transparency_map_filename : Option<String>,
    pub bump_map_filename : Option<String>,
}

pub fn mtl_load(mtl_file : &str) -> Result<ObjMaterialCollection> {
    let mut materials = HashMap::new();

    let mut mat : Option<ObjMaterial> = None;

    for line in mtl_file.lines() {
        let line = line.trim();
        let line = line.split("#").next().unwrap();
        let mut words = line.split_whitespace();
        try!(match words.next() {
            Some("newmtl") => {
                if let Some(tmat) = mat {
                    materials.insert(tmat.name.clone(), tmat);
                }
                let newmtl_name = match words.next() {
                    Some(name) => name,
                    None => return Err(From::from("No parameter in newmtl tag."))
                };
                mat = Some(ObjMaterial {
                    name: String::from(newmtl_name),
                    specular_exponent: None, optical_density: None, transparency: None,
                    transmission_filter: None, ambient_color: None, diffuse_color: None, emissive_color: None,
                    specular_color: None, illumination_model: None, ambient_map_filename: None,
                    diffuse_map_filename: None, transparency_map_filename: None,
                    bump_map_filename: None
                });
                Ok(())
            }
            Some("Ns") => {
                read_f32(&mut mat, words, |ref mut m, f| {
                    m.specular_exponent = Some(f);
                })
            }
            Some("Ni") => {
                read_f32(&mut mat, words, |ref mut m, f| {
                    m.optical_density = Some(f);
                })
            }
            Some("d") => {
                read_f32(&mut mat, words, |ref mut m, f| {
                    m.transparency = Some(1.0 - f);
                })
            }
            Some("Tr") => {
                read_f32(&mut mat, words, |ref mut m, f| {
                    m.transparency = Some(f);
                })
            }
            Some("Tf") => {
                read_vec3(&mut mat, words, |ref mut m, v| {
                    m.transmission_filter = Some(v);
                })
            }
            Some("illum") => {
                read_u32(&mut mat, words, |ref mut m, i| {
                    m.illumination_model = Some(i);
                })
            }
            Some("Ka") => {
                read_vec3(&mut mat, words, |ref mut m, v| {
                    m.ambient_color = Some(v);
                })
            }
            Some("Kd") => {
                read_vec3(&mut mat, words, |ref mut m, v| {
                    m.diffuse_color = Some(v);
                })
            }
            Some("Ke") => {
                read_vec3(&mut mat, words, |ref mut m, v| {
                    m.emissive_color = Some(v);
                })
            }
            Some("Ks") => {
                read_vec3(&mut mat, words, |ref mut m, v| {
                    m.specular_color = Some(v);
                })
            }
            Some("map_Ka") => {
                read_str(&mut mat, words, |ref mut m, s| {
                    m.ambient_map_filename = Some(s);
                })
            }
            Some("map_Kd") => {
                read_str(&mut mat, words, |ref mut m, s| {
                    m.diffuse_map_filename = Some(s);
                })
            }
            Some("map_d") => {
                read_str(&mut mat, words, |ref mut m, s| {
                    m.transparency_map_filename = Some(s);
                })
            }
            Some("map_bump") => {
                read_str(&mut mat, words, |ref mut m, s| {
                    m.bump_map_filename = Some(s);
                })
            }
            Some("bump") => {
                read_str(&mut mat, words, |ref mut m, s| {
                    m.bump_map_filename = Some(s);
                })
            }
            Some(word) => Err(From::from(format!("Unrecognized word {:?}", word))),
            None => Ok(()) // empty line
        });
    }

    if let Some(tmat) = mat {
       materials.insert(tmat.name.clone(), tmat);
    }

    Ok(ObjMaterialCollection{
        materials: materials
    })
}


fn read_u32<'a, I, F>(mat : &mut Option<ObjMaterial>, mut words : I, setter : F) -> Result<()> where I : Iterator<Item=&'a str>, F : FnOnce(&mut ObjMaterial, u32) {
    let ref mut m = match mat {
        &mut Some(ref mut m) => m,
        &mut None => return Err(From::from("Found int value before newmtl."))
    };
    let i : u32 = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    setter(m, i);
    Ok(())
}

fn read_f32<'a, I, F>(mat : &mut Option<ObjMaterial>, mut words : I, setter : F) -> Result<()> where I : Iterator<Item=&'a str>, F : FnOnce(&mut ObjMaterial, f32) {
    let ref mut m = match mat {
        &mut Some(ref mut m) => m,
        &mut None => return Err(From::from("Found float value before newmtl."))
    };
    let f : f32 = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    setter(m, f);
    Ok(())
}

fn read_vec3<'a, I, F>(mat : &mut Option<ObjMaterial>, mut words : I, setter : F) -> Result<()> where I : Iterator<Item=&'a str>, F : FnOnce(&mut ObjMaterial, [f32;3]) {
    let ref mut m = match mat {
        &mut Some(ref mut m) => m,
        &mut None => return Err(From::from("Found vec3 value before newmtl."))
    };
    let r : f32 = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    let g : f32 = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    let b : f32 = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    setter(m, [r,g,b] );
    Ok(())
}

fn read_str<'a, I, F>(mat : &mut Option<ObjMaterial>, mut words : I, setter : F) -> Result<()> where I : Iterator<Item=&'a str>, F : FnOnce(&mut ObjMaterial, String) {
    let ref mut m = match mat {
        &mut Some(ref mut m) => m,
        &mut None => return Err(From::from("Found vec3 value before newmtl."))
    };
    let st : String = words
        .next().ok_or(ObjError("Expected paramater".to_string()))?
        .parse()?;
    setter(m, st.replace("\\", "/"));
    Ok(())
}

