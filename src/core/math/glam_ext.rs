use glam::{DVec3, IVec3};

pub trait BlockPosExt {
    fn center_pos(&self) -> DVec3;
    fn top_center_pos(&self) -> DVec3;
    fn bottom_center_pos(&self) -> DVec3;
}
impl BlockPosExt for IVec3 {
    fn center_pos(&self) -> DVec3 {
        self.as_dvec3() + DVec3::new(0.5, 0.5, 0.5)
    }
    fn top_center_pos(&self) -> DVec3 {
        self.as_dvec3() + DVec3::new(0.5, 1.0, 0.5)
    }
    fn bottom_center_pos(&self) -> DVec3 {
        self.as_dvec3() + DVec3::new(0.5, 0.0, 0.5)
    }
}

pub trait PosExt {
    fn to_block_pos(&self) -> IVec3;
}
impl PosExt for DVec3 {
    fn to_block_pos(&self) -> IVec3 {
        self.floor().as_ivec3()
    }
}

