use super::*;

/// Standard library knowledge base.
pub fn std() -> Vec<Knowledge> {
    vec![
        Def(False1, Ret(Bool(false))),
        Def(Not, _if(false, true)),
        Def(Idb, _if(true, false)),
        Def(True1, Ret(Bool(true))),
        Def(And, _if(_if(true, false), false)),
        Def(Or, _if(true, _if(true, false))),
        Def(Eqb, _if(_if(true, false), _if(false, true))),
        Def(Xor, _if(_if(false, true), _if(true, false))),
        Def(Nand, _if(_if(false, true), true)),
        Def(Nor, _if(false, _if(false, true))),
        Def(Exc, _if(_if(false, true), false)),
        Def(Imply, _if(_if(true, false), true)),
        Def(Fstb, _if(true, false)),
        Def(Sndb, _if(_if(true, false), _if(true, false))),

        // `x(y, z) => x(y)(z)`
        Red(app("x", head_tail_tup("y", "z")), app2("x", "y", "z")),
        // `x{y, z} => x{y}{z}`
        Red(constr("x", head_tail_tup("y", "z")), constr(constr("x", "y"), "z")),
        // `x{y}{z}(a)(b) => x{y}(a){z}(b)`
        Red(app2(constr(constr("x", "y"), "z"), "a", "b"),
            app(constr(app(constr("x", "y"), "a"), "z"), "b")),
        // `(g, f)(y, z) => (g(y)(z), f(y)(z))`
        Red(app(("g", "f"), head_tail_tup("y", "z")),
           (app2("g", "y", "z"), app2("f", "y", "z")).into()),
        // `if(x, _)(true) => x`
        Red(app(_if("x", Any), true), "x".into()),
        // `if(_, x)(false) => x`
        Red(app(_if(Any, "x"), false), "x".into()),
        // `if(x, _){_}(true) => x`
        Red(constr(app(_if("x", Any), Any), true), "x".into()),
        // `if(_, x){_}(false) => x`
        Red(constr(app(_if(Any, "x"), Any), false), "x".into()),
        // `(x) => x`
        Red(Tup(vec!["x".into()]), "x".into()),
        // `\x(_) => x`
        Red(app(ret_var("x"), Any), "x".into()),
        // `∃(\x) => eq(x)`
        Red(app(Ex, ret_var("x")), app(Eq, "x")),
        // `∃(f{f}) => idb`
        Red(app(Ex, constr("f", "f")), Idb.into()),
        // `x() => x`
        Red(app("x", Tup(vec![])), "x".into()),
        // `f[g -> g] => f[g]`
        Red(path("f", ("g", "g")), path("f", "g")),
        // `f[g x g -> g] => f[g]`
        Red(path("f", ("g", "g", "g")), path("f", "g")),
        // `∀(f{g}) => g`
        Red(app(Triv, constr("f", "g")), "g".into()),
        // `∀(f) => \true`
        Red(app(Triv, no_constr("f")), true.into()),

        // `not . not <=> idb`
        Red(comp(Not, Not), Idb.into()),
        // `not[not] <=> not`
        Red(path(Not, Not), Not.into()),
        // `x . id => x`
        Red(comp("x", Id), "x".into()),
        // `id . x` => x
        Red(comp(Id, "x"), "x".into()),
        // `x[id] => x`
        Red(path("x", Id), "x".into()),
        // `id[x] => id`
        Red(path(Id, "x"), Id.into()),
        // `and[not] => or`.
        Red(path(And, Not), Or.into()),
        // `or[not] => and`.
        Red(path(Or, Not), And.into()),
        // `xor[not] => eqb`.
        Red(path(Xor, Not), Eqb.into()),
        // `eqb[not] => xor`.
        Red(path(Eqb, Not), Xor.into()),
        // `nand[not] => nor`
        Red(path(Nand, Not), Nor.into()),
        // `nor[not] => nand`
        Red(path(Nor, Not), Nand.into()),
        // `nand[not x not -> id] => and[not]`
        Red(path(Nand, (Not, Not, Id)), path(And, Not)),

        // `add[even] => eqb`.
        Red(path(Add, Even), Eqb.into()),
        // `add[odd] => xor`.
        Red(path(Add, Odd), Xor.into()),
        // `mul[even] => or`.
        Red(path(Mul, Even), Or.into()),
        // `mul[odd] => and`.
        Red(path(Mul, Odd), And.into()),
        // `not . even => odd`.
        Red(comp(Not, Even), Odd.into()),
        // `not . odd => even`
        Red(comp(Not, Odd), Even.into()),

        // `add[exp] => mul`
        Red(path(Add, Exp), Mul.into()),
        // `mul[ln] => add`
        Red(path(Mul, Ln), Add.into()),
        // `exp . ln => id`
        Red(comp(Exp, Ln), Id.into()),
        // `ln . exp => id`
        Red(comp(Ln, Exp), Id.into()),
        // `neg . neg => id`
        Red(comp(Neg, Neg), Id.into()),

        // `false1(_) => false`
        Red(app(False1, Any), false.into()),
        // `true1(_) => true`
        Red(app(True1, Any), true.into()),
        // `id(x) => x`
        Red(app(Id, "x"), "x".into()),
        // `and(true) => idb`
        Red(app(And, true), Idb.into()),
        // `and(false) => false1`
        Red(app(And, false), False1.into()),
        // `or(true) => true1`
        Red(app(Or, true), True1.into()),
        // `or(false) => idb`
        Red(app(Or, false), Idb.into()),
        // `fstb(x)(y) => x`
        Red(app2(Fstb, "x", "y"), "x".into()),
        // `fst(x)(y) => x`
        Red(app2(Fst, "x", "y"), "x".into()),
        // `sndb(x)(y) => y`
        Red(app2(Sndb, "x", "y"), "y".into()),
        // `snd(x)(y) => y`
        Red(app2(Snd, "x", "y"), "y".into()),
        // `eqb(false) => not`
        Red(app(Eqb, false), Not.into()),
        // `eqb(true) => idb`
        Red(app(Eqb, true), Idb.into()),

        // `lt(\x)(\y) => \x < \y`
        Red(app2(Lt, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Lt)),
        // `le(\x)(\y) => \x <= \y`
        Red(app2(Le, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Le)),
        // `gt(\x)(\y) => \x > \y`
        Red(app2(Gt, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Gt)),
        // `ge(\x)(\y) => \x >= \y`
        Red(app2(Ge, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Ge)),
        // `neg(\x) => -x`
        Red(app(Neg, ret_var("x")), unop_ret_var("x", Neg)),
        // `add(\x)(\y) => x + y`
        Red(app2(Add, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Add)),
        // `sub(\x)(\y) => x - y`
        Red(app2(Sub, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Sub)),
        // `mul(\x)(\y) => x * y`
        Red(app2(Mul, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Mul)),
        // `div(\x)(\y) => x / y`
        Red(app2(Div, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Div)),
        // `rem(\x)(\y) => x % y`
        Red(app2(Rem, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Rem)),
        // `pow(\x)(\y) => x ^ y`
        Red(app2(Pow, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Pow)),
        // `rpow(\x)(\y) => x ^ y`
        Red(app2(Rpow, ret_var("x"), ret_var("y")), binop_ret_var("y", "x", Pow)),
        // `eq(\x)(\y) => x == y`
        Red(app2(Eq, ret_var("x"), ret_var("y")), binop_ret_var("x", "y", Eq)),
        // `push([x..], y) => compute::push(x, y)`
        Red(app2(Push, list_var("x"), "y"), binop_ret_var("x", "y", Push)),
        // `push_front([x..], y) => compute::push_front(x, y)`
        Red(app2(PushFront, list_var("x"), "y"), binop_ret_var("x", "y", PushFront)),
        // `concat{(: vec)}(x){(: vec)}(y) => x ++ y`
        Red(app(constr(app(constr(Concat, app(Rty, VecType)), "x"), app(Rty, VecType)),
                "y"), binop_ret_var("x", "y", Concat)),
        // `len(x) => compute::len(x)`
        Red(app(Len, "x"), unop_ret_var("x", Len)),

        // `mul[neg] => (neg . mul)`
        Red(path(Mul, Neg), comp(Neg, Mul)),

        // `add(0)(x) => x`
        Red(app2(Add, 0.0, "x"), "x".into()),
        // `add(x)(0) => x`
        Red(app2(Add, "x", 0.0), "x".into()),
        // `mul(1)(x) => x`
        Red(app2(Mul, 1.0, "x"), "x".into()),
        // `mul(x)(1) => x`
        Red(app2(Mul, "x", 1.0), "x".into()),
        // `mul(0) => 0`
        Red(app(Mul, 0.0), 0.0.into()),
        // `mul(_)(0) => 0`
        Red(app2(Mul, Any, 0.0), 0.0.into()),

        // `f(x : \)(y : \) => f(x)(y) : \`
        concrete_op(Add),
        concrete_op(Sub),
        concrete_op(Mul),
        concrete_op(Div),
        concrete_op(Rem),
        concrete_op(Pow),
        concrete_op(Rpow),

        // Component-wise vector operations.
        vec_op(Add),
        vec_op(Sub),
        vec_op(Mul),
        vec_op(Div),
        vec_op(Rem),
        vec_op(Pow),
        vec_op(Rpow),

        // `vec_op(f)([x0, y0..])([x1, y1..]) => concat([f(x0)(x1)])(vec_op(f)(y0)(y1))`
        Red(app2(app(VecOp, "f"), head_tail_list("x0", "y0"), head_tail_list("x1", "y1")),
            app2(Concat, List(vec![app2("f", "x0", "x1")]), app2(app(VecOp, "f"), "y0", "y1"))),
        // `vec_op(f)([x])([y]) => [f(x)(y)]`
        Red(app2(app(VecOp, "f"), singleton("x"), singleton("y")), List(vec![app2("f", "x", "y")])),

        // `dot([x0, y0])([x1, y1]) => add(mul(x0)(x1))(mul(y0)(y1))`
        Red(app2(Dot, vec2("x0", "y0"), vec2("x1", "y1")),
            app2(Add, app2(Mul, "x0", "x1"), app2(Mul, "y0", "y1"))),

        // `concat[len] => add`
        Red(path(Concat, Len), Add.into()),
        // `concat[sum] => add`
        Red(path(Concat, Sum), Add.into()),
        // `concat[min] => min2`
        Red(path(Concat, Min), Min2.into()),
        // `concat[max] => max2`
        Red(path(Concat, Max), Max2.into()),

        // `mul_mat[det] => mul`
        Red(path(MulMat, Det), Mul.into()),
        // `mul_mat[fst . dim x snd . dim -> dim] => id`
        Red(path(MulMat, (comp(Fst, Dim), comp(Snd, Dim), Dim)), Id.into()),

        // `if(a, b)[not -> id] => if(b, a)`.
        Red(path(_if("a", "b"), (Not, Id)), _if("b", "a")),
        // `f[id -> g] => g . f`.
        Red(path("f", (Id, "g")), comp("g", "f")),
        // `f[id x id -> g] => g . f`
        Red(path("f", (Id, Id, "g")), comp("g", "f")),
        // `not . (not . x) => x`.
        Red(comp(Not, comp(Not, "x")), "x".into()),
        // `and . (le, ge) => eq`
        Red(comp(And, (Le, Ge)), Eq.into()),

        // `and{eq} => fstb`
        Red(constr(And, Eq), Fstb.into()),
        // `or{eq} => fstb`
        Red(constr(Or, Eq), Fstb.into()),
        // `eq{eq} => \true`
        Red(constr(Eq, Eq), true.into()),
        // `sub{eq} => \0`
        Red(constr(Sub, Eq), 0.0.into()),
        // `add{eq}(x, _) => mul(2)(x)`
        Red(app2(constr(Add, Eq), "x", Any), app2(Mul, 2.0, "x")),
        // `mul{eq}(x, _) => pow(x)(2)`
        Red(app2(constr(Mul, Eq), "x", Any), app2(Pow, "x", 2.0)),
        // `\x{eq}(_) => \x`
        Red(app(constr(ret_var("x"), Eq), Any), "x".into()),
        // `f(a)(a) => f{eq}(a)(a)`
        Red(app2(no_constr("f"), "a", "a"), app2(constr("f", Eq), "a", "a")),
        // `f{true2} => f`
        Red(constr("f", True2), "f".into()),
        // `f{true1} => f`
        Red(constr("f", True1), "f".into()),

        // `∃(false1) => not`
        Red(app(Ex, False1), Not.into()),
        // `∃(not) => true1`
        Red(app(Ex, Not), True1.into()),
        // `∃(idb) => true1`
        Red(app(Ex, Idb), True1.into()),
        // `∃(true1) => idb`
        Red(app(Ex, True1), Idb.into()),
        // `∃(and) => true1`
        Red(app(Ex, And), True1.into()),
        // `∃(or) => true1`
        Red(app(Ex, Or), True1.into()),
        // `∃(nand) => true1`
        Red(app(Ex, Nand), True1.into()),
        // `∃(nor) => true1`
        Red(app(Ex, Nor), True1.into()),
        // `∃(xor) => true1`
        Red(app(Ex, Xor), True1.into()),
        // `∃(eqb) => true1`
        Red(app(Ex, Eqb), True1.into()),
        // `∃(exc) => true1`
        Red(app(Ex, Exc), True1.into()),
        // `∃(imply) => true1`
        Red(app(Ex, Imply), True1.into()),
        // `∃(fstb) => true1`
        Red(app(Ex, Fstb), True1.into()),
        // `∃(sndb) => true1`
        Red(app(Ex, Sndb), True1.into()),
        // `∃(id) => \true`
        Red(app(Ex, Id), true.into()),

        // `idb => id`
        Red(Idb.into(), Id.into()),
        // `fstb => fst`
        Red(Fstb.into(), Fst.into()),
        // `sndb => snd`
        Red(Sndb.into(), Snd.into()),
        // `eqb => eq`
        Red(Eqb.into(), Eq.into()),

        // `len . concat => concat[len] . (len . fst, len . snd)`
        Red(comp(Len, Concat), comp(path(Concat, Len), (comp(Len, Fst), comp(Len, Snd)))),
        // `sum . concat => concat[sum] . (sum . fst, sum . snd)`
        Red(comp(Sum, Concat), comp(path(Concat, Sum), (comp(Sum, Fst), comp(Sum, Snd)))),
        // `(f . fst)(a)(_) => f(a)`
        Red(app2(comp("f", Fst), "a", Any), app("f", "a")),
        // `(f . snd)(_)(a) => f(a)`
        Red(app2(comp("f", Snd), Any, "a"), app("f", "a")),
        // `(f . fst){_}(a)(_) => f(a)`
        Red(app2(constr(comp("f", Fst), Any), "a", Any), app("f", "a")),
        // `(f . snd){_}(_)(a) => f(a)`
        Red(app2(constr(comp("f", Snd), Any), Any, "a"), app("f", "a")),

        // `(x, y) . (a, b) => (x . a, y . b)`.
        Red(comp(("x", "y"), ("a", "b")), (comp("x", "a"), comp("y", "b")).into()),
        // `(x, y, z) . (a, b, c) => (x . a, y . b, z . c)`.
        Red(comp(("x", "y", "z"), ("a", "b", "c")),
            (comp("x", "a"), comp("y", "b"), comp("z", "c")).into()),
        // `h . f[g -> id] => f[g -> h]`.
        Red(comp("h", path("f", ("g", Id))), path("f", ("g", "h"))),
        // `h . f[g0 x g1 -> id] => f[g0 x g1 -> h]`.
        Red(comp("h", path("f", ("g0", "g1", Id))), path("f", ("g0", "g1", "h"))),

        // `add(pow(cos(x))(\2))(pow(sin(x))(\2)) <=> 1`
        Red(app2(Add, app2(Pow, app(Cos, "x"), 2.0),
                      app2(Pow, app(Sin, "x"), 2.0)), 1.0.into()),

        // `f([x..]) => f{(: vec)}(x)`
        Red(app(no_constr("f"), list_var("x")), app(constr("f", app(Rty, VecType)), "x")),

        // `not . nand <=> and`.
        Eqv(comp(Not, Nand), And.into()),
        // `not . nor <=> or`.
        Eqv(comp(Not, Nor), Or.into()),
        // `not . and <=> nand`.
        Eqv(comp(Not, And), Nand.into()),
        // `not . or <=> nor`.
        Eqv(comp(Not, Or), Nor.into()),
        // `not . eqb <=> xor`.
        Eqv(comp(Not, Eqb), Xor.into()),
        // `not . xor <=> eqb`.
        Eqv(comp(Not, Xor), Eqb.into()),

        // `and(a)(b) <=> and(b)(a)`
        commutative(And),
        // `or(a)(b) <=> or(b)(a)`
        commutative(Or),
        // `nand(a)(b) <=> nand(b)(a)`
        commutative(Nand),
        // `nor(a)(b) <=> nor(b)(a)`
        commutative(Nor),
        // `xor(a)(b) <=> xor(b)(a)`
        commutative(Xor),
        // `eq(a)(b) <=> eq(b)(a)`
        commutative(Eq),
        // `add(a)(b) <=> add(b)(a)`
        commutative(Add),
        // `mul(a)(b) <=> mul(b)(a)`
        commutative(Mul),
        // `add(a)(add(b)(c)) <=> add(add(a)(b))(c)`
        associative(Add),
        // `mul(a)(mul(b)(c)) <=> mul(mul(a)(b))(c)`
        associative(Mul),
        // `mul(a)(add(b)(c)) <=> add(mul(a)(b))(mul(a)(c))`
        distributive(Mul, Add),

        // `f[g][h] <=> f[h . g]`.
        Eqv(path(path("f", "g"), "h"), path("f", comp("h", "g"))),
        // `f . (g . h) <=> (f . g) . h`.
        Eqv(comp("f", comp("g", "h")), comp(comp("f", "g"), "h")),
        // `f[g] <=> f[g -> id][id -> g]`
        Eqv(path("f", "g"), path(path("f", ("g", Id)), (Id, "g"))),
        // `(f . (g0, g1))(a)(b) <=> f(g0(a)(b))(g1(a)(b))`
        Eqv(app2(comp("f", ("g0", "g1")), "a", "b"),
            app2("f", app2("g0", "a", "b"), app2("g1", "a", "b"))),
        // `(f . (g0, g1))(a) <=> f(g0(a))(g1(a))`
        Eqv(app(comp("f", ("g0", "g1")), "a"), app2("f", app("g0", "a"), app("g1", "a"))),
        // `(f . g)(a)(b) <=> f(g(a)(b))`
        Eqv(app2(comp("f", "g"), "a", "b"), app("f", app2("g", "a", "b"))),
        // `(f . g){x}(a){y}(b) <=> f(g{x}(a){y}(b))`
        Eqv(app(constr(app(constr(comp("f", "g"), "x"), "a"), "y"), "b"),
            app("f", app(constr(app(constr("g", "x"), "a"), "y"), "b"))),
        // `(f . g)(a) <=> f(g(a))`
        Eqv(app(comp("f", "g"), "a"), app("f", app("g", "a"))),
        // `(f . g){x}(a){y}(b) <=> (f . g{x}{y})(a)(b)`
        Eqv(app(constr(app(constr(comp("f", "g"), "x"), "a"), "y"), "b"),
            app2(comp("f", constr(constr("g", "x"), "y")), "a", "b")),
        // `(f . g){x}(a) <=> f(g{x}(a))`
        Eqv(app(constr(comp("f", "g"), "x"), "a"), app("f", app(constr("g", "x"), "a"))),
        // `(g . f){_}(a){_}(b) <=> f[g](g(a))(g(b))`
        Eqv(app(constr(app(constr(comp("g", "f"), Any), "a"), Any), "b"),
            app(app(path("f", "g"), app("g", "a")), app("g", "b"))),
        // `(g . f){_}(a) <=> f[g](g(a))`
        Eqv(app(constr(comp("g", "f"), Any), "a"), app(path("f", "g"), app("g", "a"))),
        // `(g . f)(a)(b) <=> f[g](g(a))(g(b))`
        Eqv(app(app(comp("g", "f"), "a"), "b"), app2(path("f", "g"), app("g", "a"), app("g", "b"))),
        // `(g . f)(a) <=> f[g](g(a))`
        Eqv(app(comp("g", "f"), "a"), app(path("f", "g"), app("g", "a"))),
        // `(g, f)(a) <=> (g(a), f(a))`
        Eqv(app(("g", "f"), "a"), (app("g", "a"), app("f", "a")).into()),
        // `f . (g0, g1)(a) <=> (f . (g0, g1))(a)`
        Eqv(comp("f", app(("g0", "g1"), "a")), app(comp("f", ("g0", "g1")), "a")),
        // `(g . f){h} <=> g . f{h}`
        Eqv(constr(comp("g", "f"), "h"), comp("g", constr("f", "h"))),
    ]
}
