

pub struct Vec2 {
    x: f64, 
    y: f64, 
}

type Point = Vec2;

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn dot(v1: &Self, v2: &Self) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y()
    }

    pub fn length(&self) -> f64 {
        Self::dot(&self, &self).sqrt()
    }

    pub fn add(v1: &Self, v2: &Self) -> Self {
        let x = v1.x() + v2.x(); 
        let y = v1.y() + v2.y(); 
        Self::new(x, y)
    }

    pub fn sub(v1: &Self, v2: &Self) -> Self {
        let x = v1.x() - v2.x(); 
        let y = v1.y() - v2.y(); 
        Self::new(x, y)
    }
}

pub struct Triangle<'a>  {
    p1: &'a Point,
    p2: &'a Point, 
    p3: &'a Point,
}

impl <'a> Triangle<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point, p3: &'a Point) -> Self {
        Self { p1, p2, p3 }
    }

    pub fn p1(&self) -> &'a Point {
        self.p1
    }

    pub fn p2(&self) -> &'a Point {
        self.p2
    }

    pub fn p3(&self) -> &'a Point {
        self.p3
    }
}

pub struct Line<'a> {
    p1: &'a Point,
    p2: &'a Point,
}

impl <'a> Line<'a> {
    pub fn new(p1: &'a Point, p2: &'a Point) -> Self {
        Self { p1, p2 }
    }

    pub fn p1(&self) -> &'a Point {
        self.p1
    }

    pub fn p2(&self) -> &'a Point {
        self.p2
    }

    pub fn point_line_distance(&self, p: &'a Point) -> f64 {
        //Finding line equation coefficients k and b: y = kx + b
        let k = (&self.p2().y() - &self.p1().y())/(&self.p2().x() - &self.p1().x()); 
        let b = &self.p1().y() - k * &self.p1().x();      
        //Unimplemented
        0.0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_vec2_y() {
        let v = Vec2::new(1.0, 5.0); 
        assert_eq!(v.y(), 5.0);
    }

    #[test]
    fn test2_point_x() {
        let p = Point::new(1.0, 5.0); 
        assert_eq!(p.x(), 1.0);
    }

    #[test]
    fn test3_vec2_dot() {
        let v1 = Vec2::new(1.0, 5.0);
        let v2 = Vec2::new(-2.0, -3.0); 
        assert_eq!(Vec2::dot(&v1, &v2), -17.0);
    }

    #[test]
    fn test4_vec2_length() {
        let v = Vec2::new(-4.0, 0.0); 
        assert_eq!(v.length(), 4.0);
    }

}
