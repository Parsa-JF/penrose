//! Manage hooks for common manage actions
//!
//! Manage hooks should _not_ trigger a refresh directly: that is handled by penrose
//! itself when the manage hook is called.
use crate::{
    core::{hooks::ManageHook, State},
    pure::geometry::Rect,
    x::{Query, XConn},
    Result, Xid,
};

// A tuple of (query, manage hook) runs conditionally if the query holds
// for the window being managed.
impl<X, Q, H> ManageHook<X> for (Q, H)
where
    X: XConn,
    Q: Query<X>,
    H: ManageHook<X>,
{
    fn call(&mut self, id: Xid, state: &mut State<X>, x: &X) -> Result<()> {
        if self.0.run(id, x)? {
            self.1.call(id, state, x)?;
        }

        Ok(())
    }
}

fn float<X: XConn>(client: Xid, r: Rect, state: &mut State<X>, _: &X) -> Result<()> {
    state.client_set.float_unchecked(client, r);

    Ok(())
}

/// Perform no additional actions when managing a new client.
#[derive(Debug)]
pub struct DefaultTiled;
impl<X: XConn> ManageHook<X> for DefaultTiled {
    fn call(&mut self, _client: Xid, _state: &mut State<X>, _x: &X) -> Result<()> {
        Ok(())
    }
}

/// Float clients at a fixed position on the screen.
#[derive(Debug)]
pub struct FloatingFixed(pub Rect);
impl<X: XConn> ManageHook<X> for FloatingFixed {
    fn call(&mut self, client: Xid, state: &mut State<X>, x: &X) -> Result<()> {
        float(client, self.0, state, x)
    }
}

/// Float clients in the center of the screen.
#[derive(Debug)]
pub struct FloatingCentered {
    w: f64,
    h: f64,
}

impl FloatingCentered {
    /// Create a new [FloatingCentered] with the given width and height ratios.
    ///
    /// # Panics
    /// Panics if `w` or `h` are not in the range `0.0..=1.0`.
    pub fn new(w: f64, h: f64) -> Self {
        if !((0.0..=1.0).contains(&w) && (0.0..=1.0).contains(&h)) {
            panic!("w and h must be between 0.0 and 1.0: got w={w}, h={h}")
        }

        Self { w, h }
    }
}

impl<X: XConn> ManageHook<X> for FloatingCentered {
    fn call(&mut self, client: Xid, state: &mut State<X>, x: &X) -> Result<()> {
        let r_screen = &state.client_set.screens.focus.r;
        let r = r_screen
            .scale_h(self.h)
            .scale_w(self.w)
            .centered_in(r_screen)
            .expect("bounds checks in FloatingCentered::new to be upheld");

        float(client, r, state, x)
    }
}
