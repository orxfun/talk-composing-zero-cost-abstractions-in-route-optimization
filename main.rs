fn main() {
    fn abc() {
        fn def() {
            fn ghi() {
                fn abc() {
                    fn local_search_tw(
                        input_duration: &DurationMatrix,
                        input_tw: &AvailableTimeSlots,
                        mut tour: Tour,
                    ) -> Tour {
                        let mut cost = tour_cost_tw(input, &tour);

                        while let Some((mv, neighbor_cost)) =
                            best_move_duration_tw(input_duration, input_tw, &tour, cost)
                        {
                            mv.apply(&mut tour);
                            cost = neighbor_cost;
                        }

                        tour
                    }
                }
            }
        }
    }
}
