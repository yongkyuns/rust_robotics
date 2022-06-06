use super::*;
use crate::prelude::*;

// pub const l_bar: f32 = 2.0; // [m] Length of bar
// pub const M: f32 = 1.0; // [kg] Mass of cart
// pub const m: f32 = 1.0; // [kg] Mass of ball
// pub const g: f32 = 9.8; // [m/s^2] Gravity

const N: usize = 12; // Prediction horizon

pub fn mpc_control(_x: Vector4, model: Model, dt: f32) -> f32 {
    let (Ad, Bd) = model.get_model_matrix(dt);

    let u0 = 0.0;
    let umin = vector![(-30_f32 - u0).to_radians()];
    let umax = vector![(30_f32 - u0).to_radians()];
    let xmin = vector![-10_f32, -10., -1., -1.]; // lateral position, lateral velocity, yaw, yaw-rate
    let xmax = vector![10_f32, 10., 1., 1.];

    // Objective function
    let Q = diag![1_f32, 1., 1., 1.];
    let QN = Q;
    let R = eye![1];

    // Initial and reference states
    let x0 = zeros![NX];
    let xr = vector!(1_f32, 0., 0., 0.);

    // Cast MPC problem to a QP: x = (x(0),x(1),...,x(N),u(0),...,u(N-1))
    // - quadratic objective
    let P = block_diag!(kron!(eye!(N), Q), QN, kron!(eye!(N), R));
    let q = vstack!(
        kron!(ones!(N, 1), -dot!(Q, xr)),
        -dot!(QN, xr),
        zeros!({ N * NU }, 1)
    )
    .transpose();

    // - linear dynamics
    let Ax = kron!(eye!(N + 1), -eye!(NX)) + kron!(eye!({ N + 1 }, -1), Ad);
    let Bu = kron!(vstack!(zeros!(1, N), eye!(N)), Bd);

    let Aeq = hstack!(Ax, Bu);
    let leq = hstack!(-x0, zeros!(1, { N * NX }));
    let ueq = leq;

    // - input and state constraints
    let Aineq = eye!((N + 1) * NX + N * NU);
    let lineq = vstack!(kron!(ones!({ N + 1 }, 1), xmin), kron!(ones!(N, 1), umin)).transpose();
    let uineq = vstack!(kron!(ones!({ N + 1 }, 1), xmax), kron!(ones!(N, 1), umax)).transpose();

    // - OSQP constraints
    let A = vstack!(Aeq, Aineq);
    let l = hstack!(leq, lineq);
    let u = hstack!(ueq, uineq);

    let _P = &P.transpose().data.0;
    let _A = &A.transpose().data.0;
    let _q = &q.transpose().data.0[0];
    let _l = &l.transpose().data.0[0];
    let _u = &u.transpose().data.0[0];

    // TODO: OSQP expects f64 slices
    //
    // TODO:
    // Setting up the problem in this way is quite slow (~800 us)
    // Solving it takes ~120 us, so we may want to explor more efficient
    // way of setting up OSQP

    // println!(
    //     "Finished computing matrices. Elapsed = {} micros",
    //     now.elapsed().as_micros()
    // );
    // let now = Instant::now();

    // Extract the upper triangular elements of `P`
    // let P = CscMatrix::from(&P).into_upper_tri();

    // // Disable verbose output
    // let settings = Settings::default();

    // // Create an OSQP problem
    // let mut prob = Problem::new(P, q, A, l, u, &settings).expect("failed to setup problem");
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mpc_control() {
        let x = vector![0., 1., 1., 0.];
        let dt = 0.1;
        let model = Model::default();
        let _u = mpc_control(x, model, dt);
        // assert_eq!(1, 1);
    }
}
