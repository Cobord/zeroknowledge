use std::num::NonZeroU8;

use serde::Serialize;

use crate::peggy::Peggy;

pub struct PeggyDoesntKnowSoDontTellPeggy;

pub trait PeggyResponsable: Serialize {
    type AuxData: Serialize;

    fn is_correct(&self, aux: &Self::AuxData) -> bool;
}

pub trait PeggyProbe {
    type AuxData;
    type PeggyGaveThis1: PeggyResponsable<AuxData = Self::AuxData>;
    type PeggyGaveThis2: PeggyResponsable<AuxData = Self::AuxData>;

    async fn peggy_prepare(&mut self);

    async fn ask_peggy_aux(&self) -> Self::AuxData;

    async fn ask_peggy_1(&self) -> Self::PeggyGaveThis1;

    async fn ask_peggy_2(&self) -> Self::PeggyGaveThis2;
}

#[allow(dead_code)]
pub struct Victor<PeggyResponse1, PeggyResponse2, PeggyProbing, AuxData>
where
    PeggyResponse1: PeggyResponsable<AuxData = AuxData>,
    PeggyResponse2: PeggyResponsable<AuxData = AuxData>,
    PeggyProbing: PeggyProbe<
        PeggyGaveThis1 = PeggyResponse1,
        PeggyGaveThis2 = PeggyResponse2,
        AuxData = AuxData,
    >,
{
    asker: PeggyProbing,
}

impl<PeggyResponse1, PeggyResponse2, PeggyProbing, AuxData>
    Victor<PeggyResponse1, PeggyResponse2, PeggyProbing, AuxData>
where
    PeggyResponse1: PeggyResponsable<AuxData = AuxData>,
    PeggyResponse2: PeggyResponsable<AuxData = AuxData>,
    PeggyProbing: PeggyProbe<
        PeggyGaveThis1 = PeggyResponse1,
        PeggyGaveThis2 = PeggyResponse2,
        AuxData = AuxData,
    >,
{
    #[allow(dead_code)]
    pub fn new(asker: PeggyProbing) -> Self {
        Self { asker }
    }

    #[allow(dead_code)]
    pub async fn new_from_peggy(
        peggy: &Peggy<AuxData, PeggyResponse1, PeggyResponse2, PeggyProbing>,
    ) -> Self {
        Self::new(peggy.give_victor_interface().await)
    }

    async fn do_one_round(&mut self) -> bool {
        let () = self.asker.peggy_prepare().await;
        let my_aux_data = self.asker.ask_peggy_aux().await;
        let do_1: bool = true;
        if do_1 {
            let r1 = self.asker.ask_peggy_1().await;
            r1.is_correct(&my_aux_data)
        } else {
            let r2 = self.asker.ask_peggy_2().await;
            r2.is_correct(&my_aux_data)
        }
    }

    #[allow(dead_code)]
    /// `Ok(n)` if peggy probably knows
    /// probability of a false positive <= 1/2^n
    /// `Err(_)` if peggy responded with something wrong
    /// that she would have surely gotten right if she knew
    /// we have that extra `current_confidence` if we want to call this multiple times
    /// like we first get a 7/8 probability that Peggy knows
    /// then later decide that isn't good enough and want to do a few more rounds
    pub async fn peggy_does_know(
        &mut self,
        rounds_to_do: NonZeroU8,
        current_confidence: u8,
    ) -> Result<NonZeroU8, PeggyDoesntKnowSoDontTellPeggy> {
        for _ in 0..Into::<u8>::into(rounds_to_do) {
            let cur_round = self.do_one_round().await;
            if !cur_round {
                return Err(PeggyDoesntKnowSoDontTellPeggy);
            }
        }
        Ok(rounds_to_do.saturating_add(current_confidence))
    }
}
