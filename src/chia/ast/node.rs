use crate::common::{reserved::ReservedToken, token::Token};
use std::vec::Vec;

pub struct ProgramInfo<'a, 'b> {
    name: String,
    var_fn_defs: Vec<Box<ASTNode<'a, 'b>>>,
}

impl<'a, 'b> ProgramInfo<'a, 'b> {
    pub fn new(name: String, var_fn_defs: Vec<Box<ASTNode<'a, 'b>>>) -> ProgramInfo<'a, 'b> {
        ProgramInfo { name, var_fn_defs }
    }
}

pub struct TypeDef<'a, 'b> {
    type_name: &'a Token<'b>,
    definition: Box<ASTNode<'a, 'b>>,
}

impl<'a, 'b> TypeDef<'a, 'b> {
    pub fn new(type_name: &'a Token<'b>, definition: Box<ASTNode<'a, 'b>>) -> TypeDef<'a, 'b> {
        TypeDef {
            type_name: type_name,
            definition: definition,
        }
    }
}

pub struct StructDef<'a, 'b> {
    identifier: Box<ASTNode<'a, 'b>>,
    fields: Vec<TypeVarPair<'a, 'b>>,
}

impl<'a, 'b> StructDef<'a, 'b> {
    pub fn new(
        identifier: Box<ASTNode<'a, 'b>>,
        fields: Vec<TypeVarPair<'a, 'b>>,
    ) -> StructDef<'a, 'b> {
        StructDef { identifier, fields }
    }
}

pub struct EnumDef<'a, 'b> {
    identifier: Box<ASTNode<'a, 'b>>,
    fields: Vec<ASTNode<'a, 'b>>,
}

impl<'a, 'b> EnumDef<'a, 'b> {
    pub fn new(identifier: Box<ASTNode<'a, 'b>>, fields: Vec<ASTNode<'a, 'b>>) -> EnumDef<'a, 'b> {
        EnumDef { identifier, fields }
    }
}

pub struct TypeInfo<'a, 'b> {
    is_static: bool,
    is_mut: bool,
    is_volatile: bool,
    is_pointer: bool,
    base_type: Box<ASTNode<'a, 'b>>,
}

impl<'a, 'b> TypeInfo<'a, 'b> {
    pub fn new(
        is_static: bool,
        is_mut: bool,
        is_volatile: bool,
        is_pointer: bool,
        base_type: Box<ASTNode<'a, 'b>>,
    ) -> TypeInfo<'a, 'b> {
        TypeInfo {
            is_static,
            is_mut,
            is_volatile,
            is_pointer,
            base_type,
        }
    }
}

pub struct TypeVarPair<'a, 'b> {
    type_of_var: Box<ASTNode<'a, 'b>>,
    identifier: Box<ASTNode<'a, 'b>>,
}

impl<'a, 'b> TypeVarPair<'a, 'b> {
    pub fn new(
        type_of_var: Box<ASTNode<'a, 'b>>,
        identifier: Box<ASTNode<'a, 'b>>,
    ) -> TypeVarPair<'a, 'b> {
        TypeVarPair {
            type_of_var,
            identifier,
        }
    }
}

pub struct FnDef<'a, 'b> {
    return_type: Box<ASTNode<'a, 'b>>,
    identifier: Box<ASTNode<'a, 'b>>,
    arguments: Vec<TypeVarPair<'a, 'b>>,
    body: Option<Box<ASTNode<'a, 'b>>>,
}

impl<'a, 'b> FnDef<'a, 'b> {
    pub fn new(
        return_type: Box<ASTNode<'a, 'b>>,
        identifier: Box<ASTNode<'a, 'b>>,
        arguments: Vec<TypeVarPair<'a, 'b>>,
        body: Option<Box<ASTNode<'a, 'b>>>,
    ) -> FnDef<'a, 'b> {
        FnDef {
            return_type,
            identifier,
            arguments,
            body: body,
        }
    }
}

pub struct FnCall<'a, 'b> {
    fn_identifier: Box<ASTNode<'a, 'b>>,
    arguments: Vec<Box<ASTNode<'a, 'b>>>,
}

