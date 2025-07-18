fn main() {
    fn abc() {
        fn def() {
            fn ghi() {
                fn abc() {
                    fn improving_move(
                        input: &DurationMatrix,
                        tour: &Tour,
                        tour_cost: u64,
                    ) -> Option<Move> {
                        let (mut best_cost, mut best_move) = (tour_cost, None);

                        for mv in feasible_moves(input, tour) {
                            if mv.cost() < best_cost {
                                (best_cost, best_move) = (mv.cost(), Some(mv));
                            }
                        }

                        best_move
                    }
                }
            }
        }
    }
}
