
use std::collections::HashMap;
use obj::*;
use error::*;

#[derive(Debug)]
pub struct ObjRenderable {
    pub name : String,
    pub mat_name : Option<String>,
    pub v_pos : Vec<[f32;3]>,
    pub v_norm : Vec<[f32;3]>,
    pub v_tx : Vec<[f32;2]>,
    pub inds : Vec<[u32;3]>
}

pub fn obj_to_renderable(scene : &ObjScene) -> Result<Vec<ObjRenderable>> {
    let mut objs = Vec::new();

    for grp in &scene.grps {
        let mut ind_lookup = HashMap::new();
        let mut v_pos = Vec::new();
        let mut v_norm = Vec::new();
        let mut v_tx = Vec::new();
        let mut inds = Vec::new();
        for face in &grp.faces {
            for i in 1..(face.corners.len()-1) {
                let mut tri_inds = Vec::with_capacity(3);
                for c in &[&face.corners[0], &face.corners[i], &face.corners[i+1]] {
                    let (n_ind_lookup, n_v_pos, n_v_norm, n_v_tx, n_inds) = add_corner(ind_lookup, v_pos, v_norm, v_tx, tri_inds, c.pos_i, c.norm_i, c.tx_i, &scene)?;
                    ind_lookup = n_ind_lookup;
                    v_pos = n_v_pos;
                    v_norm = n_v_norm;
                    v_tx = n_v_tx;
                    tri_inds = n_inds;
                }
                inds.push([tri_inds[0], tri_inds[1], tri_inds[2]]);
            }
        }

        objs.push(ObjRenderable {
            name: grp.name.clone(),
            mat_name: grp.mat.clone(),
            v_pos: v_pos,
            v_norm: v_norm,
            v_tx : v_tx,
            inds : inds
        });
    }

    Ok(objs)
}

fn add_corner(mut ind_lookup : HashMap<(u32,u32,u32),u32>, mut v_pos : Vec<[f32;3]>, mut v_norm : Vec<[f32;3]>, mut v_tx : Vec<[f32;2]>, mut inds : Vec<u32>, pos_i : u32, norm_i : u32, tx_i : u32, scene : &ObjScene) -> Result<(HashMap<(u32,u32,u32),u32>, Vec<[f32;3]>, Vec<[f32;3]>, Vec<[f32;2]>, Vec<u32>)> {
    let v_key = (pos_i, norm_i, tx_i);
    if !ind_lookup.contains_key(&v_key) {
        let ind = v_pos.len() as u32;
        let pos4 = &scene.vert_pos[(pos_i-1) as usize];
        v_pos.push([pos4[0], pos4[1], pos4[2]]);
        v_norm.push(scene.vert_norm[(norm_i-1) as usize].clone());
        if scene.vert_tx.len() > 0 {
            v_tx.push(scene.vert_tx[(tx_i-1) as usize].clone());
        }
        inds.push(ind);
        ind_lookup.insert(v_key, ind);
    } else {
        let ind = ind_lookup
            .get(&v_key).ok_or(ObjError("Invalid mesh".to_string()))?;
        inds.push(ind.clone());
    }
    Ok((ind_lookup, v_pos, v_norm, v_tx, inds))
}


