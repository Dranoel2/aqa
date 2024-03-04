use std::{env, fs, io, path::Path};

fn main() -> io::Result<()> {
    let mut contents = String::new();

    let variants = ["Int", "Float"];
    let operators = [
        ("Add", "+"),
        ("Subtract", "-"),
        ("Multiply", "*"),
        ("Divide", "/"),
    ];

    let comparisons = [
        ("LessThan", "<"),
        ("LessThanOrEqualTo", "<="),
        ("GreaterThan", ">"),
        ("GreaterThanOrEqualTo", ">="),
    ];

    let mut combinations = Vec::new();

    for variant in variants {
        for (operator_variant, operator_char) in operators {
            combinations.push((variant, variant, operator_variant, operator_char));
        }
        for (comparison_variant, comparison_char) in comparisons {
            combinations.push((variant, "Bool", comparison_variant, comparison_char))
        }
        combinations.push((variant, "Bool", "EqualTo", "=="));
        combinations.push((variant, "Bool", "NotEqualTo", "!="));
    }

    combinations.push(("String", "String", "Add", "+ &"));

    contents += "match (left_value, operator.token_type, right_value) {\n";

    for (variant, result, operator_variant, operator_char) in combinations {
        contents += &format!(
            "\t(Value::{}(left_value), TokenType::{}, Value::{}(right_value)) => Ok(Value::{}(left_value {} right_value)),\n",
            variant, operator_variant, variant, result, operator_char,
        );
    }

    contents += "\t_ => Err(Error::new(ErrorType::MismatchedType))
}\n";

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("binary.rs");
    fs::write(dest_path, contents)
}
