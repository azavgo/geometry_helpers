use std::f64::consts::PI; 

pub struct Point {
    x: f64, 
    y: f64, 
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }
}

   pub fn radians_to_degrees(a: f64) -> f64 {
        180.0 * a / PI
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
    
    //Returns (k, b), where y = kx + b if the line is not parallel to y
    pub fn line_eq(&self) -> (f64, f64) {
        let x1 = self.p1().x(); 
        let y1 = self.p1().y(); 
        
        let x2 = self.p2().x(); 
        let y2 = self.p2().y(); 

        match x2 - x1 {
            0.0 => (f64::INFINITY, x1), 
            _   => {
                        let k = (y2 - y1) / (x2 - x1); 
                        let b = y1 - k * x1; 

                        (k, b)
                    },
        }
    }
}

//Distance between a line and a point
//Has not yet been tested yet
pub fn distance(line: &Line, point: &Point) -> f64 {
    let l_eq = line.line_eq(); 
    let x    = point.x(); 
    let y    = point.y(); 

    (l_eq.0 * x + l_eq.1 - y) * l_eq.0.atan().cos()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_line_eq() {
        let p1 = Point::new(0.0, 0.0); 
        let p2 = Point::new(2.0, 2.0); 
        let line = Line::new(&p1, &p2); 
    
        assert_eq!(line.line_eq(), (1.0, 0.0)); 
    }

    #[test]
    fn test2_line_eq() {
        let p1 = Point::new(9.0, -8.0); 
        let p2 = Point::new(-6.0, 2.0); 
        let line = Line::new(&p1, &p2); 
    
        assert_eq!(line.line_eq(), (-2.0/3.0, -2.0)); 
    }

    #[test]
    fn test3_line_eq() {
        let p1 = Point::new(2.0, -5.0); 
        let p2 = Point::new(2.0, 7.0); 
        let line = Line::new(&p1, &p2); 
    
        assert_eq!(line.line_eq(), (f64::INFINITY, 2.0)); 
    }

    #[test]
    fn test1_radians_to_degrees() {
        let result = radians_to_degrees(PI / 3.0);
        assert!((result - 60.0).abs() < 1e-6);
    }

    #[test]
    fn test2_radians_to_degrees() {
        let result = radians_to_degrees(PI / 6.0);
        assert!((result - 30.0).abs() < 1e-6);
    }

    #[test]
    fn test1_point_x() {
        let p = Point::new(1.0, 5.0); 
        assert_eq!(p.x(), 1.0);
    }

     #[test]
    fn test1_point_y() {
        let p = Point::new(-1.2, 3.9); 
        assert_eq!(p.y(), 3.9);
    }

}
