use bevy::prelude::*;

use crate::asset::{SvgData, SvgDataHandles};

pub fn test_system(
    svg_data: Res<Assets<SvgData>>,
    svg_data_handles: Option<Res<SvgDataHandles>>,
) {
    if let Some(svg_data_handles) = svg_data_handles {
        for (object_label, svg_data_handle) in &svg_data_handles.handles {
            println!(
                "{:?}:::: {:?}",
                object_label,
                svg_data.get(svg_data_handle).unwrap(),
            );
        }
    }
}
