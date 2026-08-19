use glam::{DVec3, IVec3};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct DBox3 {
    pub min: DVec3,
    pub max: DVec3,
}

impl DBox3 {
    pub const ZERO: DBox3 = DBox3 {
        min: DVec3::ZERO,
        max: DVec3::ZERO,
    };

    pub fn new(p1: DVec3, p2: DVec3) -> Self {
        Self {
            min: p1.min(p2),
            max: p1.max(p2),
        }
    }

    pub fn from_block_pos(block_pos: IVec3) -> Self {
        let min = block_pos.as_dvec3();
        Self::new(min, min + 1.0)
    }

    pub fn contains(&self, point: DVec3) -> bool {
        point.cmple(self.max).all() && point.cmpge(self.min).all()
    }

    pub fn inflate_by(&self, amount: DVec3) -> Self {
        let half_size = self.size() / 2.0;
        let amount = amount.clamp(-half_size, half_size);
        Self {
            min: self.min - amount,
            max: self.max + amount,
        }
    }

    pub fn grow_with_delta(&self, delta: DVec3) -> Self {
        Self {
            min: self.min.min(self.min + delta),
            max: self.max.max(self.max + delta),
        }
    }

    pub fn offset(&self, offset: DVec3) -> Self {
        Self {
            min: self.min + offset,
            max: self.max + offset,
        }
    }

    pub fn size(&self) -> DVec3 {
        self.max - self.min
    }

    pub fn center(&self) -> DVec3 {
        (self.min + self.max) / 2.0
    }

    pub fn clamp_within(&self, point: DVec3) -> DVec3 {
        point.clamp(self.min, self.max)
    }

    pub fn combine_with(&self, other: &DBox3) -> DBox3 {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }

    pub fn overlaps(&self, other: &Self) -> bool {
        self.min.cmple(other.max).all() && self.max.cmpge(other.min).all()
    }

    pub fn intersection(&self, other: &Self) -> Option<Self> {
        let min = self.min.max(other.min);
        let max = self.max.min(other.max);

        if min.cmple(max).all() {
            Some(Self { min, max })
        } else {
            None
        }
    }
}

impl Display for DBox3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("DBox3({}, {})", self.min, self.max))
    }
}