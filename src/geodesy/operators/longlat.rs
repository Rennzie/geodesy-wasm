/// As far as I can tell the `longlat` operator is a no-op.
/// - https://proj.org/en/9.3/operations/conversions/latlon.html
/// - https://github.com/OSGeo/PROJ/blob/2040e685f5ab9c2958b7b611f5aaafee21fed82f/src/projections/latlong.cpp#L94
///
/// I assume this is for legacy PROJ reasons therefore in geodesy-wasm latlong is a no-op.
use geodesy_rs::authoring::*;

// ----- C O M M O N -------------------------------------------------------------------

// ----- F O R W A R D -----------------------------------------------------------------

fn fwd(_op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    operands.len()
}

// ----- I N V E R S E -----------------------------------------------------------------

fn inv(_op: &Op, _ctx: &dyn Context, operands: &mut dyn CoordinateSet) -> usize {
    operands.len()
}

// ----- C O N S T R U C T O R ---------------------------------------------------------

#[rustfmt::skip]
pub const GAMUT: [OpParameter; 0] = [
];

pub fn new(parameters: &RawParameters, _ctx: &dyn Context) -> Result<Op, Error> {
    Op::plain(parameters, InnerOp(fwd), Some(InnerOp(inv)), &GAMUT, _ctx)
}

// ----- T E S T S ---------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    const GDA94: Coor4D = Coor4D([-4052051.7643, 4212836.2017, -2545106.0245, 0.0]);

    #[test]
    // Borrowed from Rust Geodesy `noop` inner_op test
    fn no_change() -> Result<(), Error> {
        let mut ctx = Minimal::default();
        ctx.register_op("longlat", OpConstructor(new));
        let op = ctx.op("longlat xy_in=us-ft z_in=us-ft")?;

        // EPSG:1134 - 3 parameter, ED50/WGS84, s = sqrt(27) m
        let mut operands = [GDA94];

        // Forward
        ctx.apply(op, Fwd, &mut operands)?;
        assert_eq!(operands[0], GDA94);

        // Inverse + roundtrip
        ctx.apply(op, Inv, &mut operands)?;
        assert_eq!(operands[0], GDA94);
        Ok(())
    }
}
