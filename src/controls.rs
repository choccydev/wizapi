use crate::model::{Color, CommonControl, DeviceClass, Lamp, LampControl};

impl CommonControl for DeviceClass {
    fn turn_on(&mut self) {
        /* ... */
        todo!()
    }
    fn turn_off(&mut self) {
        /* ... */
        todo!()
    }
    fn toggle(&mut self) {
        /* ... */
        todo!()
    }
}

impl CommonControl for Lamp {
    fn turn_on(&mut self) {
        /* ... */
        todo!()
    }
    fn turn_off(&mut self) {
        /* ... */
        todo!()
    }
    fn toggle(&mut self) {
        /* ... */
        todo!()
    }
}

impl LampControl for Lamp {
    fn set_intensity(&mut self, intensity: u8) {
        /* ... */
        todo!()
    }
    fn set_color(&mut self, color: Color) {
        /* ... */
        todo!()
    }
    fn get_state(&mut self) {
        todo!()
    }
    fn get_intensity(&mut self) {
        todo!()
    }
    fn get_color(&mut self) {
        todo!()
    }
}
