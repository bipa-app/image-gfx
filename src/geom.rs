use crate::types::Angle;

pub fn inside_arc(point: (f64, f64), start: Angle, end: Angle) -> bool {
    let (x, y) = point;

    let d = Angle::Degrees(y.atan2(x).to_degrees())
        .normalized()
        .to_degrees();

    d >= start.to_degrees() && d <= end.to_degrees()
}
