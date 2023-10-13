/// Swiss Oblique Mercator Projection
/// https://proj.org/operations/projections/somerc.html
/// Implementation copied from [Proj4rs](https://github.com/3liz/proj4rs/blob/main/src/tests.rs#L40)
///
/// Resources:
/// - https://download.osgeo.org/proj/swiss.pdf
use geodesy_rs::authoring::*;
use geodesy_rs::Error as RGError;
use std::f64::consts::{FRAC_PI_2, FRAC_PI_4};

// ----- C O M M O N -------------------------------------------------------------------

const EPS_10: f64 = 1.0e-10;

// ----- F O R W A R D -----------------------------------------------------------------

fn fwd(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let mut successes = 0_usize;
    let n = operands.len();

    let el = op.params.ellps(0);
    let e = el.eccentricity();

    // Grab precomputed values
    let hlf_e = op.params.real["hlf_e"];
    let c = op.params.real["c"];
    let k = op.params.real["k"];
    let k_r = op.params.real["k_r"];
    let sin_p_0 = op.params.real["sin_p_0"];
    let cos_p_0 = op.params.real["cos_p_0"];

    for i in 0..n {
        let mut coord = operands.get_coord(i);
        // might need to swap these
        let lon = coord[0];
        let lat = coord[1];

        let sp = e * lat.sin();
        let lat_p = 2.
            * ((c * ((FRAC_PI_4 + 0.5 * lat).tan().ln() - hlf_e * ((1. + sp) / (1. - sp)).ln())
                + k)
                .exp())
            .atan()
            - FRAC_PI_2;

        let lon_p = c * lon;
        let cp = lat_p.cos();
        let lat_pp = aasin(cos_p_0 * lat_p.sin() - sin_p_0 * cp * lon_p.cos());
        let lon_pp = aasin(cp * lon_p.sin() / lat_pp.cos());

        coord[0] = k_r * lon_pp;
        coord[1] = k_r * (FRAC_PI_4 + 0.5 * lat_pp).tan().ln();

        operands.set_coord(i, &coord);
        successes += 1;
    }

    successes
}

// ----- I N V E R S E -----------------------------------------------------------------

fn inv(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let mut successes = 0_usize;
    let n = operands.len();
    const MAX_ITERATIONS: isize = 6;

    let el = op.params.ellps(0);
    let e = el.eccentricity();

    // Grab precomputed values
    let hlf_e = op.params.real["hlf_e"];
    let c = op.params.real["c"];
    let k = op.params.real["k"];
    let k_r = op.params.real["k_r"];
    let sin_p_0 = op.params.real["sin_p_0"];
    let cos_p_0 = op.params.real["cos_p_0"];
    let rone_es = op.params.real["rone_es"];

    for i in 0..n {
        let mut coord = operands.get_coord(i);
        let x = coord[1];
        let y = coord[0];

        let lat_pp = 2. * (((y / k_r).exp()).atan() - FRAC_PI_4);
        let lon_pp = x / k_r;
        let cp = lat_pp.cos();
        let mut lat_p = aasin(cos_p_0 * lat_pp.sin() + sin_p_0 * cp * lon_pp.cos());
        let lon_p = aasin(cp * lon_pp.sin() / lat_p.cos());
        let con = (k - (FRAC_PI_4 + 0.5 * lat_p).tan().ln()) / c;

        let mut j = MAX_ITERATIONS;
        while j > 0 {
            let esp = e * lat_p.sin();
            let delta_p = (con + (FRAC_PI_4 + 0.5 * lat_p).tan().ln()
                - hlf_e * ((1. + esp) / (1. - esp)).ln())
                * (1. - esp * esp)
                * lat_p.cos()
                * rone_es;
            lat_p -= delta_p;
            if delta_p.abs() < EPS_10 {
                break;
            }
            j -= 1;
        }
        if j <= 0 {
            panic!("somerc: Too many iterations in inverse")
        } else {
            coord[0] = lon_p / c;
            coord[1] = lat_p;
            operands.set_coord(i, &coord);
            successes += 1;
        }
    }

    successes
}

