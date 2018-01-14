
#[derive(Debug)]
pub struct ObjScene {
    pub vert_pos : Vec<[f32;4]>,
    pub vert_norm : Vec<[f32;3]>,
    pub vert_tx : Vec<[f32;2]>,
    pub grps : Vec<ObjGroup>
}

#[derive(Debug)]
pub struct ObjGroup {
    pub name : String,
    pub mat : Option<String>,
    pub faces : Vec<ObjFace>
}

#[derive(Debug)]
pub struct ObjFace {
    pub corners : Vec<ObjFaceCorner>
}

#[derive(Debug)]
pub struct ObjFaceCorner {
    pub smooth_i : Option<u32>,
    pub pos_i : u32,
    pub norm_i : u32,
    pub tx_i : u32
}
