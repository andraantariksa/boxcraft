use nalgebra::Vector2;
use voronator::delaunator::Point;
use voronator::VoronoiDiagram;

pub struct Voronoi {
    diagram: VoronoiDiagram<Point>,
    bounding_box_top_left: Vector2<f32>,
    bounding_box_bottom_right: Vector2<f32>,
}

impl Voronoi {
    pub fn new(
        bounding_box_top_left: Vector2<f32>,
        bounding_box_bottom_right: Vector2<f32>,
        points: &[Vector2<f32>],
    ) -> Self {
        let diagram = VoronoiDiagram::from_tuple(
            &to_voronator_tuple(&bounding_box_top_left),
            &to_voronator_tuple(&bounding_box_bottom_right),
            &points
                .into_iter()
                .map(|point| to_voronator_tuple(point))
                .collect::<Vec<(f64, f64)>>()[..],
        )
        .unwrap();
        Self {
            diagram,
            bounding_box_top_left,
            bounding_box_bottom_right,
        }
    }

    pub fn relax(&mut self) {
        // self.diagram.cells().into_iter().map(|a| a.)
    }
}

fn to_voronator_tuple(point: &Vector2<f32>) -> (f64, f64) {
    (point.x as f64, point.y as f64)
}

fn from_voronator_vector(point: Point) -> Vector2<f32> {
    Vector2::new(point.x as f32, point.y as f32)
}
