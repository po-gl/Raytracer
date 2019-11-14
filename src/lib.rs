
//impl PartialEq for f64 {
//    fn eq(&self, other: &Self) -> bool {
//        ((self - other) < FLOAT_THRESHOLD && (self - other) >= 0.0) || ((other - self) < FLOAT_THRESHOLD && (other - self) >= 0.0)
//        // This shaves off a few ms compared to using .abs()
////        (((self.x - other.x) < FLOAT_THRESHOLD && (self.x - other.x) >= 0.0) || ((other.x - self.x) < FLOAT_THRESHOLD && (other.x - self.x) >= 0.0))
////            && (((self.y - other.y) < FLOAT_THRESHOLD && (self.y - other.y) >= 0.0) || ((other.y - self.y) < FLOAT_THRESHOLD && (other.y - self.y) >= 0.0))
////            && (((self.z - other.z) < FLOAT_THRESHOLD && (self.z - other.z) >= 0.0) || ((other.z - self.z) < FLOAT_THRESHOLD && (other.z - self.z) >= 0.0))
////            && (((self.w - other.w) < FLOAT_THRESHOLD && (self.w - other.w) >= 0.0) || ((other.w - self.w) < FLOAT_THRESHOLD && (other.w - self.w) >= 0.0))
//    }
//}
