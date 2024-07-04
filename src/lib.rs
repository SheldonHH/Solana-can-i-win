use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};

// 定义Solana程序的入口点
entrypoint!(process_instruction);

// 处理指令函数
fn process_instruction(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // 解析指令数据
    let (max_choosable_integer, desired_total) = parse_instruction_data(instruction_data)?;

    // 计算 can_i_win 的结果
    let result = can_i_win(max_choosable_integer, desired_total);

    // 打印结果
    msg!("Result: {}", result);

    Ok(())
}

// 解析指令数据函数
pub fn parse_instruction_data(data: &[u8]) -> Result<(i32, i32), ProgramError> {
    if data.len() != 8 {
        return Err(ProgramError::InvalidInstructionData);
    }
    let max_choosable_integer = i32::from_le_bytes([data[0], data[1], data[2], data[3]]);
    let desired_total = i32::from_le_bytes([data[4], data[5], data[6], data[7]]);
    Ok((max_choosable_integer, desired_total))
}

use std::collections::HashMap;

// can_i_win 函数实现
pub fn can_i_win(max_choosable_integer: i32, desired_total: i32) -> bool {
    if desired_total <= 0 {
        println!("目标和小于等于0，玩家直接胜利");
        return true;
    }
    if max_choosable_integer * (max_choosable_integer + 1) / 2 < desired_total {
        println!("所有可选数字的总和小于目标和，不可能获胜");
        return false;
    }

    let mut dp: HashMap<i32, bool> = HashMap::new();

    fn can_win(status: i32, rest: i32, dp: &mut HashMap<i32, bool>, max_choosable_integer: i32) -> bool {
        println!("当前状态: {}, 剩余目标: {}", status, rest);
        if rest <= 0 {
            println!("剩余目标小于等于0，当前玩家失败");
            return false;
        }
        if let Some(&result) = dp.get(&status) {
            println!("状态{}已计算过，结果: {}", status, result);
            return result;
        }
        for i in 1..=max_choosable_integer {
            if (status & (1 << i)) == 0 {
                println!("尝试选择数字: {}", i);
                if !can_win(status | (1 << i), rest - i, dp, max_choosable_integer) {
                    dp.insert(status, true);
                    println!("选择数字{}后，当前玩家胜利", i);
                    return true;
                }
            } else {
                println!("数字{}已被选择过，跳过", i);
            }
        }
        dp.insert(status, false);
        println!("所有选项尝试完毕，当前玩家失败");
        return false;
    }

    can_win(0, desired_total, &mut dp, max_choosable_integer)
}
