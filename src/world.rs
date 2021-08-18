use crate::renderer::{Renderable, Renderer};
use wgpu::RenderPass;
use crate::pipeline::Type;
use std::collections::HashMap;
use crate::graphics::Graphics;
use crate::region::Region;
use crate::coordinate::{Coord3D, RegionCoord3D};

pub struct World {
    regions: HashMap<RegionCoord3D, Region>,
    active_regions: Vec<RegionCoord3D>
}

impl Renderable for World {
    fn render<'a>(&'a self, pass: &mut RenderPass<'a>, renderer: &'a Renderer) {
        pass.set_pipeline(&renderer.pipelines.get(&Type::Main).unwrap().pipeline);

        for p in &self.active_regions {
            self.regions.get(p).unwrap().render(pass);
        }
    }
}

impl World {
    pub fn new(graphics: &Graphics) -> Self {
        let regions = HashMap::new();
        let active_regions = Vec::new();
        Self {
            regions,
            active_regions
        }
    }

    pub fn add_region(&mut self, coord: Coord3D) {
        self.regions.insert(coord.to_region_coord(), Region::new(coord.to_region_coord()));
    }

    pub fn get_region(&mut self, coord: &Coord3D) -> &Region {
        &self.regions.get(&coord.to_region_coord()).unwrap()
    }
}
