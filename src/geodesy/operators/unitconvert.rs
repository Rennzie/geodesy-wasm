/// Template for implementation of operators
use geodesy_rs::authoring::*;

use super::units;

// ----- C O M M O N -------------------------------------------------------------------

// ----- F O R W A R D -----------------------------------------------------------------

fn fwd(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let mut successes = 0_usize;
    let n = operands.len();
    let units = units::linear_units_map();
    let xy_in = op.params.text("xy_in").unwrap_or("m".to_string());
    let xy_out = op.params.text("xy_out").unwrap_or("m".to_string());
    let z_in = op.params.text("z_in").unwrap_or("m".to_string());
    let z_out = op.params.text("z_out").unwrap_or("m".to_string());

    // Unit A => meters => Unit B
    let xy_in_to_m = units[xy_in.as_str()].to_meters;
    let m_to_xy_out = 1. / units[xy_out.as_str()].to_meters;
    let z_in_to_m = units[z_in.as_str()].to_meters;
    let m_to_z_out = 1. / units[z_out.as_str()].to_meters;

    for i in 0..n {
        let mut coord = operands.get_coord(i);

        // Convert xy
        coord[0] *= xy_in_to_m;
        coord[0] *= m_to_xy_out;

        coord[1] *= xy_in_to_m;
        coord[1] *= m_to_xy_out;

        // Convert z
        coord[2] *= z_in_to_m;
        coord[2] *= m_to_z_out;

        operands.set_coord(i, &coord);

        successes += 1;
    }

    successes
}

// ----- I N V E R S E -----------------------------------------------------------------

fn inv(op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    let mut successes = 0_usize;
    let n = operands.len();
    let units = units::linear_units_map();
    let xy_in = op.params.text("xy_in").unwrap_or("m".to_string());
    let xy_out = op.params.text("xy_out").unwrap_or("m".to_string());
    let z_in = op.params.text("z_in").unwrap_or("m".to_string());
    let z_out = op.params.text("z_out").unwrap_or("m".to_string());

    // Unit A => meters => Unit B
    // Note: the in and out is reversed relative to fwd because we're going backwards
    let xy_in_to_m = units[xy_out.as_str()].to_meters;
    let m_to_xy_out = 1. / units[xy_in.as_str()].to_meters;
    let z_in_to_m = units[z_out.as_str()].to_meters;
    let m_to_z_out = 1. / units[z_in.as_str()].to_meters;

    for i in 0..n {
        let mut coord = operands.get_coord(i);

        // Convert xy
        coord[0] *= xy_in_to_m;
        coord[0] *= m_to_xy_out;

        coord[1] *= xy_in_to_m;
        coord[1] *= m_to_xy_out;

        // Convert z
        coord[2] *= z_in_to_m;
        coord[2] *= m_to_z_out;

        operands.set_coord(i, &coord);

        successes += 1;
    }

    successes
}

// ----- C O N S T R U C T O R ---------------------------------------------------------

// Example...
#[rustfmt::skip]
pub const GAMUT: [OpParameter; 5] = [
    OpParameter::Flag { key: "inv" },
    OpParameter::Text { key: "xy_in", default: Some("m") },
    OpParameter::Text { key: "xy_out", default: Some("m") },
    OpParameter::Text { key: "z_in", default: Some("m") },
    OpParameter::Text { key: "z_out", default: Some("m") },
];

pub fn new(parameters: &RawParameters, _ctx: &dyn Context) -> Result<Op, Error> {
    Op::plain(parameters, InnerOp(fwd), Some(InnerOp(inv)), &GAMUT, _ctx)
}

// ----- A N C I L L A R Y   F U N C T I O N S -----------------------------------------

// ----- T E S T S ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use float_eq::assert_float_eq;

    use super::*;

    #[test]
    fn xy_us_ft_round_trip() -> Result<(), Error> {
        let mut ctx = Minimal::default();
        ctx.register_op("unitconvert", OpConstructor(new));
        let op = ctx.op("unitconvert xy_in=us-ft xy_out=us-ft")?;

        let mut operands = [Coor4D::raw(5., 5., 1., 1.)];

        // Forward
        let successes = ctx.apply(op, Fwd, &mut operands)?;
        println!("{:?}", operands);
        assert_float_eq!(operands[0][0], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][1], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][2], 1., abs_all <= 1e-9);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-9);

        assert_eq!(successes, 1);

        // Inverse + roundtrip
        ctx.apply(op, Inv, &mut operands)?;
        assert_float_eq!(operands[0][0], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][1], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][2], 1., abs_all <= 1e-9);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-9);
        Ok(())
    }

    #[test]
    fn xyz_us_ft_to_m() -> Result<(), Error> {
        let mut ctx = Minimal::default();
        ctx.register_op("unitconvert", OpConstructor(new));
        let op = ctx.op("unitconvert xy_in=us-ft xy_out=m z_in=us-ft z_out=m")?;

        let mut operands = [Coor4D::raw(5., 5., 5., 1.)];

        // Forward
        let successes = ctx.apply(op, Fwd, &mut operands)?;
        println!("{:?}", operands);
        assert_float_eq!(operands[0][0], 1.524003048, abs_all <= 1e-9);
        assert_float_eq!(operands[0][1], 1.524003048, abs_all <= 1e-9);
        assert_float_eq!(operands[0][2], 1.524003048, abs_all <= 1e-9);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-9);

        assert_eq!(successes, 1);

        // Inverse + roundtrip
        ctx.apply(op, Inv, &mut operands)?;
        assert_float_eq!(operands[0][0], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][1], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][2], 5., abs_all <= 1e-9);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-9);
        Ok(())
    }

    #[test]
    fn xy_yd_to_m() -> Result<(), Error> {
        let mut ctx = Minimal::default();
        ctx.register_op("unitconvert", OpConstructor(new));
        let op = ctx.op("unitconvert xy_in=us-yd xy_out=m")?;

        let mut operands = [Coor4D::raw(1000., 1000., 500., 1.)];

        // Forward
        let successes = ctx.apply(op, Fwd, &mut operands)?;
        println!("{:?}", operands);
        assert_float_eq!(operands[0][0], 914.40182880, abs_all <= 1e-5);
        assert_float_eq!(operands[0][1], 914.40182880, abs_all <= 1e-5);
        assert_float_eq!(operands[0][2], 500., abs_all <= 1e-5);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-5);

        assert_eq!(successes, 1);

        // Inverse + roundtrip
        ctx.apply(op, Inv, &mut operands)?;
        assert_float_eq!(operands[0][0], 1000.0, abs_all <= 1e-5);
        assert_float_eq!(operands[0][1], 1000.0, abs_all <= 1e-5);
        assert_float_eq!(operands[0][2], 500., abs_all <= 1e-9);
        assert_float_eq!(operands[0][3], 1., abs_all <= 1e-9);
        Ok(())
    }
}