// ----- C O N S T R U C T O R ---------------------------------------------------------

#[rustfmt::skip]
pub const GAMUT: [OpParameter; 7] = [
    OpParameter::Flag { key: "inv" },
    OpParameter::Text { key: "ellps",  default: Some("GRS80") },
    // TODO: Handle case when R is used.
    // If R is present it take precedence over ellps
    // OpParameter::Real{key: "R", default: None},

    OpParameter::Real { key: "lon_0",  default: Some(0_f64) },
    OpParameter::Real { key: "lat_0",  default: Some(0_f64) },
    OpParameter::Real { key: "x_0",    default: Some(0_f64) },
    OpParameter::Real { key: "y_0",    default: Some(0_f64) },

    OpParameter::Real { key: "k_0",    default: Some(1_f64) },
];

pub fn new(parameters: &RawParameters, _ctx: &dyn Context) -> Result<Op, RGError> {
    let def = &parameters.definition;
    let mut params = ParsedParameters::new(parameters, &GAMUT)?;

    let el = params.ellps(0);
    let hlf_e = 0.5 * el.eccentricity();
    let es = el.eccentricity_squared();
    let e = el.eccentricity();

    // As per https://github.com/3liz/proj4rs/blob/a06fb2082fb1b7d7fca609a9f6a1259c993781d6/src/ellps.rs#L89C1-L90C34
    let one_es = 1. - el.eccentricity_squared();
    let rone_es = 1. / one_es;

    let lat_0 = params.real["lat_0"].to_radians();
    // https://github.com/3liz/proj4rs/blob/a06fb2082fb1b7d7fca609a9f6a1259c993781d6/src/proj.rs#L350C4-L350C68
    let (sin_lat, cos_lat) = lat_0.sin_cos();

    let cp = cos_lat * cos_lat;
    let c = (1. + es * cp * cp * rone_es).sqrt();
    let sin_p_0 = sin_lat / c;
    let lat_p0 = aasin(sin_p_0);
    let cos_p_0 = lat_p0.cos();
    let sp = sin_lat * e;
    let k = (FRAC_PI_4 + 0.5 * lat_p0).tan().ln()
        - c * ((FRAC_PI_4 + 0.5 * lat_0).tan().ln() - hlf_e * ((1. + sp) / (1. - sp)).ln());

    // let alpha = (1.0 + es / (1.0 - es) * (cos_lat.powi(4))).sqrt();

    // let k1 = (FRAC_PI_4 + 0.5 * sin_p_0).tan().ln();
    // let k2 = (FRAC_PI_4 + 0.5 * lat_0).tan().ln();
    // let k3 = ((1.0 + e * sin_lat) / (1.0 - e * sin_lat)).ln();
    // let K = k1 - alpha * k2 + alpha * e / 2.0 * k3;

    let k_r = params.real["k_0"] * one_es.sqrt() / (1. - sp * sp);

    params.real.insert("c", c);
    params.real.insert("hlf_e", hlf_e);
    params.real.insert("rone_es", rone_es);
    params.real.insert("k", k);
    params.real.insert("k_r", k_r);
    params.real.insert("cos_p_0", cos_p_0);
    params.real.insert("sin_p_0", sin_p_0);

    let descriptor = OpDescriptor::new(def, InnerOp(fwd), Some(InnerOp(inv)));
    let steps = Vec::<Op>::new();
    let id = OpHandle::new();

    Ok(Op {
        descriptor,
        params,
        steps,
        id,
    })
}

// ----- Ancillary functions -----------------------------------------------------------
// https://github.com/3liz/proj4rs/blob/a06fb2082fb1b7d7fca609a9f6a1259c993781d6/src/math/aasincos.rs#L7-L21
const ONE_TOL: f64 = 1.000_000_000_000_01;
fn aasin(v: f64) -> f64 {
    let av = v.abs();
    if av >= 1. {
        if av > ONE_TOL {
            f64::NAN
        } else {
            FRAC_PI_2 * v.signum()
        }
    } else {
        v.asin()
    }
}

