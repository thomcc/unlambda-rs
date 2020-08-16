use unlambda::*;

macro_rules! assert_evals_to {
    ($src:expr, $expect:expr $(,)?) => {
        assert_evals_to!($src, $expect, "");
    };
    ($src:expr, $expect:expr, $input:expr $(,)?) => {
        let e = eval_to_string($src, Input::Str($input)).unwrap_or_else(|e| {
            panic!("Failed to evaluate {:?}: {:?}", $src, e);
        });
        assert_eq!(&*e, $expect, "{:?} did not evaluate as expected", $src);
    };
}

#[test]
fn hellos() {
    assert_evals_to!("`.!`.d`.l`.r`.o`.w`. `.,`.o`.l`.l`.e`.Hi", "Hello, world!");
    assert_evals_to!(
        "
        ```si`k``s.H``s.e``s.l``s.l``s.o``s. \
        ``s.w``s.o``s.r``s.l``s.d``s.!``sri
        ``si``si``si``si``si``si``si``si`ki
        ",
        "Hello world!\nHello world!\nHello world!\nHello world!\nHello world!\nHello world!\nHello world!\nHello world!\n",
    );
}

#[test]
fn cat() {
    assert_evals_to!("``cd``d`@|`cd", "example", "example");
    assert_evals_to!("``cd``d`@|`cd", "", "");
    assert_evals_to!("``cd``d`@|`cd", "1234", "1234");
}

#[test]
fn church() {
    assert_evals_to!(
        "```si`k``s.f``s.o``s.o``s.p``s. i``si``si``si`ki",
        "foop foop foop ",
    );
}

#[test]
fn quine0() {
    let quine = concat!(
        "``d.v```s``si`kv``si`k`d`..`.c`.s`.``.``.s`.``.`v",
        "``s``sc.```s``sc.```s``sc.d``s``sc..``s``sc.v``s`",
        "`sc.```s``sc.```s``sc.```s``sc.s``s``sc.```s``sc.",
        "```s``sc.s``s``sc.i``s``sc.```s``sc.k``s``sc.v``s",
        "``sc.```s``sc.```s``sc.s``s``sc.i``s``sc.```s``sc",
        ".k``s``sc.```s``sc.d``s``sc.```s``sc..``s``sc..``",
        "s``sc.```s``sc..``s``sc.c``s``sc.```s``sc..``s``s",
        "c.s``s``sc.```s``sc..``s``sc.```s``sc.```s``sc..`",
        "`s``sc.```s``sc.```s``sc..``s``sc.s``s``sc.```s``",
        "sc..``s``sc.```s``sc.```s``sc..``s``sc.```s``sc.vv"
    );
    assert_evals_to!(quine, quine);
}

#[test]
fn quine10() {
    let quine = include_str!("fixtures/quine10.unl");
    assert_evals_to!(quine, quine);
}

#[test]
fn squares() {
    assert_evals_to!(
        "`r```si`k``s ``s`kk `si ``s``si`k ``s`k`s`k ``sk ``sr`k.* i r``si``si``si``si``si``si``si``si``si`k`ki",
        include_str!("fixtures/square_out.txt"),
    );
}

#[test]
fn unl_test_g0() {
    assert_evals_to!("`r`.*i", "*\n");
    assert_evals_to!("`r`d`.*i", "\n");
    assert_evals_to!("`r``d.*i", "*\n");
    assert_evals_to!("`r``id`.*i", "\n");
    assert_evals_to!("`r``dd`.*i", "*\n");
    assert_evals_to!("`r```kdi`.*i", "\n");
    assert_evals_to!("`r```sd.*i", "*\n");
    assert_evals_to!("`r```s`kd`.*ii", "*\n");
}

#[test]
fn unl_test_g1() {
    // Abstraction elimination from `.*i
    assert_evals_to!("`r```s`k.*`kii", "*\n");
    assert_evals_to!("`r``k`.*ii", "*\n");
    assert_evals_to!("`r```si`ki.*", "*\n");
    assert_evals_to!("`r```s`k.*ii", "*\n");
}

