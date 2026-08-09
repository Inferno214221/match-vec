use syn::{Arm, Attribute, Expr, ExprMatch, Ident, Pat, PatConst, PatIdent, PatLit, PatPath, PatRange, PatReference, PatRest, PatSlice, PatStruct, PatTuple, PatTupleStruct, PatWild, Token, punctuated::Punctuated, token::{Brace, Bracket}};

pub struct MatchArgs {
    pub attrs: Vec<Attribute>,
    pub expr: Box<Expr>,
    pub brace_token: Brace,
    pub arms: Vec<VecArm>,
}

impl TryFrom<ExprMatch> for MatchArgs {
    type Error = ();

    fn try_from(value: ExprMatch) -> Result<Self, Self::Error> {
        let ExprMatch {
            attrs, match_token: _, expr, brace_token, arms
        } = value;
        Ok(MatchArgs {
            attrs, expr, brace_token,
            arms: arms.into_iter().map(VecArm::try_from).try_collect()?
        })
    }
}

pub struct VecArm {
    pub attrs: Vec<Attribute>,
    pub pat: VecPat,
    pub fat_arrow_token: Token![=>],
    pub body: Box<Expr>,
    pub comma: Option<Token![,]>,
}

impl TryFrom<Arm> for VecArm {
    type Error = ();

    fn try_from(value: Arm) -> Result<Self, Self::Error> {
        let Arm {
            attrs, pat, fat_arrow_token, body, comma
        } = value;
        Ok(VecArm {
            attrs, fat_arrow_token, body, comma,
            pat: pat.try_into()?
        })
    }
}

pub enum VecPat {
    Const(PatConst),
    // Guard(PatGuard),
    Ident(VecPatIdent),
    Lit(PatLit),
    // Or(PatOr),
    // Paren(PatParen),
    Path(PatPath),
    Range(PatRange),
    Reference(PatReference),
    Struct(PatStruct),
    Tuple(PatTuple),
    TupleStruct(PatTupleStruct),
    Rest(PatRest),
    Slice(VecPatSlice),
    Wild(PatWild),
}

impl TryFrom<Pat> for VecPat {
    type Error = ();

    fn try_from(value: Pat) -> Result<Self, Self::Error> {
        Ok(match value {
            Pat::Const(const_)    => VecPat::Const(const_),
            Pat::Ident(ident)     => VecPat::Ident(VecPatIdent::try_from(ident)?),
            Pat::Lit(lit)         => VecPat::Lit(lit),
            Pat::Path(path)       => VecPat::Path(path),
            Pat::Range(range)     => VecPat::Range(range),
            Pat::Reference(ref_)  => VecPat::Reference(ref_),
            Pat::Rest(rest)       => VecPat::Rest(rest),
            Pat::Slice(slice)     => VecPat::Slice(VecPatSlice::try_from(slice)?),
            Pat::Struct(struct_)  => VecPat::Struct(struct_),
            Pat::Tuple(tup)       => VecPat::Tuple(tup),
            Pat::TupleStruct(tup) => VecPat::TupleStruct(tup),
            Pat::Wild(wild)       => VecPat::Wild(wild),
            _ => Err(())?
        })
    }
}

pub struct VecPatIdent {
    pub attrs: Vec<Attribute>,
    pub by_ref: Option<Token![ref]>,
    pub mutability: Option<Token![mut]>,
    pub ident: Ident,
    pub subpat: Option<(Token![@], Box<VecPat>)>,
}

impl TryFrom<PatIdent> for VecPatIdent {
    type Error = ();

    fn try_from(value: PatIdent) -> Result<Self, Self::Error> {
        let PatIdent { attrs, by_ref, mutability, ident, subpat } = value;
        Ok(VecPatIdent {
            attrs, by_ref, mutability, ident,
            subpat: match subpat {
                Some((at, boxed)) => Some((
                    at,
                    Box::new(VecPat::try_from(*boxed)?)
                )),
                None => None,
            }
        })
    }
}

pub struct VecPatSlice {
    pub attrs: Vec<Attribute>,
    pub bracket_token: Bracket,
    pub elems: Punctuated<VecPat, Token![,]>,
}

impl TryFrom<PatSlice> for VecPatSlice {
    type Error = ();

    fn try_from(value: PatSlice) -> Result<Self, Self::Error> {
        let PatSlice { attrs, bracket_token, elems } = value;
        Ok(VecPatSlice {
            attrs, bracket_token,
            elems: elems.into_iter().map(VecPat::try_from).try_collect()?
        })
    }
}