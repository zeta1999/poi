use std::sync::Arc;
use std::fmt;
use super::Expr;

/// Contains symbols and operators on symbols.
#[derive(Clone, PartialEq, Debug)]
pub enum Symbol {
    /// The wildcard symbol `_`.
    Any,
    /// A variable bound from context.
    ///
    /// This can be anything.
    Var(Arc<String>),
    /// A list variable.
    ListVar(Arc<String>),
    /// A list variable of length 1.
    ///
    /// Lifts the value out of the list at binding.
    Singleton(Arc<String>),
    /// A head-tail pattern match on a tuple.
    ///
    /// This requires the tuple to have at least length 2.
    /// It is to avoid cycles between reductions.
    HeadTailTup(Box<Expr>, Box<Expr>),
    /// A head-tail pattern match on a list.
    ///
    /// This requires the list to have at least length 2.
    /// It is to avoid cycles between reductions.
    HeadTailList(Box<Expr>, Box<Expr>),
    /// A value variable.
    ///
    /// This requires the expression to be `Ret` variant.
    /// It is used in special rules such as `(\k)(x) => \k`.
    RetVar(Arc<String>),
    /// Compute a binary function.
    ///
    /// This is used when the right side of the rule computes something from two left side expressions.
    BinopRetVar(Arc<String>, Arc<String>, Box<Symbol>),
    /// Compute a unary function.
    ///
    /// This is used when the right side of the rule computes something from a left side expression.
    UnopRetVar(Arc<String>, Box<Symbol>),
    /// A function without domain constraints.
    NoConstrVar(Arc<String>),
    /// `\false` for one argument.
    False1,
    /// `not`.
    Not,
    /// `id` for booleans.
    Idb,
    /// `\true` for one argument.
    True1,
    /// `\false` for two arguments.
    False2,
    /// `\true` for two arguments.
    True2,
    /// `and`.
    And,
    /// `or`.
    Or,
    /// `eq` for booleans.
    Eqb,
    /// `xor`.
    Xor,
    /// `nand`.
    Nand,
    /// `nor`.
    Nor,
    /// `exc`.
    Exc,
    /// `imply`.
    Imply,
    /// `fst` for booleans.
    Fstb,
    /// `snd` for booleans.
    Sndb,
    /// `even`.
    Even,
    /// `odd`.
    Odd,
    /// `lt`.
    Lt,
    /// `le`.
    Le,
    /// `gt`.
    Gt,
    /// `ge`.
    Ge,
    /// `neg`.
    Neg,
    /// `add`.
    Add,
    /// `sub`.
    Sub,
    /// `mul`.
    Mul,
    /// `div`.
    Div,
    /// `rem`.
    Rem,
    /// `pow`.
    Pow,
    /// `rpow`.
    Rpow,
    /// `sqrt`.
    Sqrt,
    /// `ln`.
    Ln,
    /// `log2`.
    Log2,
    /// `log10`.
    Log10,
    /// `exp`.
    Exp,
    /// `len`.
    Len,
    /// `concat`.
    Concat,
    /// `sum`.
    Sum,
    /// `min2`.
    Min2,
    /// `max2`.
    Max2,
    /// `min`.
    Min,
    /// `max`.
    Max,
    /// `mul_mat`.
    MulMat,
    /// `det`.
    Det,
    /// `dim`.
    Dim,
    /// `fst`.
    Fst,
    /// `snd`.
    Snd,
    /// `sin`.
    Sin,
    /// `asin`.
    Asin,
    /// `cos`.
    Cos,
    /// `acos`.
    Acos,
    /// `tan`.
    Tan,
    /// `atan`.
    Atan,
    /// `atan2`.
    Atan2,
    /// `dot`.
    Dot,
    /// `push`.
    Push,
    /// `push_front`.
    PushFront,
    /// `el`.
    El,
    /// Generic `id`.
    Id,
    /// Generic `eq`.
    Eq,
    /// Generic `neq`.
    Neq,
    /// `if`.
    ///
    /// This is used in Boolean functions.
    If,
    /// Existential path `∃`.
    Ex,
    /// Trivial path `∀`.
    Triv,
    /// `\`, the type of `\x`.
    RetType,
    /// The type of lists.
    VecType,
    /// The judgement `(: a)(b)`.
    Rty,
    /// Applies a function component-wise to lists.
    VecOp,
}

