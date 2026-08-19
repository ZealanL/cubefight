use glam::{DMat3, DVec2, DVec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Angle {
    pub yaw: f64,
    pub pitch: f64,
}

impl Angle {
    pub const ZERO: Angle = Angle {
        pitch: 0.0,
        yaw: 0.0,
    };

    pub const fn new(yaw: f64, pitch: f64) -> Self {
        Self { pitch, yaw }.fixed()
    }

    pub fn from_forward(forward: DVec3) -> Self {
        let yaw = f64::atan2(-forward.x, forward.z).to_degrees();
        let h_len = f64::hypot(forward.x, forward.z);
        let pitch = -f64::atan2(forward.y, h_len).to_degrees();
        Angle { pitch, yaw }
    }

    pub fn get_forward(self) -> DVec3 {
        let cy = self.yaw.to_radians().cos();
        let sy = self.yaw.to_radians().sin();
        let cp = -self.pitch.to_radians().cos();
        let sp = -self.pitch.to_radians().sin();

        DVec3::new(sy * cp, -sp, -cy * cp)
    }

    pub fn get_forward_yaw_only(self) -> DVec3 {
        let cy = self.yaw.to_radians().cos();
        let sy = self.yaw.to_radians().sin();

        DVec3::new(-sy, 0.0, cy)
    }

    pub fn get_right(self) -> DVec3 {
        let cy = self.yaw.to_radians().cos();
        let sy = self.yaw.to_radians().sin();
        DVec3::new(-cy, 0.0, -sy)
    }

    pub fn to_mat(self) -> DMat3 {
        let forward = self.get_forward();
        let right = self.get_right();
        let up = forward.cross(right);
        DMat3::from_cols(forward, right, up)
    }

    pub const fn fixed(self) -> Angle {
        let mut norm_yaw = self.yaw % 360.0;
        if norm_yaw >= 180.0 {
            norm_yaw -= 360.0;
        } else if norm_yaw < -180.0 {
            norm_yaw += 360.0;
        }

        let norm_pitch = self.pitch.clamp(-90.0, 90.0);
        Angle {
            yaw: norm_yaw,
            pitch: norm_pitch,
        }
    }

    pub const fn with_yaw(self, yaw: f64) -> Self {
        Self { yaw, ..self }
    }
    pub const fn with_pitch(self, pitch: f64) -> Self {
        Self { pitch, ..self }
    }

    pub const fn to_vec2(self) -> DVec2 {
        DVec2::new(self.yaw, self.pitch)
    }

    pub fn from_vec2(v: DVec2) -> Self {
        Self::new(v.x, v.y)
    }

    pub fn rotate_vec_yaw_only(self, vec: DVec3) -> DVec3 {
        let yaw_rad = self.yaw.to_radians();
        let cy = yaw_rad.cos();
        let sy = yaw_rad.sin();

        DVec3::new(
            vec.x * cy - vec.z * sy,
            vec.y,
            vec.x * sy + vec.z * cy,
        )
    }

    pub fn unrotate_vec_yaw_only(self, vec: DVec3) -> DVec3 {
        let neg_yaw = Angle { yaw: -self.yaw, pitch: 0.0 };
        neg_yaw.rotate_vec_yaw_only(vec)
    }

    pub fn rotate_vec(self, vec: DVec3) -> DVec3 {
        self.to_mat() * vec
    }

    pub fn unrotate_vec(self, vec: DVec3) -> DVec3 {
        self.to_mat().transpose() * vec
    }
}

impl std::ops::Add<Angle> for Angle {
    type Output = Angle;

    fn add(self, rhs: Angle) -> Self::Output {
        Angle::new(self.yaw + rhs.yaw, self.pitch + rhs.pitch)
    }
}
impl std::ops::Sub<Angle> for Angle {
    type Output = Angle;
    fn sub(self, rhs: Angle) -> Self::Output {
        Angle::new(self.yaw - rhs.yaw, self.pitch - rhs.pitch)
    }
}

impl std::ops::AddAssign<Angle> for Angle {
    fn add_assign(&mut self, rhs: Angle) {
        *self = *self + rhs
    }
}
impl std::ops::SubAssign<Angle> for Angle {
    fn sub_assign(&mut self, rhs: Angle) {
        *self = *self - rhs
    }
}