#[test]
fn unl_test_g2() {
    // Abstraction elimination from `d`.*i
    assert_evals_to!("`r```s`kd``s`k.*`kii", "\n");
    assert_evals_to!("`r```s`kd`k`.*ii", "*\n");
    assert_evals_to!("`r``k`d`.*ii", "\n");
    assert_evals_to!("`r```si``s`k.*`kid", "\n");
    assert_evals_to!("`r```si`k`.*id", "*\n");
    assert_evals_to!("`r```s`kd``sii.*", "\n");
    assert_evals_to!("`r```s`kd``s`k.*ii", "\n");
    assert_evals_to!("`r```s`kd.*i", "\n");
}
#[test]
fn unl_test_g4() {
    // Abstraction elimination from ``d.*i
    assert_evals_to!("`r```s``s`kd`k.*`kii", "*\n");
    assert_evals_to!("`r```s`k`d.*`kii", "*\n");
    assert_evals_to!("`r``k``d.*ii", "*\n");
    assert_evals_to!("`r```s``si`k.*`kid", "*\n");
    assert_evals_to!("`r```s``s`kdi`ki.*", "*\n");
    assert_evals_to!("`r```sd`ki.*", "*\n");
    assert_evals_to!("`r```s``s`kd`k.*ii", "*\n");
    assert_evals_to!("`r```s`k`d.*ii", "*\n");
}

