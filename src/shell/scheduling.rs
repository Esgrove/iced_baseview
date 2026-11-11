//! Coalescing native frame requests without presenting from timer or action callbacks.

use iced_core::window::RedrawRequest;
use std::time::Instant;

#[derive(Default)]
pub(super) struct NativeFrameScheduler {
    immediate: bool,
    deadline: Option<Instant>,
}

impl NativeFrameScheduler {
    /// A native draw consumes an immediate request, including externally damaged frames.
    pub fn begin_draw(&mut self, now: Instant) {
        self.immediate = false;
        if self.deadline.is_some_and(|deadline| deadline <= now) {
            self.deadline = None;
        }
    }

    /// Returns only new or earlier requests. Existing delayed wakes cannot be cancelled,
    /// but an obsolete wake is harmless because all native draws repaint current state.
    pub fn request(
        &mut self,
        dirty: bool,
        deadline: Option<Instant>,
        now: Instant,
    ) -> RedrawRequest {
        if dirty || deadline.is_some_and(|deadline| deadline <= now) {
            if self.immediate {
                return RedrawRequest::Wait;
            }
            self.immediate = true;
            return RedrawRequest::NextFrame;
        }

        if let Some(deadline) = deadline
            && self.deadline.is_none_or(|scheduled| deadline < scheduled)
        {
            self.deadline = Some(deadline);
            return RedrawRequest::At(deadline);
        }

        RedrawRequest::Wait
    }
}

/// Timer-driven compatibility mode may render outside draw; native mode must not.
pub(super) fn draw_on_timer(native_frame_scheduling: bool) -> bool {
    !native_frame_scheduling
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn native_timer_and_resize_do_not_present() {
        assert!(!draw_on_timer(true));
        assert!(draw_on_timer(false));
    }

    #[test]
    fn dirty_actions_and_input_coalesce_until_native_draw() {
        let now = Instant::now();
        let mut scheduler = NativeFrameScheduler::default();
        assert_eq!(scheduler.request(true, None, now), RedrawRequest::NextFrame);
        assert_eq!(scheduler.request(true, None, now), RedrawRequest::Wait);
        scheduler.begin_draw(now);
        assert_eq!(scheduler.request(false, None, now), RedrawRequest::Wait);
        assert_eq!(scheduler.request(true, None, now), RedrawRequest::NextFrame);
    }

    #[test]
    fn always_redraw_and_next_frame_rearm_after_each_draw() {
        let now = Instant::now();
        let mut scheduler = NativeFrameScheduler::default();
        for _ in 0..4 {
            scheduler.begin_draw(now);
            assert_eq!(scheduler.request(true, None, now), RedrawRequest::NextFrame);
        }
    }

    #[test]
    fn deadlines_schedule_earliest_once_and_become_due_without_other_work() {
        let now = Instant::now();
        let early = now + Duration::from_millis(20);
        let late = now + Duration::from_millis(40);
        let mut scheduler = NativeFrameScheduler::default();
        assert_eq!(
            scheduler.request(false, Some(late), now),
            RedrawRequest::At(late)
        );
        assert_eq!(
            scheduler.request(false, Some(late), now),
            RedrawRequest::Wait
        );
        assert_eq!(
            scheduler.request(false, Some(early), now),
            RedrawRequest::At(early)
        );
        assert_eq!(
            scheduler.request(false, Some(late), now),
            RedrawRequest::Wait
        );
        scheduler.begin_draw(early);
        assert_eq!(
            scheduler.request(false, Some(early), early),
            RedrawRequest::NextFrame
        );
    }

    #[test]
    fn external_damage_can_draw_and_recovery_can_retry() {
        let now = Instant::now();
        let mut scheduler = NativeFrameScheduler::default();
        scheduler.begin_draw(now);
        assert_eq!(scheduler.request(false, None, now), RedrawRequest::Wait);
        assert_eq!(scheduler.request(true, None, now), RedrawRequest::NextFrame);
    }

    #[test]
    fn native_scheduling_is_opt_in() {
        let settings = crate::IcedWindowSettings::default();
        assert!(!settings.native_frame_scheduling);
        assert!(
            settings
                .with_native_frame_scheduling(true)
                .native_frame_scheduling
        );
    }
}
