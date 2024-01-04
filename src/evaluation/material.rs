use crate::prelude::*;

#[derive(Clone, Copy)]
pub struct MaterialEvaluator;

impl StaticEvaluator for MaterialEvaluator {
    fn eval(&self, position: &Position) -> f64 {
        let wpawns = (position.pawns & position.n_white).count_ones() as f64;
        let wknights = (position.knights & position.n_white).count_ones() as f64;
        let wbishops = (position.bishops & position.n_white).count_ones() as f64;
        let wrooks = (position.rooks & position.n_white).count_ones() as f64;
        let wqueens = (position.queens & position.n_white).count_ones() as f64;

        let bpawns = (position.pawns & position.n_black).count_ones() as f64;
        let bknights = (position.knights & position.n_black).count_ones() as f64;
        let bbishops = (position.bishops & position.n_black).count_ones() as f64;
        let brooks = (position.rooks & position.n_black).count_ones() as f64;
        let bqueens = (position.queens & position.n_black).count_ones() as f64;

        let wscore = wpawns + (3. * wknights) + (3. * wbishops) + (5. * wrooks) + (9. * wqueens);
        let bscore = bpawns + (3. * bknights) + (3. * bbishops) + (5. * brooks) + (9. * bqueens);

        wscore - bscore
    }
}
