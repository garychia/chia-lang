use crate::common::token::Token;
use std::vec::Vec;

pub struct ProgramInfo<'a> {
    name: String,
    var_fn_defs: Vec<Box<ASTNode<'a>>>,
}

impl<'a> ProgramInfo<'a> {
    pub fn new(name: String, var_fn_defs: Vec<Box<ASTNode<'a>>>) -> ProgramInfo<'a> {
        ProgramInfo { name, var_fn_defs }
    }
}

pub struct TypeDef<'a> {
    type_name: &'a Token<'a>,
    definition: Box<ASTNode<'a>>,
}

impl<'a> TypeDef<'a> {
    pub fn new(type_name: &'a Token<'a>, definition: Box<ASTNode<'a>>) -> TypeDef<'a> {
        TypeDef {
            type_name: type_name,
            definition: definition,
        }
    }
}

pub struct StructDef<'a> {
    identifier: Box<ASTNode<'a>>,
    fields: Vec<TypeVarPair<'a>>,
}

impl<'a> StructDef<'a> {
    pub fn new(identifier: Box<ASTNode<'a>>, fields: Vec<TypeVarPair<'a>>) -> StructDef<'a> {
        StructDef { identifier, fields }
    }
}

pub struct EnumDef<'a> {
    identifier: Box<ASTNode<'a>>,
    fields: Vec<ASTNode<'a>>,
}

impl<'a> EnumDef<'a> {
    pub fn new(identifier: Box<ASTNode<'a>>, fields: Vec<ASTNode<'a>>) -> EnumDef<'a> {
        EnumDef { identifier, fields }
    }
}

pub struct TypeInfo<'a> {
    is_static: bool,
    is_mut: bool,
    is_volatile: bool,
    is_pointer: bool,
    base_type: Box<ASTNode<'a>>,
}

impl<'a> TypeInfo<'a> {
    pub fn new(
        is_static: bool,
        is_mut: bool,
        is_volatile: bool,
        is_pointer: bool,
        base_type: Box<ASTNode<'a>>,
    ) -> TypeInfo<'a> {
        TypeInfo {
            is_static,
            is_mut,
            is_volatile,
            is_pointer,
            base_type,
        }
    }
}

pub struct TypeVarPair<'a> {
    type_of_var: Box<ASTNode<'a>>,
    identifier: Box<ASTNode<'a>>,
}

impl<'a> TypeVarPair<'a> {
    pub fn new(type_of_var: Box<ASTNode<'a>>, identifier: Box<ASTNode<'a>>) -> TypeVarPair<'a> {
        TypeVarPair {
            type_of_var,
            identifier,
        }
    }
}

pub struct FnDef<'a> {
    return_type: Box<ASTNode<'a>>,
    identifier: Box<ASTNode<'a>>,
    arguments: Vec<TypeVarPair<'a>>,
    body: Option<Box<ASTNode<'a>>>,
}

impl<'a> FnDef<'a> {
    pub fn new(
        return_type: Box<ASTNode<'a>>,
        identifier: Box<ASTNode<'a>>,
        arguments: Vec<TypeVarPair<'a>>,
        body: Option<Box<ASTNode<'a>>>,
    ) -> FnDef<'a> {
        FnDef {
            return_type,
            identifier,
            arguments,
            body: body,
        }
    }
}

pub struct FnCall<'a> {
    fn_identifier: Box<ASTNode<'a>>,
    arguments: Vec<Box<ASTNode<'a>>>,
}

impl<'a> FnCall<'a> {
    pub fn new(fn_identifier: Box<ASTNode<'a>>, arguments: Vec<Box<ASTNode<'a>>>) -> FnCall<'a> {
        FnCall {
            fn_identifier,
            arguments,
        }
    }
}

pub struct VarDef<'a> {
    variable: TypeVarPair<'a>,
    value: Option<Box<ASTNode<'a>>>,
}

impl<'a> VarDef<'a> {
    pub fn new(variable: TypeVarPair<'a>, value: Option<Box<ASTNode<'a>>>) -> VarDef<'a> {
        VarDef { variable, value }
    }
}

pub enum ControlFlowType {
    If,
    ElseIf,
    Else,
    While,
    DoWhile,
    Switch,
    SwitchCase,
    SwitchDefault,
}

pub struct ControlFlowInfo<'a> {
    control_type: ControlFlowType,
    condition: Box<ASTNode<'a>>,
    sequence: Box<ASTNode<'a>>,
    next_flow: Option<Box<ASTNode<'a>>>,
}

impl<'a> ControlFlowInfo<'a> {
    pub fn new(
        control_type: ControlFlowType,
        condition: Box<ASTNode<'a>>,
        sequence: Box<ASTNode<'a>>,
        next_flow: Option<Box<ASTNode<'a>>>,
    ) -> ControlFlowInfo<'a> {
        ControlFlowInfo {
            control_type,
            condition,
            sequence,
            next_flow,
        }
    }
}

pub enum ASTNode<'a> {
    Program(ProgramInfo<'a>),
    TypeDef(TypeDef<'a>),
    StructDef(StructDef<'a>),
    EnumDef(EnumDef<'a>),
    Type(TypeInfo<'a>),
    Tuple(Vec<Box<ASTNode<'a>>>),
    Number(&'a Token<'a>),
    Identifier(&'a Token<'a>),
    Expression(Box<ASTNode<'a>>),
    Function(FnDef<'a>),
    FunctionCall(FnCall<'a>),
    Variable(VarDef<'a>),
    Sequence(Vec<Box<ASTNode<'a>>>),
    ControlFlow(ControlFlowInfo<'a>),
}

impl<'a> ASTNode<'a> {
    pub fn new_type_def(type_def: TypeDef<'a>) -> ASTNode<'a> {
        Self::TypeDef(type_def)
    }

    pub fn new_type(type_info: TypeInfo<'a>) -> ASTNode<'a> {
        Self::Type(type_info)
    }

    pub fn new_number(token: &'a Token<'a>) -> ASTNode<'a> {
        Self::Number(token)
    }

    pub fn new_identifier(token: &'a Token<'a>) -> ASTNode<'a> {
        Self::Identifier(token)
    }

    pub fn new_expression(child: Box<ASTNode<'a>>) -> ASTNode<'a> {
        Self::Expression(child)
    }

    pub fn new_function(def: FnDef<'a>) -> ASTNode<'a> {
        Self::Function(def)
    }

    pub fn new_function_call(call: FnCall<'a>) -> ASTNode<'a> {
        Self::FunctionCall(call)
    }

    pub fn new_variable(def: VarDef<'a>) -> ASTNode<'a> {
        Self::Variable(def)
    }

    pub fn new_sequence(children: Vec<Box<ASTNode<'a>>>) -> ASTNode<'a> {
        Self::Sequence(children)
    }

    pub fn new_control_flow(info: ControlFlowInfo<'a>) -> ASTNode<'a> {
        Self::ControlFlow(info)
    }
}