impl fmt::Display for Symbol {
    fn fmt(&self, w: &mut fmt::Formatter<'_>) -> std::result::Result<(), fmt::Error> {
        use Symbol::*;

        match self {
            False1 => write!(w, "false1")?,
            Not => write!(w, "not")?,
            Idb => write!(w, "idb")?,
            True1 => write!(w, "true1")?,
            False2 => write!(w, "false2")?,
            True2 => write!(w, "true2")?,
            And => write!(w, "and")?,
            Or => write!(w, "or")?,
            Eqb => write!(w, "eqb")?,
            Xor => write!(w, "xor")?,
            Nand => write!(w, "nand")?,
            Nor => write!(w, "nor")?,
            Exc => write!(w, "exc")?,
            Imply => write!(w, "imply")?,
            Fstb => write!(w, "fstb")?,
            Sndb => write!(w, "sndb")?,
            Even => write!(w, "even")?,
            Odd => write!(w, "odd")?,
            Lt => write!(w, "lt")?,
            Le => write!(w, "le")?,
            Gt => write!(w, "gt")?,
            Ge => write!(w, "ge")?,
            Neg => write!(w, "neg")?,
            Add => write!(w, "add")?,
            Sub => write!(w, "sub")?,
            Mul => write!(w, "mul")?,
            Div => write!(w, "div")?,
            Rem => write!(w, "rem")?,
            Pow => write!(w, "pow")?,
            Rpow => write!(w, "rpow")?,
            Sqrt => write!(w, "sqrt")?,
            Ln => write!(w, "ln")?,
            Log2 => write!(w, "log2")?,
            Log10 => write!(w, "log10")?,
            Exp => write!(w, "exp")?,
            Len => write!(w, "len")?,
            Concat => write!(w, "concat")?,
            Sum => write!(w, "sum")?,
            Min2 => write!(w, "min2")?,
            Max2 => write!(w, "max2")?,
            Min => write!(w, "min")?,
            Max => write!(w, "max")?,
            MulMat => write!(w, "mul_mat")?,
            Det => write!(w, "det")?,
            Dim => write!(w, "dim")?,
            Fst => write!(w, "fst")?,
            Snd => write!(w, "snd")?,
            Sin => write!(w, "sin")?,
            Asin => write!(w, "asin")?,
            Cos => write!(w, "cos")?,
            Acos => write!(w, "acos")?,
            Tan => write!(w, "tan")?,
            Atan => write!(w, "atan")?,
            Atan2 => write!(w, "atan2")?,
            Dot => write!(w, "dot")?,
            Push => write!(w, "push")?,
            PushFront => write!(w, "push_front")?,
            El => write!(w, "el")?,
            Id => write!(w, "id")?,
            Eq => write!(w, "eq")?,
            Neq => write!(w, "neq")?,
            If => write!(w, "if")?,
            Any => write!(w, "_")?,
            Ex => write!(w, "∃")?,
            Triv => write!(w, "∀")?,
            RetType => write!(w, "\\")?,
            VecType => write!(w, "vec")?,
            Rty => write!(w, "rty")?,
            VecOp => write!(w, "vec_op")?,
            Var(x) | NoConstrVar(x) => write!(w, "{}", x)?,
            RetVar(x) => write!(w, "\\{}", x)?,
            ListVar(x) => write!(w, "[{}..]", x)?,
            Singleton(x) => write!(w, "[{}]", x)?,
            HeadTailTup(x, y) => write!(w, "({}, {}..)", x, y)?,
            HeadTailList(x, y) => write!(w, "[{}, {}..]", x, y)?,
            BinopRetVar(x, y, f) => {
                match **f {
                    Lt => write!(w, "{} < {}", x, y)?,
                    Le => write!(w, "{} <= {}", x, y)?,
                    Gt => write!(w, "{} > {}", x, y)?,
                    Ge => write!(w, "{} >= {}", x, y)?,
                    Add => write!(w, "{} + {}", x, y)?,
                    Sub => write!(w, "{} - {}", x, y)?,
                    Mul => write!(w, "{} * {}", x, y)?,
                    Div => write!(w, "{} / {}", x, y)?,
                    Pow => write!(w, "{} ^ {}", x, y)?,
                    Rem => write!(w, "{} % {}", x, y)?,
                    Eq => write!(w, "{} == {}", x, y)?,
                    Concat => write!(w, "{} ++ {}", x, y)?,
                    Push => write!(w, "compute::push({}, {})", x, y)?,
                    PushFront => write!(w, "compute::push_front({}, {})", x, y)?,
                    _ => write!(w, "{:?}", self)?,
                }
            }
            UnopRetVar(x, f) => {
                match **f {
                    Neg => write!(w, "-{}", x)?,
                    Len => write!(w, "compute::len({})", x)?,
                    _ => write!(w, "{:?}", self)?,
                }
            }
            // _ => write!(w, "{:?}", self)?,
        }
        Ok(())
    }
}
