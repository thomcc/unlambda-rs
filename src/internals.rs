//! The interpreter guts, mostly.

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Func {
    V,
    I,
    E,
    C,
    D,
    At,
    Pipe,
    R,
    K,
    S,
    Dot(char),
    Q(char),
    Op(Box<OpFunc>),
}

impl Func {
    pub fn k1(o: Func) -> Self {
        Self::Op(Box::new(OpFunc::K1(o)))
    }
    pub fn s1(x: Func) -> Self {
        Self::Op(Box::new(OpFunc::S1(x)))
    }
    pub fn s2(x: Func, y: Func) -> Self {
        Self::Op(Box::new(OpFunc::S2(x, y)))
    }
    pub fn cont(c: P<Cont>) -> Self {
        Self::Op(Box::new(OpFunc::Cont(c)))
    }
    pub fn d1(v: P<Expr>) -> Self {
        Self::Op(Box::new(OpFunc::D1(v)))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OpFunc {
    K1(Func),
    S1(Func),
    S2(Func, Func),
    Cont(P<Cont>),
    D1(P<Expr>),
}

impl Func {
    pub fn apply_to(self, ctx: &mut Ctx, operand: Func, cont: P<Cont>) -> Result<Task, Error> {
        Ok(match self {
            Self::V => cont.invoke(self),
            Self::I => cont.invoke(operand),
            Self::E => Task::Final,
            Self::C => Task::App(operand, Func::cont(cont.clone()), cont),
            Self::R => {
                ctx.putc('\n')?;
                cont.invoke(operand)
            }
            Self::D => cont.invoke(Func::d1(p(Expr::Func(operand)))),

            Self::At => {
                let f = if ctx.getc()?.is_some() {
                    Func::I
                } else {
                    Func::V
                };
                Task::App(operand, f, cont)
            }

            Self::Pipe => {
                let f = if let Some(c) = ctx.last_char() {
                    Func::Dot(c)
                } else {
                    Func::V
                };
                Task::App(f, operand, cont)
            }
            Self::Dot(c) => {
                ctx.putc(c)?;
                cont.invoke(operand)
            }
            Self::Q(ch) => {
                let f = if ctx.last_char() == Some(ch) {
                    Func::I
                } else {
                    Func::V
                };
                Task::App(f, operand, cont)
            }
            Self::S => cont.invoke(Func::s1(operand)),
            Self::K => cont.invoke(Func::k1(operand)),

            Self::Op(o) => match *o {
                OpFunc::K1(v) => cont.invoke(v),

                OpFunc::S1(x) => cont.invoke(Func::s2(x, operand)),
                OpFunc::S2(x, y) => {
                    let operand = p(Expr::Func(operand));
                    Task::Eval(
                        p(Expr::App(
                            p(Expr::App(p(Expr::Func(x)), operand.clone())),
                            p(Expr::App(p(Expr::Func(y)), operand)),
                        )),
                        cont,
                    )
                }
                OpFunc::Cont(c) => c.invoke(operand),
                OpFunc::D1(promise) => {
                    let cont = Cont::Del(operand, cont);
                    Task::Eval(promise, p(cont))
                }
            },
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Expr {
    Func(Func),
    App(P<Expr>, P<Expr>),
}

impl Expr {
    pub fn eval(self, cont: P<Cont>) -> Task {
        match self {
            Self::Func(f) => cont.invoke(f),
            Self::App(operator, operand) => Task::Eval(operator, p(Cont::App1(operand, cont))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Cont {
    /// `(operand, cont)`:
    App1(P<Expr>, P<Cont>),
    /// `(operator, cont)` Have evaluated operator, waiting for operand.
    App(Func, P<Cont>),
    Del(Func, P<Cont>),
    Final,
}

impl Cont {
    pub fn invoke(&self, val: Func) -> Task {
        match self {
            Self::App1(operand, cont) => Task::App1(val, operand.clone(), cont.clone()),
            Self::App(operator, cont) => Task::App(operator.clone(), val, cont.clone()),
            Self::Del(operand, cont) => Task::App(val, operand.clone(), cont.clone()),
            Self::Final => Task::Final,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Task {
    Eval(P<Expr>, P<Cont>),
    /// (operator, operand, cont)
    App1(Func, P<Expr>, P<Cont>),
    /// (operator, operand, cont)
    App(Func, Func, P<Cont>),
    Final,
}

impl Task {
    pub fn run(self, ctx: &mut Ctx) -> Result<Option<Task>, Error> {
        match self {
            Self::Eval(expr, cont) => Ok(Some((&*expr).clone().eval(cont))),
            Self::App1(Func::D, operand, cont) => Ok(Some(cont.invoke(Func::d1(operand)))),
            Self::App1(operator, operand, cont) => {
                Ok(Some((&*operand).clone().eval(p(Cont::App(operator, cont)))))
            }
            Self::App(operator, operand, cont) => operator.apply_to(ctx, operand, cont).map(Some),
            Self::Final => Ok(None),
        }
    }
}
