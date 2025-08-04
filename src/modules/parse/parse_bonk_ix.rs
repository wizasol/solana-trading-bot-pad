use crate::{
    read_string, read_u64_le, BonkConstantCurve, BonkCurveParams, BonkFixedCurve, BonkLinearCurve, BonkMintParams, BonkVestingParams, BonkfunMIntInfo
};

pub fn parse_mint_params(data: &[u8], offset: &mut usize) -> BonkMintParams {
    let decimals = data[*offset];
    *offset += 1;

    let name = read_string(data, offset);
    let symbol = read_string(data, offset);
    let uri = read_string(data, offset);

    BonkMintParams {
        decimals,
        name,
        symbol,
        uri,
    }
}

pub fn parse_constant_curve(data: &[u8], offset: &mut usize) -> BonkConstantCurve {
    let supply = read_u64_le(data, offset);
    let total_base_sell = read_u64_le(data, offset);
    let total_quote_fund_raising = read_u64_le(data, offset);
    let migrate_type = data[*offset];
    *offset += 1;

    BonkConstantCurve {
        supply,
        total_base_sell,
        total_quote_fund_raising,
        migrate_type,
    }
}

pub fn parse_fixed_curve(data: &[u8], offset: &mut usize) -> BonkFixedCurve {
    let supply = read_u64_le(data, offset);
    let total_quote_fund_raising = read_u64_le(data, offset);
    let migrate_type = data[*offset];
    *offset += 1;

    BonkFixedCurve {
        supply,
        total_quote_fund_raising,
        migrate_type,
    }
}

pub fn parse_linear_curve(data: &[u8], offset: &mut usize) -> BonkLinearCurve {
    let supply = read_u64_le(data, offset);
    let total_quote_fund_raising = read_u64_le(data, offset);
    let migrate_type = data[*offset];
    *offset += 1;

    BonkLinearCurve {
        supply,
        total_quote_fund_raising,
        migrate_type,
    }
}

pub fn parse_curve_params(data: &[u8], offset: &mut usize) -> BonkCurveParams {
    let curve_type = data[*offset];
    *offset += 1;

    match curve_type {
        0 => BonkCurveParams::Constant(parse_constant_curve(data, offset)),
        1 => BonkCurveParams::Fixed(parse_fixed_curve(data, offset)),
        2 => BonkCurveParams::Linear(parse_linear_curve(data, offset)),
        // Add other curve types here
        _ => panic!("Unknown curve type"),
    }
}

pub fn parse_vesting_params(data: &[u8], offset: &mut usize) -> BonkVestingParams {
    let total_locked_amount = read_u64_le(data, offset);
    let cliff_period = read_u64_le(data, offset);
    let unlock_period = read_u64_le(data, offset);

    BonkVestingParams {
        total_locked_amount,
        cliff_period,
        unlock_period,
    }
}

pub fn parse_bonk_initialize_params(data: &[u8]) -> BonkfunMIntInfo {
    let mut offset: usize = 8;

    let base_mint_param = parse_mint_params(data, &mut offset);
    let curve_param = parse_curve_params(data, &mut offset);
    let vesting_param = parse_vesting_params(data, &mut offset);

    BonkfunMIntInfo {
        base_mint_param,
        curve_param,
        vesting_param,
    }
}