// ----- T E S T S ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    // NOTE: Tests are failing. Probs need better ones.

    #[test]
    fn proj_somerc_el() -> Result<(), RGError> {
        let mut ctx = Minimal::default();
        ctx.register_op("somerc", OpConstructor(new));
        let op = ctx.op("somerc ellps=GRS80")?;

        let input = [
            Coor4D::gis(2., 1., 0., 0.0),
            Coor4D::gis(2., -1., 0., 0.0),
            Coor4D::gis(-2., 1., 0., 0.0),
            Coor4D::gis(-2., -1., 0., 0.0),
        ];

        let mut operands = input.clone();

        let expected = [
            Coor4D::raw(222638.98158654713, 110579.96521824898, 0., 0.0),
            Coor4D::raw(222638.98158654713, -110579.96521825089, 0., 0.0),
            Coor4D::raw(-222638.98158654713, 110579.96521824898, 0., 0.0),
            Coor4D::raw(-222638.98158654713, -110579.96521825089, 0., 0.0),
        ];

        // Forward
        let successes = ctx.apply(op, Fwd, &mut operands)?;

        for i in 0..successes {
            assert_float_eq!(operands[i][0], expected[i][0], abs_all <= 1e-9);
            assert_float_eq!(operands[i][1], expected[i][1], abs_all <= 1e-9);
            assert_float_eq!(operands[i][2], expected[i][2], abs_all <= 1e-9);
            assert_float_eq!(operands[i][3], expected[i][3], abs_all <= 1e-9);
        }

        // Inverse + roundtrip
        let inverse_successes = ctx.apply(op, Inv, &mut operands)?;
        for i in 0..inverse_successes {
            assert_float_eq!(operands[i][0], input[i][0], abs_all <= 1e-9);
            assert_float_eq!(operands[i][1], input[i][1], abs_all <= 1e-9);
            assert_float_eq!(operands[i][2], input[i][2], abs_all <= 1e-9);
            assert_float_eq!(operands[i][3], input[i][3], abs_all <= 1e-9);
        }

        Ok(())
    }

    #[test]
    fn proj_somerc_sp() -> Result<(), RGError> {
        let mut ctx = Minimal::default();
        ctx.register_op("somerc", OpConstructor(new));
        let op = ctx.op("somerc a=6400000")?;

        let input = [
            Coor4D::gis(2., 1., 0., 0.0),
            Coor4D::gis(2., -1., 0., 0.0),
            Coor4D::gis(-2., 1., 0., 0.0),
            Coor4D::gis(-2., -1., 0., 0.0),
        ];

        let mut operands = input.clone();

        let expected = [
            Coor4D::raw(223402.14425527418, 111706.74357494408, 0., 0.),
            Coor4D::raw(223402.14425527418, -111706.74357494518, 0., 0.),
            Coor4D::raw(-223402.14425527418, 111706.74357494408, 0., 0.),
            Coor4D::raw(-223402.14425527418, -111706.74357494518, 0., 0.),
        ];

        let successes = ctx.apply(op, Fwd, &mut operands)?;

        for i in 0..successes {
            assert_float_eq!(operands[i][0], expected[i][0], abs_all <= 1e-9);
            assert_float_eq!(operands[i][1], expected[i][1], abs_all <= 1e-9);
            assert_float_eq!(operands[i][2], expected[i][2], abs_all <= 1e-9);
            assert_float_eq!(operands[i][3], expected[i][3], abs_all <= 1e-9);
        }

        // Inverse + roundtrip
        let inverse_successes = ctx.apply(op, Inv, &mut operands)?;
        for i in 0..inverse_successes {
            assert_float_eq!(operands[i][0], input[i][0], abs_all <= 1e-9);
            assert_float_eq!(operands[i][1], input[i][1], abs_all <= 1e-9);
            assert_float_eq!(operands[i][2], input[i][2], abs_all <= 1e-9);
            assert_float_eq!(operands[i][3], input[i][3], abs_all <= 1e-9);
        }

        Ok(())
    }
}