#[test]
fn unl_test_g5() {
    // Abstraction elimination from ``id`.*i
    assert_evals_to!("`r```s``s`ki`kd``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``s`ki`kd`k`.*ii", "*\n");
    assert_evals_to!("`r```s`k`id``s`k.*`kii", "\n");
    assert_evals_to!("`r```s`k`id`k`.*ii", "*\n");
    assert_evals_to!("`r``k``id`.*ii", "\n");
    assert_evals_to!("`r```s``si`kd``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``si`kd`k`.*ii", "*\n");
    assert_evals_to!("`r```s``s`ki`kd``s`k.*ii", "\n");
    assert_evals_to!("`r```s`k`id``s`k.*ii", "\n");
    assert_evals_to!("`r```s``s`ki`kd.*i", "\n");
    assert_evals_to!("`r```s`k`id.*i", "\n");
    assert_evals_to!("`r```s``si`kd``s`k.*ii", "\n");
    assert_evals_to!("`r```s``si`kd.*i", "\n");
    assert_evals_to!("`r```s``s`kii``s`k.*`kid", "\n");
    assert_evals_to!("`r```s``s`kii`k`.*id", "*\n");
    assert_evals_to!("`r```si``s`k.*`kid", "\n");
    assert_evals_to!("`r```si`k`.*id", "*\n");
    assert_evals_to!("`r```s``s`ki`kd``si`ki.*", "\n");
    assert_evals_to!("`r```s`k`id``si`ki.*", "\n");
}
#[test]
fn unl_test_g6() {
    // Abstraction elimination from ``dd`.*i
    assert_evals_to!("`r```s``s`kd`kd``s`k.*`kii", "*\n");
    assert_evals_to!("`r```s``s`kd`kd`k`.*ii", "*\n");
    assert_evals_to!("`r```s`k`dd``s`k.*`kii", "*\n");
    assert_evals_to!("`r```s`k`dd`k`.*ii", "*\n");
    assert_evals_to!("`r``k``dd`.*ii", "*\n");
    assert_evals_to!("`r```s``s`kd`kd``s`k.*ii", "*\n");
    assert_evals_to!("`r```s`k`dd``s`k.*ii", "*\n");
    assert_evals_to!("`r```s``s`kd`kd.*i", "*\n");
    assert_evals_to!("`r```s`k`dd.*i", "*\n");
    assert_evals_to!("`r```s``s`kdi``s`k.*`kid", "*\n");
    assert_evals_to!("`r```s``s`kdi`k`.*id", "*\n");
    assert_evals_to!("`r```sd``s`k.*`kid", "*\n");
    assert_evals_to!("`r```sd`k`.*id", "*\n");
    assert_evals_to!("`r```s``si`kd``s`k.*`kid", "*\n");
    assert_evals_to!("`r```s``si`kd`k`.*id", "*\n");
    assert_evals_to!("`r```s``sii``s`k.*`kid", "*\n");
    assert_evals_to!("`r```s``sii`k`.*id", "*\n");
    assert_evals_to!("`r```s``s`kd`kd``si`ki.*", "*\n");
    assert_evals_to!("`r```s`k`dd``si`ki.*", "*\n");
}
#[test]
fn unl_test_g7() {
    // Abstraction elimination from ```kdi`.*i
    assert_evals_to!("`r```s``s``s`kk`kd`ki``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``s``s`kk`kd`ki`k`.*ii", "*\n");
    assert_evals_to!("`r```s``s`k`kd`ki``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``s`k`kd`ki`k`.*ii", "*\n");
    assert_evals_to!("`r```s`k``kdi``s`k.*`kii", "\n");
    assert_evals_to!("`r```s`k``kdi`k`.*ii", "*\n");
    assert_evals_to!("`r``k```kdi`.*ii", "\n");
    assert_evals_to!("`r```s``s``si`kd`ki``s`k.*`kik", "\n");
    assert_evals_to!("`r```s``s``si`kd`ki`k`.*ik", "*\n");
    assert_evals_to!("`r```s``s``s`kki`ki``s`k.*`kid", "\n");
    assert_evals_to!("`r```s``s``s`kki`ki`k`.*id", "*\n");
    assert_evals_to!("`r```s``sk`ki``s`k.*`kid", "\n");
    assert_evals_to!("`r```s``sk`ki`k`.*id", "*\n");
    assert_evals_to!("`r```s``s``s`kk`kdi``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``s`k`kdi``s`k.*`kii", "\n");
    assert_evals_to!("`r```s`kd``s`k.*`kii", "\n");
    assert_evals_to!("`r```s``s``s`kk`kdi`k`.*ii", "*\n");
    assert_evals_to!("`r```s``s``s`kk`kd`ki``s`k.*ii", "\n");
    assert_evals_to!("`r```s``s`k`kd`ki``s`k.*ii", "\n");
    assert_evals_to!("`r```s`k``kdi``s`k.*ii", "\n");
    assert_evals_to!("`r```s``s``s`kk`kdi.*i", "\n");
    assert_evals_to!("`r```s``s`k`kd`ki.*i", "\n");
    assert_evals_to!("`r```s`k``kdi.*i", "\n");
    assert_evals_to!("`r```s``s``s`kk`kdi``s`k.*ii", "\n");
    assert_evals_to!("`r```s``s`k`kdi``s`k.*ii", "\n");
    assert_evals_to!("`r```s``s``s`kk`kd`ki``si`ki.*", "\n");
    assert_evals_to!("`r```s``s`k`kd`ki``si`ki.*", "\n");
    assert_evals_to!("`r```s`k``kdi``si`ki.*", "\n");
}
#[test]
fn unl_test_g8() {
    // Abstraction elimination from ```s`kd.*i
    assert_evals_to!("`r```s``s``s`ks``s`kk`kd`k.*`kii", "\n");
    assert_evals_to!("`r```s``s``s`ks`k`kd`k.*`kii", "\n");
    assert_evals_to!("`r```s``s`k`s`kd`k.*`kii", "\n");
    assert_evals_to!("`r```s`k``s`kd.*`kii", "\n");
    assert_evals_to!("`r``k```s`kd.*ii", "\n");
    assert_evals_to!("`r```s``s``si``s`kk`kd`k.*`kis", "\n");
    assert_evals_to!("`r```s``s``si`k`kd`k.*`kis", "\n");
    assert_evals_to!("`r```s``s``s`ks``si`kd`k.*`kik", "\n");
    assert_evals_to!("`r```s``s``s`ks``s`kki`k.*`kid", "\n");
    assert_evals_to!("`r```s``s``s`ksk`k.*`kid", "\n");
    assert_evals_to!("`r```s``s``s`ks``s`kk`kdi`ki.*", "\n");
    assert_evals_to!("`r```s``s``s`ks`k`kdi`ki.*", "\n");
    assert_evals_to!("`r```s``s`k`s`kdi`ki.*", "\n");
    assert_evals_to!("`r```s`s`kd`ki.*", "\n");
    assert_evals_to!("`r```s``s``s`ks``s`kk`kd`k.*ii", "\n");
    assert_evals_to!("`r```s``s``s`ks`k`kd`k.*ii", "\n");
    assert_evals_to!("`r```s``s`k`s`kd`k.*ii", "\n");
    assert_evals_to!("`r```s`k``s`kd.*ii", "\n");
}

#[test]
fn unl_test_g9() {
    // And a few more...
    assert_evals_to!("`r```s``s``s`ks``s`kk`kd``ssi`k.*i", "\n");
    assert_evals_to!("`r```s``s``s`ks``s`kk`kds`k.*i", "\n");
    assert_evals_to!("`r```s``s``s`ks`k`kd``ssi`k.*i", "\n");
    assert_evals_to!("`r```s``s``s`ks`k`kds`k.*i", "\n");
    assert_evals_to!("`r```s``s`k`s`kd``ssi`k.*i", "\n");
    assert_evals_to!("`r```s``s`k`s`kds`k.*i", "\n");
}
