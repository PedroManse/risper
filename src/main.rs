use risper::*;

//(def print-twice ('
//    (print this)
//    (print this)
//) this)
//('' print-twice "hello")
//
//(print "hello")
//
//(print "what's your name?")
//
//(def name (input))
//
//(print "hello")
//(print name)
//
//(def infix ('
//    (op v1 v2)
//) v1 op v2)

// m-implement
// -
// def
// '/"
// =

// u-implement
// if/else
// contants

const CODE_EXAMPLE: &'static str = r#"
(def true ('
    a
) a b)

(def false ('
    b
) a b)

(def if ('
    (''('' if-else expr-true expr-false))
) if-else expr-true -syn-else expr-false)

('' if true ('
    print "true"
) else ('
    print "false"
) )

(def _for ('
    ('' if (= index to)) (') else ('
        ('' code index)
        ('' _for (- -1 index) to code)
    )
) index to code )

(def for ('
    ('' _for init end code )
) init -syn-arrow end code)

('' for 0 -> 100 ('
    (print index)
))

"#;

fn main() -> eyre::Result<()> {
    let x = into_tokens(CODE_EXAMPLE)?;
    let t = into_expr(x)?;
    println!("{t:#?}");
    Ok(())
}

//use std::collections::HashMap;
//pub struct RispRunner {
//    env: RispEnv,
//    code: Vec<RispExp>,
//}

//struct RispEnv {
//    stack: Vec<HashMap<String, RispExp>>
//}
