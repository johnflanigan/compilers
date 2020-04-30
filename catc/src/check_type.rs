use crate::checked_grammar::{
    CheckedDec, CheckedExp, CheckedLValue, CheckedProgram, CheckedTopLevelDec, FunctionType,
    GenerateTypeId, GlobalTypeInfo, Type, TypeId, B,
};
use crate::common::{Label, LabelGenerator, Symbol, SymbolGenerator};
use crate::source_grammar::{Dec, Exp, LValue, Program, TopLevelDec};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub struct TypeError(pub &'static str);

#[derive(Debug, Clone)]
struct Context {
    f: HashMap<String, Label>,
    s: HashMap<String, Symbol>,
    t: HashMap<String, TypeId>,
}

#[derive(Debug, Clone)]
struct StackedContext {
    context: Context,
    rest: Option<Box<StackedContext>>,
}

impl StackedContext {
    fn new(gti: &mut GlobalTypeInfo) -> Self {
        let int_id = gti.new_type_id(Type::Int);
        let str_id = gti.new_type_id(Type::Str);
        let void_id = gti.new_type_id(Type::Void);
        let s = vec![].into_iter().collect();
        let t = vec![
            ("int".to_string(), int_id),
            ("string".to_string(), str_id),
            ("void".to_string(), void_id),
        ]
        .into_iter()
        .collect();

        // TODO add primitives
        let f = vec![].into_iter().collect();
        StackedContext {
            context: Context { f, s, t },
            rest: None,
        }
    }

    fn stack(
        self,
        f: HashMap<String, Label>,
        s: HashMap<String, Symbol>,
        t: HashMap<String, TypeId>,
    ) -> Self {
        Self {
            context: Context { f, s, t },
            rest: Some(Box::new(self)),
        }
    }

    fn lookup_t(&self, name: &String) -> Option<TypeId> {
        self.context
            .t
            .get(name)
            .cloned()
            .or_else(|| self.rest.as_ref().and_then(|s| s.lookup_t(name)))
    }

    fn lookup_s(&self, name: &String) -> Option<Symbol> {
        self.context
            .s
            .get(name)
            .cloned()
            .or_else(|| self.rest.as_ref().and_then(|s| s.lookup_s(name)))
    }

    fn lookup_f(&self, name: &String) -> Option<Label> {
        self.context
            .f
            .get(name)
            .cloned()
            .or_else(|| self.rest.as_ref().and_then(|s| s.lookup_f(name)))
    }

    fn declare_type_array(
        &mut self,
        gti: &mut GlobalTypeInfo,
        new_type: String,
        element_type: String,
    ) -> Result<(), TypeError> {
        let element_type_id = match self.lookup_t(&element_type) {
            None => return Err(TypeError("no such type in array declaration")),
            Some(type_id) => match gti.lookup_c(type_id) {
                None => return Err(TypeError("element type not defined")),
                Some(_tau) => type_id,
            },
        };

        let new_type_id = gti.new_type_id(Type::Array(element_type_id));

        *self = self.clone().stack(
            HashMap::new(),
            HashMap::new(),
            vec![(new_type, new_type_id)].into_iter().collect(),
        );
        Ok(())
    }

    fn declare_type_record(
        &mut self,
        gti: &mut GlobalTypeInfo,
        new_type: String,
        feild_decs: VecDeque<(String, String)>,
    ) -> Result<(), TypeError> {
        pairwise_diffrent(&feild_decs)?;

        let feild_decs: Vec<(String, TypeId)> = feild_decs
            .iter()
            .cloned()
            .map(|(id, type_name)| match self.lookup_t(&type_name) {
                None => Err(TypeError("no such type in record declaration")),
                Some(type_id) => match gti.lookup_c(type_id) {
                    None => Err(TypeError("element_type not defined")),
                    Some(_tau) => Ok((id, type_id)),
                },
            })
            .collect::<Result<_, _>>()?;

        let new_type_id = gti.new_type_id(Type::Record(feild_decs));

        *self = self.clone().stack(
            HashMap::new(),
            HashMap::new(),
            vec![(new_type, new_type_id)].into_iter().collect(),
        );

        Ok(())
    }

    fn declare_function(
        &mut self,
        gti: &mut GlobalTypeInfo,
        name: String,
        return_type: String,
        args: VecDeque<(String, String)>,
        exp: Exp,
    ) -> Result<CheckedTopLevelDec, TypeError> {
        pairwise_diffrent(&args)?;

        let arguments: Vec<(String, Symbol, TypeId)> = args
            .iter()
            .cloned()
            .map(|(id, type_name)| match self.lookup_t(&type_name) {
                None => Err(TypeError("no such type in function declaration")),
                Some(type_id) => match gti.lookup_c(type_id) {
                    None => Err(TypeError("arg type not defined")),
                    Some(_) => Ok((id, gti.gen_sym.new_symbol(), type_id)),
                },
            })
            .collect::<Result<_, _>>()?;

        let return_type: TypeId = match self.lookup_t(&return_type) {
            None => Err(TypeError("no such type in function declaration")),
            Some(return_type_id) => match gti.lookup_c(return_type_id) {
                None => return Err(TypeError("arg type not defined")),
                Some(_) => Ok(return_type_id),
            },
        }?;

        let new_name = if name == "main" {
            Label::Main
        } else {
            gti.gen_label.new_label()
        };

        for i in arguments.clone().into_iter() {
            gti.insert_gamma(i.1, i.2)?;
        }

        gti.insert_f(
            &new_name,
            FunctionType {
                return_type,
                arguments: arguments.clone().iter().map(|i| (i.1, i.2)).collect(),
            },
        )?;

        let (type_, cexp) = type_check_exp(
            gti,
            &self.clone().stack(
                vec![(name.clone(), new_name)].into_iter().collect(),
                arguments.clone().into_iter().map(|i| (i.0, i.1)).collect(),
                HashMap::new(),
            ),
            false,
            exp,
        )?;

        if gti.types.get(&return_type).unwrap() != &type_ {
            return Err(TypeError("Function body does not evaluate to proper type"));
        }

        *self = self.clone().stack(
            vec![(name, new_name)].into_iter().collect(),
            HashMap::new(),
            HashMap::new(),
        );

        Ok(CheckedTopLevelDec::FunDec {
            name: new_name,
            args: arguments.into_iter().map(|i| (i.1)).collect(),
            body: Box::new(cexp),
        })
    }

    fn declare_variable(
        self,
        gti: &mut GlobalTypeInfo,
        name: String,
        value_type: String,
        value: Exp,
    ) -> Result<(Self, CheckedDec), TypeError> {
        let (type_, cexp) = type_check_exp(gti, &self, false, value)?;

        let dec_type: TypeId = match self.lookup_t(&value_type) {
            None => Err(TypeError("no such type in var declaration")),
            Some(dec_type_id) => match gti.lookup_c(dec_type_id) {
                None => {
                    return Err(TypeError(
                        "internal error: couldn't find type associated with type id",
                    ))
                }
                Some(_) => Ok(dec_type_id),
            },
        }?;

        let tp = gti.types.get(&dec_type).unwrap();

        if tp != &type_ {
            return Err(TypeError("declaration doesn't match exp"));
        }

        let s = gti.gen_sym.new_symbol();
        gti.insert_gamma(s, dec_type)?;

        Ok((
            self.stack(
                HashMap::new(),
                vec![(name, s)].into_iter().collect(),
                HashMap::new(),
            ),
            CheckedDec::VarDec {
                name: s,
                value: cexp,
            },
        ))
    }
}

pub fn type_check(program: Program) -> Result<CheckedProgram, TypeError> {
    let mut type_info = GlobalTypeInfo {
        function_symbols: HashMap::new(),
        symbol_table: HashMap::new(),
        types: HashMap::new(),
        gen_type: GenerateTypeId::new(),
        gen_sym: SymbolGenerator::new(),
        gen_label: LabelGenerator::new(),
    };

    let mut found_main = false;

    let mut sc = StackedContext::new(&mut type_info);

    let mut dec_list = VecDeque::new();

    for dec in program.dec_list.into_iter() {
        match dec {
            TopLevelDec::TyDecArray {
                new_type,
                element_type,
            } => {
                sc.declare_type_array(&mut type_info, new_type, element_type)?;
            }
            TopLevelDec::TyDecRecord {
                new_type,
                feild_decs,
            } => {
                sc.declare_type_record(&mut type_info, new_type, feild_decs)?;
            }
            TopLevelDec::FunDec {
                name,
                return_type,
                args,
                body,
            } => {
                if name == "main" {
                    if args.is_empty() && (return_type == "int" || return_type == "void") {
                        found_main = true;
                    } else {
                        return Err(TypeError(
                            "Main doesn't return int/void or main takes more than 0 args",
                        ));
                    }
                }

                let func = sc.declare_function(&mut type_info, name, return_type, args, *body)?;
                dec_list.push_back(func);
            }
        }
    }
    if found_main {
        Ok(CheckedProgram {
            dec_list,
            function_symbols: type_info.function_symbols,
            symbol_table: type_info.symbol_table,
            types: type_info.types,
            gen_sym: type_info.gen_sym,
            gen_label: type_info.gen_label,
        })
    } else {
        Err(TypeError("No Main Found"))
    }
}

fn type_check_exp(
    gti: &mut GlobalTypeInfo,
    c: &StackedContext,
    brk: B,
    exp: Exp,
) -> Result<(Type, CheckedExp), TypeError> {
    match exp {
        Exp::Break => {
            if brk {
                Ok((Type::Void, CheckedExp::Break))
            } else {
                Err(TypeError("Break appearing in place it shouldn't"))
            }
        }
        Exp::IntLit { value } => Ok((Type::Int, CheckedExp::IntLit { value })),
        Exp::StringLit { value } => Ok((Type::Str, CheckedExp::StringLit { value })),
        Exp::LValue { lvalue } => {
            let (tp, clvalue) = type_check_lvalue(gti, c, false, lvalue)?;
            Ok((tp, CheckedExp::LValue { lvalue: clvalue }))
        }
        Exp::Sequence { sequence } => {
            let mut seq = VecDeque::new();
            let mut tp = None;
            for exp in sequence.into_iter() {
                let (tpe, cexp) = type_check_exp(gti, c, brk, exp)?;
                tp = Some(tpe);
                seq.push_back(cexp);
            }
            let final_type = match tp {
                None => Type::Void,
                Some(tpe) => tpe,
            };

            Ok((final_type, CheckedExp::Sequence { sequence: seq }))
        }
        Exp::Negate { exp } => {
            let (tp, cexp) = type_check_exp(gti, c, brk, *exp)?;
            match tp {
                Type::Int => Ok((
                    Type::Int,
                    CheckedExp::Negate {
                        exp: Box::new(cexp),
                    },
                )),
                _ => Err(TypeError("Negate contains non-integer")),
            }
        }
        Exp::Infix { left, op, right } => {
            let (tp_l, lexp) = type_check_exp(gti, c, brk, *left)?;
            let (tp_r, rexp) = type_check_exp(gti, c, brk, *right)?;
            match (tp_l, tp_r) {
                (Type::Int, Type::Int) => Ok((
                    Type::Int,
                    CheckedExp::Infix {
                        left: Box::new(lexp),
                        op,
                        right: Box::new(rexp),
                    },
                )),
                _ => Err(TypeError("Not Both int on either side of infix op")),
            }
        }
        Exp::ArrayCreate {
            type_id,
            length,
            inital_value,
        } => {
            let (tp_init, init_exp) = type_check_exp(gti, c, brk, *inital_value)?;
            let (tp_len, len_exp) = type_check_exp(gti, c, brk, *length)?;
            if Type::Int != tp_len {
                return Err(TypeError("Array length not int"));
            }
            let tau_id = c.lookup_t(&type_id).ok_or(TypeError("Type not found"))?;
            let tau = gti.lookup_c(tau_id).ok_or(TypeError("Type not found"))?;
            let element_type_id = match tau {
                Type::Array(i) => i,
                _ => return Err(TypeError("Type not array")),
            };
            let element_type = gti
                .lookup_c(element_type_id)
                .ok_or(TypeError("Type not found"))?;
            if tp_init != element_type {
                return Err(TypeError("Array type doesn't match exp"));
            }
            Ok((
                Type::Array(element_type_id),
                CheckedExp::ArrayCreate {
                    length: Box::new(len_exp),
                    initial_value: Box::new(init_exp),
                },
            ))
        }
        Exp::RecordCreate { type_id, fields } => {
            let tau_id = c.lookup_t(&type_id).ok_or(TypeError("Type not found"))?;
            let tau = gti.lookup_c(tau_id).ok_or(TypeError("Type not found"))?;
            let field_type_ids = match tau {
                Type::Record(fields) => fields,
                _ => return Err(TypeError("Not a record type")),
            };
            let cfeilds = fields
                .into_iter()
                .zip(field_type_ids.clone().into_iter())
                .map(|((id, exp), (field_name, type_id))| {
                    let (tp_elem, cexp) = type_check_exp(gti, c, brk, exp)?;
                    let tp = gti.lookup_c(type_id).ok_or(TypeError("Type not found"))?;
                    if tp != tp_elem {
                        return Err(TypeError("Array type doesn't match exp"));
                    }
                    if id != field_name {
                        return Err(TypeError("Unknown Field name "));
                    }
                    Ok((field_name, cexp))
                })
                .collect::<Result<VecDeque<(String, CheckedExp)>, TypeError>>()?;
            Ok((
                Type::Record(field_type_ids),
                CheckedExp::RecordCreate { fields: cfeilds },
            ))
        }
        Exp::Assign { left, right } => {
            let (type_left, cleft) = type_check_lvalue(gti, c, brk, left)?;
            let (type_right, cright) = type_check_exp(gti, c, false, *right)?;

            if type_left != type_right {
                return Err(TypeError("Types on either side of = don't match"));
            }

            Ok((
                Type::Void,
                CheckedExp::Assign {
                    left: cleft,
                    right: Box::new(cright),
                },
            ))
        }
        Exp::IfThenElse {
            if_exp,
            then_exp,
            else_exp,
        } => {
            let (tp_if, if_cexp) = type_check_exp(gti, c, false, *if_exp)?;
            let (tp_then, then_cexp) = type_check_exp(gti, c, brk, *then_exp)?;
            let (tp_else, else_cexp) = type_check_exp(gti, c, brk, *else_exp)?;
            match tp_if {
                Type::Int => (),
                _ => return Err(TypeError("Cond not int if then else")),
            }
            if tp_then != tp_else {
                return Err(TypeError("then and else branch don't match"));
            }
            Ok((
                tp_then,
                CheckedExp::IfThenElse {
                    if_exp: Box::new(if_cexp),
                    then_exp: Box::new(then_cexp),
                    else_exp: Box::new(else_cexp),
                },
            ))
        }
        Exp::IfThen { if_exp, then_exp } => {
            let (tp_if, if_cexp) = type_check_exp(gti, c, false, *if_exp)?;
            let (tp_then, then_cexp) = type_check_exp(gti, c, brk, *then_exp)?;
            match (tp_if, tp_then) {
                (Type::Int, Type::Void) => (),
                _ => return Err(TypeError("Cond not int if_then")),
            }

            Ok((
                Type::Void,
                CheckedExp::IfThen {
                    if_exp: Box::new(if_cexp),
                    then_exp: Box::new(then_cexp),
                },
            ))
        }
        Exp::While { while_exp, do_exp } => {
            let (tp_while, while_cexp) = type_check_exp(gti, c, false, *while_exp)?;
            let (tp_do, do_cexp) = type_check_exp(gti, c, true, *do_exp)?;
            match (tp_while, tp_do) {
                (Type::Int, Type::Void) => (),
                _ => return Err(TypeError("Cond not int while")),
            }

            Ok((
                Type::Void,
                CheckedExp::While {
                    while_exp: Box::new(while_cexp),
                    do_exp: Box::new(do_cexp),
                },
            ))
        }
        Exp::For {
            id,
            for_exp,
            to_exp,
            do_exp,
        } => {
            let (tp_for, for_cexp) = type_check_exp(gti, c, false, *for_exp)?;
            let (tp_to, to_cexp) = type_check_exp(gti, c, false, *to_exp)?;

            let i = gti.gen_sym.new_symbol();
            let int = gti.new_type_id(Type::Int);
            gti.insert_gamma(i, int)?;
            let scope = c.clone().stack(
                HashMap::new(),
                vec![(id, i)].into_iter().collect(),
                HashMap::new(),
            );

            let (tp_do, do_cexp) = type_check_exp(gti, &scope, true, *do_exp)?;

            match (tp_do, tp_for, tp_to) {
                (Type::Void, Type::Int, Type::Int) => (),
                _ => return Err(TypeError("For loop incorrect")),
            }

            Ok((
                Type::Void,
                CheckedExp::For {
                    id: i,
                    for_exp: Box::new(for_cexp),
                    to_exp: Box::new(to_cexp),
                    do_exp: Box::new(do_cexp),
                },
            ))
        }
        Exp::Let { let_exp, in_exp } => {
            let mut sc = c.clone();
            let mut decs = VecDeque::new();
            for dec in let_exp.into_iter() {
                match dec {
                    Dec::VarDec {
                        name,
                        value_type,
                        value,
                    } => {
                        let (sc_, dec) = sc.declare_variable(gti, name, value_type, value)?;
                        sc = sc_;
                        decs.push_back(dec);
                    }
                }
            }

            let (tp_in_exp, in_cexp) = type_check_exp(gti, &sc, false, *in_exp)?;

            Ok((
                tp_in_exp,
                CheckedExp::Let {
                    let_exp: decs,
                    in_exp: Box::new(in_cexp),
                },
            ))
        }
        Exp::Call {
            function_name,
            args,
        } => {
            let function_sym = match c.lookup_f(&function_name) {
                Some(function_sym) => function_sym,
                None => return Err(TypeError("Unknown Function being called")),
            };

            let FunctionType {
                return_type,
                arguments,
            } = match gti.lookup_f(&function_sym) {
                Some(function_type) => function_type,
                None => return Err(TypeError("internal Error Unknown function")),
            };

            let return_type_full = match gti.lookup_c(return_type) {
                Some(return_type_full) => return_type_full,
                None => return Err(TypeError("Unknown return type")),
            };

            let mut cargs = VecDeque::new();

            for ((_, type_id), exp) in arguments.into_iter().zip(args.into_iter()) {
                let expected_type = match gti.lookup_c(type_id) {
                    Some(s) => (s),
                    None => return Err(TypeError("Unknown arg type")),
                };
                let (actual_type, cexp) = type_check_exp(gti, c, false, exp)?;
                if expected_type != actual_type {
                    return Err(TypeError("argument doesn't have expected type"));
                }
                cargs.push_back(cexp);
            }

            Ok((
                return_type_full,
                CheckedExp::Call {
                    function_name: function_sym,
                    args: cargs,
                },
            ))
        }
    }
}

fn type_check_lvalue(
    gti: &mut GlobalTypeInfo,
    c: &StackedContext,
    brk: B,
    lvalue: LValue,
) -> Result<(Type, CheckedLValue), TypeError> {
    match lvalue {
        LValue::Id { name } => {
            let name_s = c
                .lookup_s(&name)
                .ok_or(TypeError("No Such name in LValue"))?;
            let type_id = gti
                .lookup_gamma(name_s)
                .ok_or(TypeError("No Such type id"))?;
            let tau = gti.lookup_c(type_id).ok_or(TypeError("No Such type"))?;
            Ok((tau, CheckedLValue::Id { name: name_s }))
        }
        LValue::Subscript { array, index } => {
            let (tau_array, carray) = type_check_lvalue(gti, c, brk, *array)?;
            let (tau_index, cindex) = type_check_exp(gti, c, false, *index)?;

            let element_type_id = match (tau_array, tau_index) {
                (Type::Array(element_type), Type::Int) => element_type,
                _ => return Err(TypeError("Array index error")),
            };

            let element_type = match gti.lookup_c(element_type_id) {
                Some(element_type) => element_type,
                None => return Err(TypeError("Unknown element Type")),
            };

            Ok((
                element_type,
                CheckedLValue::Subscript {
                    array: Box::new(carray),
                    index: Box::new(cindex),
                },
            ))
        }
        LValue::FieldExp { record, field } => {
            let (tau_rec, crec) = type_check_lvalue(gti, c, brk, *record)?;
            let fields = match tau_rec {
                Type::Record(fields) => fields,
                _ => return Err(TypeError(".field is into non-record")),
            };

            let mut type_id = None;
            for (f, fty) in fields {
                if f == field {
                    type_id = Some(fty);
                    break;
                }
            }
            let type_id = type_id.ok_or(TypeError("No such field in .field"))?;
            let exp_type = gti
                .lookup_c(type_id)
                .ok_or(TypeError("Field Type Not found"))?;

            Ok((
                exp_type,
                CheckedLValue::FieldExp {
                    record: Box::new(crec),
                    field,
                },
            ))
        }
    }
}

fn pairwise_diffrent(feild_decs: &VecDeque<(String, String)>) -> Result<(), TypeError> {
    let mut i = 0;

    let mut pairwise_diffrent = true;

    while i < feild_decs.len() {
        let mut j = i + 1;
        while j < feild_decs.len() {
            pairwise_diffrent = pairwise_diffrent && (feild_decs[i].0 != feild_decs[j].0);
            j += 1;
        }
        i += 1;
    }

    if !pairwise_diffrent {
        return Err(TypeError("Record Field Names in Types Not distinct"));
    }

    Ok(())
}