impl<'a, 'b> FnCall<'a, 'b> {
    pub fn new(
        fn_identifier: Box<ASTNode<'a, 'b>>,
        arguments: Vec<Box<ASTNode<'a, 'b>>>,
    ) -> FnCall<'a, 'b> {
        FnCall {
            fn_identifier,
            arguments,
        }
    }
}

pub struct VarDef<'a, 'b> {
    variable: TypeVarPair<'a, 'b>,
    value: Option<Box<ASTNode<'a, 'b>>>,
}

impl<'a, 'b> VarDef<'a, 'b> {
    pub fn new(
        variable: TypeVarPair<'a, 'b>,
        value: Option<Box<ASTNode<'a, 'b>>>,
    ) -> VarDef<'a, 'b> {
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

pub struct ControlFlowInfo<'a, 'b> {
    control_type: ControlFlowType,
    condition: Box<ASTNode<'a, 'b>>,
    sequence: Box<ASTNode<'a, 'b>>,
    next_flow: Option<Box<ASTNode<'a, 'b>>>,
}

impl<'a, 'b> ControlFlowInfo<'a, 'b> {
    pub fn new(
        control_type: ControlFlowType,
        condition: Box<ASTNode<'a, 'b>>,
        sequence: Box<ASTNode<'a, 'b>>,
        next_flow: Option<Box<ASTNode<'a, 'b>>>,
    ) -> ControlFlowInfo<'a, 'b> {
        ControlFlowInfo {
            control_type,
            condition,
            sequence,
            next_flow,
        }
    }
}

pub enum ASTNode<'a, 'b> {
    Program(ProgramInfo<'a, 'b>),
    TypeDef(TypeDef<'a, 'b>),
    StructDef(StructDef<'a, 'b>),
    EnumDef(EnumDef<'a, 'b>),
    Type(TypeInfo<'a, 'b>),
    Tuple(Vec<Box<ASTNode<'a, 'b>>>),
    Number(&'a Token<'a>),
    String(&'a Token<'a>),
    Identifier(&'a Token<'a>),
    Expression(Box<ASTNode<'a, 'b>>),
    Function(FnDef<'a, 'b>),
    FunctionCall(FnCall<'a, 'b>),
    Variable(VarDef<'a, 'b>),
    Sequence(Vec<Box<ASTNode<'a, 'b>>>),
    ControlFlow(ControlFlowInfo<'a, 'b>),
    PrefixOperation(&'a ReservedToken<'b>, Box<ASTNode<'a, 'b>>),
    PostfixOperation(&'a ReservedToken<'b>, Box<ASTNode<'a, 'b>>),
    BinaryOperation(
        &'a ReservedToken<'b>,
        Box<ASTNode<'a, 'b>>,
        Box<ASTNode<'a, 'b>>,
    ),
}

impl<'a, 'b> ASTNode<'a, 'b> {
    pub fn new_type_def(type_def: TypeDef<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::TypeDef(type_def)
    }

    pub fn new_type(type_info: TypeInfo<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::Type(type_info)
    }

    pub fn new_number(token: &'a Token<'a>) -> ASTNode<'a, 'b> {
        Self::Number(token)
    }

    pub fn new_identifier(token: &'a Token<'a>) -> ASTNode<'a, 'b> {
        Self::Identifier(token)
    }

    pub fn new_expression(child: Box<ASTNode<'a, 'b>>) -> ASTNode<'a, 'b> {
        Self::Expression(child)
    }

    pub fn new_function(def: FnDef<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::Function(def)
    }

    pub fn new_function_call(call: FnCall<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::FunctionCall(call)
    }

    pub fn new_variable(def: VarDef<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::Variable(def)
    }

    pub fn new_sequence(children: Vec<Box<ASTNode<'a, 'b>>>) -> ASTNode<'a, 'b> {
        Self::Sequence(children)
    }

    pub fn new_control_flow(info: ControlFlowInfo<'a, 'b>) -> ASTNode<'a, 'b> {
        Self::ControlFlow(info)
    }
}
