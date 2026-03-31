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

    //This method is only for Vec2
    //
    //pub fn length(&self) -> f64 {
    //    dot(&self, &self).sqrt()
    //}

    //This method is only for Vec2
    //alpha is an angle in radians the vector is being turned 
    //pub fn turn(v: &Self, alpha: f64) -> Self {
    //    let x0 = v.x(); 
    //    let y0 = v.y(); 
    //    let r = v.length();
    //    let delta = (y0/x0).atan() - alpha; 
    //    let x1 = r * delta.cos(); 
    //    let y1 = r * delta.sin(); 
    //    Self::new(x1, y1)
    //}
}
    //add() function is only for Vec2
    //pub fn add(v1: &Vec2, v2: &Vec2) -> Vec2 {
    //    let x = v1.x() + v2.x(); 
    //    let y = v1.y() + v2.y(); 
    //    Vec2::new(x, y)
    //}
    //sub() function is only for Vec2
    //pub fn sub(v1: &Vec2, v2: &Vec2) -> Vec2 {
    //    let x = v1.x() - v2.x(); 
    //    let y = v1.y() - v2.y(); 
    //    Vec2::new(x, y)
    //}
    
    //dot() function is only for Vec2
    //pub fn dot(v1: &Vec2, v2: &Vec2) -> f64 {
    //    v1.x() * v2.x() + v1.y() * v2.y()
    //}

    //to_origin() function is only for Vec2
    //Translates a vector with a beginning p1 and ending p2 to Origin  
    //pub fn to_origin(p1: &Point, p2: &Point) -> Vec2 {
    //    let x = p2.x() - p1.x(); 
    //    let y = p2.y() - p1.y(); 
    //    Vec2::new(x, y)
    //}

    pub struct Angle {
        radians: f64, //In radians
    }

    impl Angle {
        pub fn new(radians: f64) -> Self {
            Self { radians }
        }

        //Returns radians
        pub fn radians(&self) -> f64 {
            self.radians
        }

        //Returns degrees
        pub fn degrees(&self) -> f64 {
            let radians = self.radians();
            180.0 * radians / PI
        }

        pub fn angle(degrees: f64) -> Self {
            Self::new(PI * degrees / 180.0)
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

    //This has to be completely re-develop
    //Test does not pass area() simply returns 0.0
    //pub fn area(&self) -> f64 {
    //    let a1 = self.p1(); 
    //    let b1 = self.p2();
    //    let c1 = self.p2();
    //
    //    let a = to_origin(a1, b1); 
    //    let b = to_origin(a1, c1); 
    //    let la = a.length(); 
    //    let lb = b.length(); 
    //    let dot_ab = dot(&a, &b); 
    //    0.5 * la * lb * (1.0 - dot_ab * dot_ab / (la * la * lb * lb)).sqrt()
    //}
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

    //This needs to be completely re-developed
    //pub fn point_line_distance(&self, p: &'a Point) -> f64 {
        //Checking whether given line is parallel to either x or y axes
        //(1) Is the given line parallel to x axis 
        //let v = to_origin(&self.p1(), &self.p2()); 
        //let i = Vec2::new(1.0, 0.0);
        //let j = Vec2::new(0.0, 1.0);
        //if dot(&v, &i) = 0 {v - parallel y axis} or dot (&v, &j) = 0 {v - parallel x - axis} else
        //continue on finding the line equation
        //Finding line equation coefficients k and b: y = kx + b
        //let k = (&self.p2().y() - &self.p1().y())/(&self.p2().x() - &self.p1().x()); 
        //let b = &self.p1().y() - k * &self.p1().x();      
        //Translating the coordinates y' = y - b of the point p
        //let p_1 = Point::new(p.x(), p.y() - b);
        //Turn vector p_1 to -atan(k) radians
        //let p_2 = Vec2::turn(&p_1, -k.atan());
        //Distance from the point to the line will be the absolute value of the y coordinate of the point
        //p_2.y().abs()
    //}
}


#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn test1_triangle_area() {
    //    let p1 = Point::new(0.0, 0.0); 
    //    let p2 = Point::new(0.0, 2.0); 
    //    let p3 = Point::new(2.0, 0.0); 
    //
    //    let t = Triangle::new(&p1, &p2, &p3);
    //
    //    assert_eq!(t.area(), 2.0); 
    //}

    #[test]
    fn test1_angle() {
        let degrees = 45.0; 
        let angle = Angle::angle(degrees); 
        let radians = angle.radians(); 
        assert_eq!(radians, PI / 4.0); 
    }

    //#[test]
    //fn test1_vec2_y() {
    //    let v = Vec2::new(1.0, 5.0); 
    //    assert_eq!(v.y(), 5.0);
    //}

    #[test]
    fn test2_point_x() {
        let p = Point::new(1.0, 5.0); 
        assert_eq!(p.x(), 1.0);
    }

    //#[test]
    //fn test3_vec2_dot() {
    //    let v1 = Vec2::new(1.0, 5.0);
    //    let v2 = Vec2::new(-2.0, -3.0); 
    //    assert_eq!(dot(&v1, &v2), -17.0);
    //}

    //#[test]
    //fn test4_vec2_length() {
    //    let v = Vec2::new(-4.0, 0.0); 
    //    assert_eq!(v.length(), 4.0);
    //}

}
