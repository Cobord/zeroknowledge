use std::marker::PhantomData;

use crate::victor::{PeggyProbe, PeggyResponsable};

pub struct Peggy<AuxData, PeggyGaveThis1, PeggyGaveThis2, MyInterface>
where
    PeggyGaveThis1: PeggyResponsable<AuxData = AuxData>,
    PeggyGaveThis2: PeggyResponsable<AuxData = AuxData>,
    MyInterface: PeggyProbe<
        AuxData = AuxData,
        PeggyGaveThis1 = PeggyGaveThis1,
        PeggyGaveThis2 = PeggyGaveThis2,
    >,
{
    z0: PhantomData<MyInterface>,
}

impl<AuxData, PeggyGaveThis1, PeggyGaveThis2, MyInterface>
    Peggy<AuxData, PeggyGaveThis1, PeggyGaveThis2, MyInterface>
where
    PeggyGaveThis1: PeggyResponsable<AuxData = AuxData>,
    PeggyGaveThis2: PeggyResponsable<AuxData = AuxData>,
    MyInterface: PeggyProbe<
        AuxData = AuxData,
        PeggyGaveThis1 = PeggyGaveThis1,
        PeggyGaveThis2 = PeggyGaveThis2,
    >,
{
    #[allow(clippy::unused_async)]
    pub async fn give_victor_interface(&self) -> MyInterface {
        todo!()
    }
}
