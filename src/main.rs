use core::panic;
use dirs;
use rand::Rng;
use std::{process::Command, str::FromStr};
use once_cell::sync::OnceCell;
static INSTANCE: OnceCell<String> = OnceCell::new();
enum VariableType{
    WhichChain, // 5,6,7
    UserIndex, 
    Asset
}
use VariableType::*;
fn main() {
    let home_dir = match dirs::home_dir() {
        Some(path) => String::from_str(path.to_str().unwrap()).unwrap(),
        None => panic!(),
    };
    INSTANCE.set(format!("{}/.foundry/bin/cast abi-encode", home_dir)).unwrap();
    let mut cast = Command::new("sh");

    let mut rng = rand::thread_rng();
    let operation_type = rng.gen_range(0..3);
    let t=match operation_type {
        0 => get_deposit_args(0,&mut cast),
        1 => get_deposit_and_withdraw_args(1,&mut cast),
        2 => get_withdraw(2,&mut cast),
        _ => panic!(),
    };

    print!("{}", t);
}

fn get_deposit_args(operation_type:u8, cast_command: &mut std::process::Command)-> String {
    assert_eq!(operation_type,0);
    let command_arg=INSTANCE.get().unwrap();

    let which_chain=get_different_number(WhichChain);
    let asset=get_different_number(Asset);

    let decimals=check_decimals(which_chain, asset);

    let args=format!(
        "{} \"fun(uint8,uint64,uint8,uint8,uint8,uint8,uint256)\" {} {} {} {} {} {} {}", 
        command_arg,

        operation_type,
        which_chain,
        get_different_number(UserIndex),
        asset,
        get_different_number(UserIndex),
        get_different_number(UserIndex),
        get_amount(decimals)
    );

    cast_command.arg("-c")
        .arg(args);

    let output = cast_command.output().unwrap();
    String::from_utf8(output.stdout).unwrap()

}

fn get_deposit_and_withdraw_args(operation_type:u8, cast_command: &mut std::process::Command)-> String {
    assert_eq!(operation_type,1);
    let command_arg=INSTANCE.get().unwrap();

    let which_chain=get_different_number(WhichChain);
    let asset=get_different_number(Asset);

    let decimals=check_decimals(which_chain, asset);
    
    
    cast_command.arg("-c")
        .arg(format!(
            "{} \"fun(uint8,uint64,uint8,uint8,uint8,uint8,uint8,uint64,uint256)\" {} {} {} {} {} {} {} {} {}", 
            command_arg,

            operation_type,
            which_chain,
            get_different_number(UserIndex),
            asset,
            get_different_number(UserIndex),
            get_different_number(UserIndex),
            get_different_number(UserIndex),
            get_different_number(WhichChain),
            get_amount(decimals)
        ));

    let output = cast_command.output().unwrap();
    String::from_utf8(output.stdout).unwrap()

    
}

fn get_withdraw(operation_type:u8, cast_command: &mut std::process::Command)-> String {
    assert_eq!(operation_type,2);
    let command_arg=INSTANCE.get().unwrap();
    
    cast_command.arg("-c")
        .arg(format!(
            "{} \"fun(uint8,uint8,uint8,uint8,uint64,uint256)\" {} {} {} {} {} {}", 
            command_arg,

            operation_type,
            get_different_number(UserIndex),
            get_different_number(Asset),
            get_different_number(UserIndex),
            get_different_number(WhichChain),
            get_amount(18)
        ));

    let output = cast_command.output().unwrap();
    String::from_utf8(output.stdout).unwrap()
}

fn get_different_number(vt:VariableType)-> u64 {
    match vt {
        VariableType::WhichChain=>{
            let mut rng = rand::thread_rng();
            rng.gen_range(5..8)
        },
        VariableType::UserIndex=>{
            let mut rng = rand::thread_rng();
            rng.gen_range(0..100)
        },
        VariableType::Asset=>{
            let mut rng = rand::thread_rng();
            rng.gen_range(0..3)
        }
    }
}

fn get_amount(decimal:u8)->String{
    let number_length=match decimal{
        18=>{
            let mut rng = rand::thread_rng();
            rng.gen_range(1..23)
        },
        6=>{
            let mut rng = rand::thread_rng();
            rng.gen_range(1..11)
        },
        _=>panic!()
    };
    let mut long_number=String::from("");
    if number_length==22 {
        long_number.push(std::char::from_digit(1, 10).unwrap());
    }else{
        let mut rng = rand::thread_rng();
        long_number.push(std::char::from_digit(rng.gen_range(1..10), 10).unwrap());
    }

    for _ in 0..number_length-1 {
        let mut rng = rand::thread_rng();
        long_number.push(std::char::from_digit(rng.gen_range(0..10), 10).unwrap());
    }

    long_number     
}

fn check_decimals(which_chain:u64,asset:u64)->u8{
    if (which_chain==5||which_chain==7) && (asset==1 || asset==2) {
        6
    }else{
        18
    }
}