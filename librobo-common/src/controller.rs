use crate::{
    sbdbt::{Buttons, Sbdbt, Sticks},
    steering::Steering
};
use alloc::boxed::Box;
use core::error::Error;
use num_traits::real::Real;

const STICK_MAX: i8 = 63;

pub struct Controller {
    sbdbt: Sbdbt,
    st: Box<dyn Steering>,
    deadzones: ((i8, i8), (i8, i8))
}

impl Controller {
    pub fn new(sbdbt: Sbdbt, st: Box<dyn Steering>, deadzones: ((i8, i8), (i8, i8))) -> Self {
        Self {
            sbdbt,
            st,
            deadzones
        }
    }

    pub fn get_sticks(&self) -> Sticks {
        let sticks = self.sbdbt.get_sticks();
        Sticks(
            (
                if self.deadzones.0.0 < sticks.0.0.abs() {
                    sticks.0.0
                } else {
                    0i8
                },
                if self.deadzones.0.1 < sticks.0.1.abs() {
                    sticks.0.1
                } else {
                    0i8
                }
            ),
            (
                if self.deadzones.1.0 < sticks.1.0.abs() {
                    sticks.1.0
                } else {
                    0i8
                },
                if self.deadzones.1.1 < sticks.1.1.abs() {
                    sticks.1.1
                } else {
                    0i8
                }
            )
        )
    }

    pub fn get_buttons(&self) -> &Buttons {
        self.sbdbt.get_buttons()
    }

    pub fn update_buttons(&mut self, data: [u8; 8]) -> Result<(), Box<dyn Error + 'static>> {
        self.sbdbt.update_buttons(data)?;
        Ok(())
    }

    pub fn shaft(&mut self) -> Result<(), Box<dyn Error + 'static>> {
        let sticks = self.get_sticks();
        let r = (
            (sticks.0.0 as f32).hypot(sticks.0.1 as f32) / STICK_MAX as f32,
            (sticks.1.0 as f32).hypot(sticks.1.1 as f32) / STICK_MAX as f32
        );
        let theta = self.stick_to_polar(&sticks);
        self.st.polar(r, theta)?;
        Ok(())
    }

    fn stick_to_polar(&self, sticks: &Sticks) -> (f32, f32) {
        (
            if self.deadzones.0.1 < sticks.0.1.abs() {
                (sticks.0.1 as f32).atan2(if self.deadzones.0.0 < sticks.0.0.abs() {
                    sticks.0.0 as f32
                } else {
                    0f32
                })
            } else {
                0f32
            },
            if self.deadzones.1.1 < sticks.1.1.abs() {
                (sticks.1.1 as f32).atan2(if self.deadzones.1.0 < sticks.1.0.abs() {
                    sticks.1.0 as f32
                } else {
                    0f32
                })
            } else {
                0f32
            }
        )
    }
}
