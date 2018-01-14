use error::*;
use obj::*;

use obj_renderable::obj_to_renderable;
use obj_renderable::ObjRenderable;

pub fn obj_load(wf_file : &str) -> Result<Vec<ObjRenderable>> {
    obj_to_renderable(&obj_load_raw(wf_file)?)
}

fn obj_load_raw(wf_file : &str) -> Result<ObjScene> {
    let mut scene = ObjScene {
        vert_pos: Vec::new(),
        vert_norm: Vec::new(),
        vert_tx: Vec::new(),
        grps: Vec::new()
    };

    let mut group = ObjGroup {
        name: String::new(), mat: None, faces: Vec::new()
    };
    let mut smooth_i : Option<u32> = None;

    for line in wf_file.lines() {
        let (n_scene, n_group, n_smooth_i) = process_line(line, scene, group, smooth_i)?;
        scene = n_scene;
        group = n_group;
        smooth_i = n_smooth_i;
    }

    if group.faces.len() > 0 {
        scene.grps.push(group)
    }

    println!("vertices: pos: {}, tx: {}, norm {}, grps: {}", scene.vert_pos.len(), scene.vert_tx.len(), scene.vert_norm.len(), scene.grps.len());

    Ok(scene)
}

fn process_line(line : &str, mut scene : ObjScene, mut group : ObjGroup, mut smooth_i : Option<u32>) -> Result<(ObjScene, ObjGroup, Option<u32>)> {
    let line = line.trim();
    let line = line.split("#").next().unwrap(); // strip comment
    let mut words = line.split_whitespace();
    match words.next() {
        Some("v") => {
            scene.vert_pos.push(parse_v(&mut words)?);
        }
        Some("vt") => {
            scene.vert_tx.push(parse_vt(&mut words)?);
        }
        Some("vn") => {
            scene.vert_norm.push(parse_vn(&mut words)?);
        }
        Some("usemtl") => {
            group.mat = Some(String::from(words.next().ok_or(ObjError("Expected paramteter for usemtl".to_string()))?))
        }
        Some("g") => {
            if group.faces.len() > 0 {
                scene.grps.push(group)
            }
            let group_name = words.next().ok_or(ObjError("Expected group name".to_string()))?;
            group = ObjGroup {
                name: String::from(group_name), mat: None, faces: Vec::new()
            }
        }
        Some("s") => {
            match words.next() {
                Some("off") => smooth_i = None,
                Some(i_str) => smooth_i = Some(i_str.parse()?),
                _ => smooth_i = None
            }
        }
        Some("f") => {
            group.faces.push(parse_f(&mut words, &smooth_i)?);
        }
        Some("#") => (),
        Some(_s) => {
            //println!("Unknown obj tag: {}", _s);
        }
        _ => ()
    }
    Ok((scene, group, smooth_i))
}

fn parse_v<'a, I>(words : &mut I) -> Result<[f32;4]> where I : Iterator<Item=&'a str> {
    let x : f32 = words
        .next().ok_or(ObjError("Expected paramteter for v".to_string()))?
        .parse()?;
    let y : f32 = words
        .next().ok_or(ObjError("Expected paramteter for v".to_string()))?
        .parse()?;
    let z : f32 = words
        .next().ok_or(ObjError("Expected paramteter for v".to_string()))?
        .parse()?;
    let w : f32 = match words.next() {
        Some(w_str) => w_str.parse()?,
        _ => 1.0
    };
    Ok([x,y,z,w])
}

fn parse_vt<'a, I>(words : &mut I) -> Result<[f32;2]> where I : Iterator<Item=&'a str> {
    let x : f32 = words
        .next().ok_or(ObjError("Expected parameter for vt".to_string()))?
        .parse()?;
    let y : f32 = words
        .next().ok_or(ObjError("Expected parameter for vt".to_string()))?
        .parse()?;
    Ok([x,y])
}

fn parse_vn<'a, I>(words : &mut I) -> Result<[f32;3]> where I : Iterator<Item=&'a str> {
    let x : f32 = words
        .next().ok_or(ObjError("Expected parameter for vn".to_string()))?
        .parse()?;
    let y : f32 = words
        .next().ok_or(ObjError("Expected parameter for vn".to_string()))?
        .parse()?;
    let z : f32 = words
        .next().ok_or(ObjError("Expected parameter for vn".to_string()))?
        .parse()?;
    Ok([x,y,z])
}


fn parse_f<'a,I>(words : &mut I, smooth_i : &Option<u32>) -> Result<ObjFace> where I : Iterator<Item=&'a str> {
    let mut face = ObjFace {
        corners: Vec::new()
    };
    for facecorner in words {
        let mut els = facecorner.split("/");
        let pos_i : u32 = els
            .next().ok_or(ObjError("Expected parameter for f".to_string()))?
            .parse()?;
        let tx_i : u32 = match els.next() {
            Some(tx_i_str) => match tx_i_str.parse() {
                Ok(i) => i,
                Err(_) => pos_i
            },
            None => pos_i
        };
        let norm_i : u32 = match els.next() {
            Some(norm_i_str) => match norm_i_str.parse() {
                Ok(i) => i,
                Err(_) => pos_i
            },
            None => pos_i
        };
        face.corners.push(ObjFaceCorner {
            smooth_i: smooth_i.clone(), pos_i: pos_i, tx_i: tx_i, norm_i: norm_i
        });
    }
    Ok(face)
}







