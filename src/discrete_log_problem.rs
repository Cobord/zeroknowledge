use core::ops::{Div, Mul};
use num_traits::identities::One;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

use crate::victor::{PeggyProbe, PeggyResponsable};

pub trait DiscreteLogProblem:
    Clone + Eq + Sized + Mul<Self, Output = Self> + Div<Self, Output = Self> + One
{
    fn generator() -> Self;
    fn desired_element() -> Self;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupPower<T>
where
    T: DiscreteLogProblem,
{
    power: usize,
    ambient_group: PhantomData<T>,
}

impl<T> GroupPower<T>
where
    T: DiscreteLogProblem,
{
    fn gen_to_this(&self) -> T {
        if self.power == 0 {
            return T::one();
        }
        if self.power == 1 {
            return T::generator();
        }
        todo!()
    }

    fn random() -> Self {
        todo!();
    }
}

impl<T> PeggyResponsable for GroupPower<T>
where
    T: DiscreteLogProblem + Serialize,
{
    type AuxData = T;

    fn is_correct(&self, aux: &Self::AuxData) -> bool {
        self.gen_to_this().eq(aux)
    }
}

impl<T> PeggyResponsable for T
where
    T: DiscreteLogProblem + Serialize,
{
    type AuxData = T;

    fn is_correct(&self, aux: &Self::AuxData) -> bool {
        // this is g^{x+r}
        let mut self_clone = self.clone();
        self_clone = self_clone / aux.clone();
        self_clone.eq(&T::desired_element())
    }
}

struct HonestPeggyInterface<T>
where
    T: DiscreteLogProblem,
{
    x: GroupPower<T>,
    r: GroupPower<T>,
}

impl<T> PeggyProbe for HonestPeggyInterface<T>
where
    T: DiscreteLogProblem + Serialize,
{
    type AuxData = T;

    type PeggyGaveThis1 = GroupPower<Self::AuxData>;

    type PeggyGaveThis2 = Self::AuxData;

    async fn ask_peggy_aux(&self) -> Self::AuxData {
        self.r.gen_to_this()
    }

    async fn ask_peggy_1(&self) -> Self::PeggyGaveThis1 {
        self.r.clone()
    }

    async fn ask_peggy_2(&self) -> Self::PeggyGaveThis2 {
        let new_power = GroupPower::<T> {
            power: self.r.power + self.x.power,
            ambient_group: PhantomData,
        };
        new_power.gen_to_this()
    }

    async fn peggy_prepare(&mut self) {
        self.r = GroupPower::<T>::random();
    }
}
