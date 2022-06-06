use super::Model;
use crate::prelude::*;

pub const eps: f32 = 0.01; // Tolerance for computing matrix pseudo-inverse
pub const max_iter: u32 = 150; // Max iteration for DARE

pub fn lqr_control(x: Vector4, model: &Model, dt: f32) -> f32 {
    let (Ad, Bd) = model.get_model_matrix(dt);
    let Q = diag![0., 1., 1., 0.];
    let R = diag![0.01];
    let K = dlqr(Ad, Bd, Q, R);
    let u = -K * x;
    *u.index(0)
}

fn dlqr(A: Matrix4, B: Vector4, Q: Matrix4, R: Matrix1) -> RowVector4 {
    let P = solve_DARE(A, B, Q, R);

    // compute the LQR gain
    let BT = B.transpose();
    let inv = (BT * P * B + R)
        .pseudo_inverse(eps)
        .expect("Matrix inverse failed for DARE");
    let K = inv * (BT * P * A);

    let _eigen_vals = (A - B * K).eigenvalues();

    K
}

/// Solve the discrete time LQR controller.
///
/// x[k+1] = A x[k] + B u[k]
/// cost = sum x[k].T*Q*x[k] + u[k].T*R*u[k]
///
/// # ref Bertsekas, p.151
fn solve_DARE(A: Matrix4, B: Vector4, Q: Matrix4, R: Matrix1) -> Matrix4 {
    let mut P = Q;
    let AT = A.transpose();
    let BT = B.transpose();

    for _ in 0..max_iter {
        let inv = (R + BT * P * B)
            .pseudo_inverse(eps)
            .expect("Matrix inverse failed for DARE");

        let Pn = (AT * P * A) - (AT * P * B) * inv * (BT * P * A) + Q;
        if (Pn - P).abs().amax() < eps {
            return Pn;
        }

        P = Pn;
    }
    return P;
}
