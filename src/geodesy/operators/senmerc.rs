//! Sensat Mercator
//! A bastardisation for 3D where the Z value is scaled by 1 / cos(lat) in the forward direction
use geodesy_rs::authoring::*;
use std::f64::consts::FRAC_PI_2;
use std::f64::consts::FRAC_PI_4;

// ----- F O R W A R D -----------------------------------------------------------------

fn fwd(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let ellps = op.params.ellps(0);
    let a = ellps.semimajor_axis();

    let mut successes = 0_usize;
    let length = operands.len();
    for i in 0..length {
        let mut coord = operands.get_coord(i);
        // Easting
        coord[0] *= a;

        // Northing
        let lat = coord[1];
        coord[1] = a * (FRAC_PI_4 + lat / 2.0).tan().ln();

        // Altitude
        coord[2] *= 1.0 / lat.cos();

        operands.set_coord(i, &coord);
        successes += 1;
    }

    successes
}

// ----- I N V E R S E -----------------------------------------------------------------

fn inv(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let ellps = op.params.ellps(0);
    let a = ellps.semimajor_axis();

    let mut successes = 0_usize;
    let length = operands.len();
    for i in 0..length {
        let mut coord = operands.get_coord(i);

        // Easting -> Longitude
        coord[0] /= a;

        // Northing -> Latitude
        coord[1] = FRAC_PI_2 - 2.0 * (-coord[1] / a).exp().atan();

        // Altitude
        coord[2] *= coord[1].cos();

        operands.set_coord(i, &coord);
        successes += 1;
    }

    successes
}

// ----- C O N S T R U C T O R ---------------------------------------------------------

#[rustfmt::skip]
pub const GAMUT: [OpParameter; 2] = [
    OpParameter::Flag { key: "inv" },
    OpParameter::Text { key: "ellps",  default: Some("WGS84") },
];

pub fn new(parameters: &RawParameters, _ctx: &dyn Context) -> Result<Op, Error> {
    let def = &parameters.definition;
    let params = ParsedParameters::new(parameters, &GAMUT)?;

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

// ----- T E S T S ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use float_eq::assert_float_eq;

    #[test]
    fn senmerc() -> Result<(), Error> {
        let mut ctx = Minimal::default();
        ctx.register_op("senmerc", OpConstructor(new));
        let definition = "senmerc";
        let op = ctx.op(definition)?;

        let ldn = [Coor4D::geo(51.505, -0.09, 30., 0.)];

        let projected = [Coor4D::raw(
            -10_018.754_171_394_621,
            6_711_113.243_704_713,
            48.196_925_789_267_21,
            0.,
        )];

        // Forward
        let mut operands = ldn;
        ctx.apply(op, Fwd, &mut operands)?;
        for i in 0..operands.len() {
            assert_float_eq!(operands[i].0, projected[i].0, abs_all <= 1e-8);
        }

        // Roundtrip
        ctx.apply(op, Inv, &mut operands)?;
        for i in 0..operands.len() {
            assert_float_eq!(operands[i].0, ldn[i].0, abs_all <= 2e-9);
        }

        Ok(())
    }
}
