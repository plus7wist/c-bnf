// Parse Result.
type Pr<'t, T> = std::result::Result<(T, &'t [u8]), (Pe, &'t [u8])>;

// Parse error.
enum Pe {
    Todo,
}

fn pce(code: &[u8]) -> (Pe, &[u8]) {
    (Pe::Todo, code)
}

// external-declaration ::= function-definition | declaration

enum ExternalDeclaration {
    FunctionDefinition(FunctionDefinition),
    Declaration(Declaration),
}

trait Pz {
    type Data;
    fn parse<'t>(&self, code: &'t [u8]) -> Pr<'t, Self::Data>;
}

impl<F, D> Pz for F
where
    F: Fn(&[u8]) -> Pr<'_, D>,
{
    type Data = D;
    fn parse<'t>(&self, code: &'t [u8]) -> Pr<'t, Self::Data> {
        self(code)
    }
}

fn pc_or<PzL, PzR, Enum>(lhs: PzL, rhs: PzR) -> impl Pz<Data = Enum>
where
    PzL: Pz,
    PzR: Pz,
    PzL::Data: Into<Enum>,
    PzR::Data: Into<Enum>,
{
    |code| {
        if let Ok((lhs, code)) = lhs.parse(code) {
            return Ok((lhs.into(), code));
        }
        rhs.parse(code).map(|(rhs, code)| (rhs.into(), code))
    }
}

fn pc_external_declaration(code: &[u8]) -> Pr<ExternalDeclaration> {
    pc_function_definition(code)
        .map(|(f, code)| (ExternalDeclaration::FunctionDefinition(f), code))
        .or(pc_declaration(code).map(|(d, code)| (ExternalDeclaration::Declaration(d), code)))
}

// function-definition
struct FunctionDefinition {}

fn pc_function_definition(code: &[u8]) -> Pr<FunctionDefinition> {
    todo!()
}

// declaration

struct Declaration {}

fn pc_declaration(code: &[u8]) -> Pr<Declaration> {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
