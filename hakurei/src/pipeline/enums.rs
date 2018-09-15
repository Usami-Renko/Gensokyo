
use ash::vk;
use utility::marker::VulkanEnum;

/// PolygonMode specifies the method of rasterization for polygons.
///
/// These modes affect only the final rasterization of polygons:
/// in particular, a polygon’s vertices are shaded and the polygon is clipped and possibly culled before these modes are applied.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PolygonMode {
    /// Fill specifies that polygon vertices are drawn as points.
    Fill,
    /// Line specifies that polygon edges are drawn as line segments.
    Line,
    /// Point specifies that polygons are rendered using the polygon rasterization rules in this section.
    Point
}

impl VulkanEnum for PolygonMode {
    type EnumType = vk::PolygonMode;

    fn value(&self) -> Self::EnumType {
        match *self {
            | PolygonMode::Fill  => vk::PolygonMode::Fill,
            | PolygonMode::Line  => vk::PolygonMode::Line,
            | PolygonMode::Point => vk::PolygonMode::Point,
        }
    }
}

// TODO: Add description for PrimitiveTopology.
/// Primitive topology determines how consecutive vertices are organized into primitives, and determines the type of primitive that is used at the beginning of the graphics pipeline.
///
/// The effective topology for later stages of the pipeline is altered by tessellation or geometry shading (if either is in use) and depends on the execution modes of those shaders.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

impl VulkanEnum for PrimitiveTopology {
    type EnumType = vk::PrimitiveTopology;

    fn value(&self) -> Self::EnumType {
        match *self {
            | PrimitiveTopology::PointList                  => vk::PrimitiveTopology::PointList,
            | PrimitiveTopology::LineList                   => vk::PrimitiveTopology::LineList,
            | PrimitiveTopology::LineStrip                  => vk::PrimitiveTopology::LineStrip,
            | PrimitiveTopology::TriangleList               => vk::PrimitiveTopology::TriangleList,
            | PrimitiveTopology::TriangleStrip              => vk::PrimitiveTopology::TriangleStrip,
            | PrimitiveTopology::TriangleFan                => vk::PrimitiveTopology::TriangleFan,
            | PrimitiveTopology::LineListWithAdjacency      => vk::PrimitiveTopology::LineListWithAdjacency,
            | PrimitiveTopology::LineStripWithAdjacency     => vk::PrimitiveTopology::LineStripWithAdjacency,
            | PrimitiveTopology::TriangleListWithAdjacency  => vk::PrimitiveTopology::TriangleListWithAdjacency,
            | PrimitiveTopology::TriangleStripWithAdjacency => vk::PrimitiveTopology::TriangleStripWithAdjacency,
            | PrimitiveTopology::PatchList                  => vk::PrimitiveTopology::PatchList,
        }
    }
}
