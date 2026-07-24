#[derive(Clone, Copy)]
pub struct ButteryColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Default for ButteryColor {
    fn default() -> Self {
        Self {
            r: Default::default(),
            g: Default::default(),
            b: Default::default(),
            a: 255,
        }
    }
}

pub struct ButteryGradient {
    pub stages: Vec<(f32, ButteryColor)>,
}

impl ButteryGradient {
    pub fn get_color(&self, value: f32) -> ButteryColor {
        let Some(mut source_stage) = self.stages.get(0).copied() else {
            panic!("Need to define at least one color to get color from Gradient");
        };
        let mut target_stage = None;
        for (cutoff, color) in self.stages.clone().into_iter() {
            if value < cutoff {
                source_stage = (cutoff, color);
                continue;
            }
            target_stage = Some((cutoff, color));
            break;
        }

        let Some((target_cutoff, target_color)) = target_stage else {
            panic!("Value must be larger than or equal to at least one stage defined in Gradient");
        };
        let (source_cutoff, source_color) = source_stage;
        let cutoff_diff = source_cutoff - target_cutoff;
        let blend_value = if cutoff_diff == 0.0 {
            1.0
        } else {
            (value - target_cutoff) / cutoff_diff
        };

        ButteryColor {
            r: (target_color.r as f32
                + (source_color.r as f32 - target_color.r as f32) * blend_value)
                as u8,
            g: (target_color.g as f32
                + (source_color.g as f32 - target_color.g as f32) * blend_value)
                as u8,
            b: (target_color.b as f32
                + (source_color.b as f32 - target_color.b as f32) * blend_value)
                as u8,
            a: (target_color.a as f32
                + (source_color.a as f32 - target_color.a as f32) * blend_value)
                as u8,
        }
    }
}
